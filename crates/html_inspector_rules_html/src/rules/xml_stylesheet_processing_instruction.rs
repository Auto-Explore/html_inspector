use std::collections::BTreeMap;

use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

use super::media_query::is_invalid_media_query_list;

#[derive(Default)]
pub struct XmlStylesheetProcessingInstruction {
    seen_any_element: bool,
    has_xslt_pi: bool,
}

impl Rule for XmlStylesheetProcessingInstruction {
    fn id(&self) -> &'static str {
        "xhtml.xml_stylesheet.processing_instruction"
    }

    fn interest(&self) -> Interest {
        Interest::START_TAG | Interest::PROCESSING_INSTRUCTION
    }

    fn on_event(
        &mut self,
        event: &ParseEvent,
        ctx: &mut ValidationContext,
        out: &mut dyn MessageSink,
    ) {
        match event {
            ParseEvent::StartTag { .. } => {
                self.seen_any_element = true;
            }
            ParseEvent::ProcessingInstruction { target, data, span } => {
                if ctx.format != html_inspector::InputFormat::Xhtml {
                    return;
                }
                if target != "xml-stylesheet" {
                    return;
                }
                if data.contains("&#X") {
                    out.push(Message::new(
                        "xhtml.xml_stylesheet.hex_ncr.uppercase_x",
                        Severity::Error,
                        Category::Html,
                        "In XML documents, a hexadecimal character reference must begin with “&#x” (lowercase “x”), not “&#X” (uppercase “X”).",
                        *span,
                    ));
                }
                if self.seen_any_element {
                    out.push(Message::new(
                        "xhtml.xml_stylesheet.after_element",
                        Severity::Error,
                        Category::Html,
                        "Any “xml-stylesheet” instruction in a document must occur before any elements in the document. Suppressing any further errors for this “xml-stylesheet” instruction.",
                        *span,
                    ));
                    return;
                }

                let attrs = match parse_pseudo_attributes(data) {
                    Ok(a) => a,
                    Err(e) => {
                        out.push(Message::new(
                            "xhtml.xml_stylesheet.pseudo_attribute.syntax",
                            Severity::Error,
                            Category::Html,
                            e,
                            *span,
                        ));
                        return;
                    }
                };

                // Validate allowed pseudo-attributes and duplicates during parsing.
                let href = attrs.get("href").map(String::as_str);
                let type_attr = attrs.get("type").map(String::as_str);
                let title = attrs.get("title").map(String::as_str);
                let charset = attrs.get("charset").map(String::as_str);
                let alternate = attrs.get("alternate").map(String::as_str);
                let media = attrs.get("media").map(String::as_str);

                let Some(href) = href else {
                    out.push(Message::new(
                        "xhtml.xml_stylesheet.missing_href",
                        Severity::Error,
                        Category::Html,
                        "“xml-stylesheet” instruction lacks “href” pseudo-attribute. The “href” pseudo-attribute is required in all “xml-stylesheet” instructions.",
                        *span,
                    ));
                    return;
                };

                if href.contains(' ') {
                    out.push(Message::new(
                        "xhtml.xml_stylesheet.href.bad_url_space",
                        Severity::Error,
                        Category::Html,
                        format!(
                            "Bad value “{href}” for “xml-stylesheet” pseudo-attribute “href”. Bad URL: Illegal character in path segment. Space is not allowed."
                        ),
                        *span,
                    ));
                    return;
                }

                if let Some(alternate) = alternate {
                    if alternate != "yes" && alternate != "no" {
                        out.push(Message::new(
                            "xhtml.xml_stylesheet.alternate.bad_value",
                            Severity::Error,
                            Category::Html,
                            "The value of the “xml-stylesheet” pseudo-attribute “alternate” must be either “yes” or “no”.",
                            *span,
                        ));
                        return;
                    }
                    if alternate == "yes" {
                        let title_ok = title.is_some_and(|t| !t.trim().is_empty());
                        if !title_ok {
                            out.push(Message::new(
                                "xhtml.xml_stylesheet.alternate_yes.requires_title",
                                Severity::Error,
                                Category::Html,
                                "An “xml-stylesheet” instruction with an “alternate” pseudo-attribute whose value is “yes” must also have a “title” pseudo-attribute with a non-empty value.",
                                *span,
                            ));
                            return;
                        }
                    }
                }

                if let Some(charset) = charset {
                    if !charset.eq_ignore_ascii_case("utf-8") {
                        out.push(Message::new(
                            "xhtml.xml_stylesheet.charset.bad_value",
                            Severity::Error,
                            Category::Html,
                            format!(
                                "Bad value “{charset}” for “xml-stylesheet” pseudo-attribute “charset”. Bad encoding name: “{charset}” is not a valid character encoding name."
                            ),
                            *span,
                        ));
                        return;
                    }
                    out.push(Message::new(
                        "xhtml.xml_stylesheet.charset.ignored_warning",
                        Severity::Warning,
                        Category::Html,
                        "Some browsers ignore the value of the “xml-stylesheet” pseudo-attribute “charset”.",
                        *span,
                    ));
                }

                if let Some(type_attr) = type_attr {
                    if !is_valid_mime_type(type_attr) {
                        out.push(Message::new(
                            "xhtml.xml_stylesheet.type.bad_value",
                            Severity::Error,
                            Category::Html,
                            format!(
                                "Bad value “{type_attr}” for “xml-stylesheet” pseudo-attribute “type”. Bad MIME type: Subtype missing."
                            ),
                            *span,
                        ));
                        return;
                    }

                    let type_lc = type_attr.to_ascii_lowercase();
                    let mime_main = type_lc.split_once(';').map_or(type_lc.as_str(), |(t, _)| t);
                    let mime_main = mime_main.trim();

                    if mime_main != "text/css" && mime_main != "text/xsl" {
                        out.push(Message::new(
                            "xhtml.xml_stylesheet.type.unsupported_warning",
                            Severity::Warning,
                            Category::Html,
                            "“text/css” and “text/xsl” are the only MIME types for the “xml-stylesheet” pseudo-attribute “type” that are supported across browsers.",
                            *span,
                        ));
                    }

                    if mime_indicates_xslt(mime_main) {
                        if mime_main != "text/xsl" {
                            out.push(Message::new(
                                "xhtml.xml_stylesheet.type.xslt_supported_mime_warning",
                                Severity::Warning,
                                Category::Html,
                                "For indicating XSLT, “text/xsl” is the only MIME type for the “xml-stylesheet” pseudo-attribute “type” that is supported across browsers.",
                                *span,
                            ));
                        }
                        if self.has_xslt_pi {
                            out.push(Message::new(
                                "xhtml.xml_stylesheet.type.multiple_xslt_warning",
                                Severity::Warning,
                                Category::Html,
                                "Browsers do not support multiple “xml-stylesheet” instructions with a “type” value that indicates XSLT.",
                                *span,
                            ));
                        }
                        self.has_xslt_pi = true;
                    }
                }

                if let Some(media) = media
                    && is_invalid_media_query_list(media)
                {
                    out.push(Message::new(
                        "xhtml.xml_stylesheet.media.bad_value",
                        Severity::Error,
                        Category::Html,
                        format!(
                            "Bad value “{media}” for “xml-stylesheet” pseudo-attribute “media”."
                        ),
                        *span,
                    ));
                }
            }
            _ => {}
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, _out: &mut dyn MessageSink) {
        self.seen_any_element = false;
        self.has_xslt_pi = false;
    }
}

fn missing_pseudo_attribute_value_error(name: &str) -> String {
    format!(
        "Found “xml-stylesheet” pseudo-attribute “{name}” without a value. All pseudo-attributes in “xml-stylesheet” instructions must have values."
    )
}

fn parse_pseudo_attributes(data: &str) -> Result<BTreeMap<String, String>, String> {
    let mut out = BTreeMap::new();
    let bytes = data.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        if i >= bytes.len() {
            break;
        }

        let name_start = i;
        while i < bytes.len() && !bytes[i].is_ascii_whitespace() && bytes[i] != b'=' {
            i += 1;
        }
        let name = &data[name_start..i];
        if name.is_empty() {
            break;
        }

        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        if i >= bytes.len() || bytes[i] != b'=' {
            return Err(missing_pseudo_attribute_value_error(name));
        }
        i += 1;

        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        if i >= bytes.len() {
            return Err(missing_pseudo_attribute_value_error(name));
        }

        let quote = bytes[i];
        if quote != b'"' && quote != b'\'' {
            return Err(missing_pseudo_attribute_value_error(name));
        }
        i += 1;

        let value_start = i;
        while i < bytes.len() && bytes[i] != quote {
            i += 1;
        }
        if i >= bytes.len() {
            return Err(format!(
                "Unterminated quoted value for “xml-stylesheet” pseudo-attribute “{name}”."
            ));
        }
        let value = &data[value_start..i];
        i += 1;
        if value.contains('<') {
            return Err(format!(
                "Found “xml-stylesheet” pseudo-attribute “{name}” with the character “<” in its value. All pseudo-attribute values in “xml-stylesheet” instructions must not contain the character “<”."
            ));
        }

        if !is_allowed_pseudo_attribute(name) {
            return Err(format!(
                "Pseudo-attribute “{name}” not allowed in “xml-stylesheet” instruction."
            ));
        }
        if out.contains_key(name) {
            return Err(format!(
                "Duplicate “xml-stylesheet” pseudo-attribute “{name}”."
            ));
        }
        out.insert(name.to_string(), value.to_string());
    }
    Ok(out)
}

fn is_allowed_pseudo_attribute(name: &str) -> bool {
    matches!(
        name,
        "alternate" | "charset" | "href" | "media" | "title" | "type"
    )
}

fn is_valid_mime_type(value: &str) -> bool {
    value
        .split_once('/')
        .is_some_and(|(t, s)| !t.is_empty() && !s.is_empty())
}

fn mime_indicates_xslt(mime_main: &str) -> bool {
    matches!(
        mime_main,
        "application/xml" | "text/xml" | "application/xslt+xml" | "text/xsl" | "text/xslt"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    use html_inspector::{Attribute, Config, InputFormat, Span};

    #[derive(Default)]
    struct Sink(Vec<Message>);
    impl MessageSink for Sink {
        fn push(&mut self, msg: Message) {
            self.0.push(msg);
        }
    }

    fn pi(target: &str, data: &str) -> ParseEvent {
        ParseEvent::ProcessingInstruction {
            target: target.to_string(),
            data: data.to_string(),
            span: Some(Span::new(0, 0, 1, 1)),
        }
    }

    fn start(name: &str) -> ParseEvent {
        ParseEvent::StartTag {
            name: name.to_string(),
            attrs: Vec::<Attribute>::new(),
            self_closing: false,
            span: Some(Span::new(0, 0, 1, 1)),
        }
    }

    #[test]
    fn parse_pseudo_attributes_rejects_missing_values_unknown_attrs_and_duplicates() {
        let err = parse_pseudo_attributes("href").unwrap_err();
        assert_eq!(
            err,
            "Found “xml-stylesheet” pseudo-attribute “href” without a value. All pseudo-attributes in “xml-stylesheet” instructions must have values."
        );
        assert!(parse_pseudo_attributes("href=unquoted").is_err());
        assert!(parse_pseudo_attributes("bogus=\"x\"").is_err());
        let err = parse_pseudo_attributes("href=\"a\" href=\"b\"").unwrap_err();
        assert_eq!(err, "Duplicate “xml-stylesheet” pseudo-attribute “href”.");
        let err = parse_pseudo_attributes("href=\"a\" href = \"b\"").unwrap_err();
        assert_eq!(err, "Duplicate “xml-stylesheet” pseudo-attribute “href”.");
        assert!(parse_pseudo_attributes("href=\"a\" type='text/css'").is_ok());
        assert!(parse_pseudo_attributes("href=\"a").is_err());
    }

    #[test]
    fn rule_ignores_non_xhtml_and_non_xml_stylesheet_targets() {
        let mut rule = XmlStylesheetProcessingInstruction::default();
        let mut sink = Sink::default();

        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        rule.on_event(&pi("xml-stylesheet", "href=\"a\""), &mut ctx, &mut sink);
        assert!(sink.0.is_empty());

        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        rule.on_event(&pi("not-it", "href=\"a\""), &mut ctx, &mut sink);
        assert!(sink.0.is_empty());
    }

    #[test]
    fn rule_reports_after_element_and_missing_href_and_href_spaces() {
        let mut rule = XmlStylesheetProcessingInstruction::default();
        let mut sink = Sink::default();
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);

        rule.on_event(&start("html"), &mut ctx, &mut sink);
        rule.on_event(&pi("xml-stylesheet", "href=\"a\""), &mut ctx, &mut sink);
        assert!(
            sink.0
                .iter()
                .any(|m| m.code == "xhtml.xml_stylesheet.after_element")
        );

        let mut rule = XmlStylesheetProcessingInstruction::default();
        let mut sink = Sink::default();
        rule.on_event(
            &pi("xml-stylesheet", "type=\"text/css\""),
            &mut ctx,
            &mut sink,
        );
        assert!(
            sink.0
                .iter()
                .any(|m| m.code == "xhtml.xml_stylesheet.missing_href")
        );

        let mut sink = Sink::default();
        rule.on_event(&pi("xml-stylesheet", "href=\"a b\""), &mut ctx, &mut sink);
        assert!(
            sink.0
                .iter()
                .any(|m| m.code == "xhtml.xml_stylesheet.href.bad_url_space")
        );
    }

    #[test]
    fn rule_validates_alternate_charset_and_type() {
        let mut rule = XmlStylesheetProcessingInstruction::default();
        let mut sink = Sink::default();
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);

        rule.on_event(
            &pi("xml-stylesheet", "href=\"a\" alternate=\"maybe\""),
            &mut ctx,
            &mut sink,
        );
        assert!(
            sink.0
                .iter()
                .any(|m| m.code == "xhtml.xml_stylesheet.alternate.bad_value")
        );

        let mut sink = Sink::default();
        rule.on_event(
            &pi("xml-stylesheet", "href=\"a\" alternate=\"yes\""),
            &mut ctx,
            &mut sink,
        );
        assert!(
            sink.0
                .iter()
                .any(|m| m.code == "xhtml.xml_stylesheet.alternate_yes.requires_title")
        );

        let mut sink = Sink::default();
        rule.on_event(
            &pi("xml-stylesheet", "href=\"a\" charset=\"latin1\""),
            &mut ctx,
            &mut sink,
        );
        assert!(
            sink.0
                .iter()
                .any(|m| m.code == "xhtml.xml_stylesheet.charset.bad_value")
        );

        let mut sink = Sink::default();
        rule.on_event(
            &pi("xml-stylesheet", "href=\"a\" charset=\"utf-8\""),
            &mut ctx,
            &mut sink,
        );
        assert!(
            sink.0
                .iter()
                .any(|m| m.code == "xhtml.xml_stylesheet.charset.ignored_warning")
        );

        let mut sink = Sink::default();
        rule.on_event(
            &pi("xml-stylesheet", "href=\"a\" type=\"text\""),
            &mut ctx,
            &mut sink,
        );
        assert!(
            sink.0
                .iter()
                .any(|m| m.code == "xhtml.xml_stylesheet.type.bad_value")
        );

        let mut sink = Sink::default();
        rule.on_event(
            &pi("xml-stylesheet", "href=\"a\" type=\"application/json\""),
            &mut ctx,
            &mut sink,
        );
        assert!(
            sink.0
                .iter()
                .any(|m| m.code == "xhtml.xml_stylesheet.type.unsupported_warning")
        );

        let mut sink = Sink::default();
        rule.on_event(
            &pi(
                "xml-stylesheet",
                "href=\"a\" type=\"text/css; charset=utf-8\"",
            ),
            &mut ctx,
            &mut sink,
        );
        assert!(
            !sink
                .0
                .iter()
                .any(|m| m.code == "xhtml.xml_stylesheet.type.unsupported_warning")
        );

        let mut sink = Sink::default();
        rule.on_event(
            &pi("xml-stylesheet", "href=\"a\" media=\"(min-width: )\""),
            &mut ctx,
            &mut sink,
        );
        assert!(
            sink.0
                .iter()
                .any(|m| m.code == "xhtml.xml_stylesheet.media.bad_value")
        );
    }

    #[test]
    fn parse_pseudo_attributes_rejects_values_containing_lt() {
        let err = parse_pseudo_attributes("href=\"a<\"").unwrap_err();
        assert_eq!(
            err,
            "Found “xml-stylesheet” pseudo-attribute “href” with the character “<” in its value. All pseudo-attribute values in “xml-stylesheet” instructions must not contain the character “<”."
        );
    }

    #[test]
    fn rule_reports_uppercase_x_in_hex_ncr() {
        let mut rule = XmlStylesheetProcessingInstruction::default();
        let mut sink = Sink::default();
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);

        rule.on_event(
            &pi("xml-stylesheet", "href=\"a\" title=\"&#X1F4A9;\""),
            &mut ctx,
            &mut sink,
        );
        assert!(
            sink.0
                .iter()
                .any(|m| m.code == "xhtml.xml_stylesheet.hex_ncr.uppercase_x")
        );
    }

    #[test]
    fn mime_indicates_xslt_recognizes_known_types() {
        assert!(mime_indicates_xslt("application/xml"));
        assert!(mime_indicates_xslt("text/xml"));
        assert!(mime_indicates_xslt("application/xslt+xml"));
        assert!(mime_indicates_xslt("text/xsl"));
        assert!(mime_indicates_xslt("text/xslt"));
        assert!(!mime_indicates_xslt("text/css"));
        assert!(!mime_indicates_xslt("application/json"));
    }

    #[test]
    fn parse_pseudo_attributes_handles_whitespace_only_and_empty_name_prefix() {
        assert!(parse_pseudo_attributes("").unwrap().is_empty());
        assert!(parse_pseudo_attributes("   ").unwrap().is_empty());
        assert!(parse_pseudo_attributes("=x").unwrap().is_empty());
        assert!(parse_pseudo_attributes("href=").is_err());
    }

    #[test]
    fn rule_covers_unmatched_event_branch() {
        let mut rule = XmlStylesheetProcessingInstruction::default();
        let mut sink = Sink::default();
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        rule.on_event(
            &ParseEvent::Text {
                text: "x".to_string(),
                span: None,
            },
            &mut ctx,
            &mut sink,
        );
        assert!(sink.0.is_empty());
    }

    #[test]
    fn is_valid_mime_type_requires_type_and_subtype() {
        assert!(!is_valid_mime_type(""));
        assert!(!is_valid_mime_type("text"));
        assert!(!is_valid_mime_type("text/"));
        assert!(!is_valid_mime_type("/css"));
        assert!(is_valid_mime_type("text/css"));
        assert!(is_valid_mime_type("application/vnd.test+json"));
    }
}
