use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

use super::foreign_content::{self, Namespace};
use super::shared::normalize_name;

#[derive(Default)]
pub struct SvgXmlnsConstraints;

impl Rule for SvgXmlnsConstraints {
    fn id(&self) -> &'static str {
        "html.svg.xmlns.constraints"
    }

    fn interest(&self) -> Interest {
        Interest::START_TAG
    }

    fn on_event(
        &mut self,
        event: &ParseEvent,
        ctx: &mut ValidationContext,
        out: &mut dyn MessageSink,
    ) {
        let ParseEvent::StartTag {
            name, attrs, span, ..
        } = event
        else {
            return;
        };

        if foreign_content::namespace_for_next_start_tag(ctx, name) != Namespace::Svg {
            return;
        }

        // Default SVG namespace in HTML must be the SVG namespace URI.
        if let Some(xmlns) = attrs
            .iter()
            .find(|a| normalize_name(ctx, &a.name) == "xmlns")
            .and_then(|a| a.value.as_deref())
            && xmlns != "http://www.w3.org/2000/svg"
        {
            out.push(Message::new(
                    "html.svg.xmlns.default.bad_value",
                    Severity::Error,
                    Category::Html,
                    format!(
                        "Bad value “{xmlns}” for the attribute “xmlns” (only “http://www.w3.org/2000/svg” permitted here)."
                    ),
                    *span,
                ));
        }

        for a in attrs {
            let name_norm = normalize_name(ctx, &a.name);
            if let Some(local) = name_norm.strip_prefix("xmlns:") {
                if local == "xlink" {
                    let value = a.value.as_deref().unwrap_or("");
                    if value != "http://www.w3.org/1999/xlink" {
                        out.push(Message::new(
                            "html.svg.xmlns.xlink.bad_value",
                            Severity::Error,
                            Category::Html,
                            format!(
                                "Bad value “{value}” for the attribute “xmlns:link” (only “http://www.w3.org/1999/xlink” permitted here)."
                            ),
                            *span,
                        ));
                    }
                } else {
                    out.push(Message::new(
                        "html.svg.xmlns.prefix.disallowed",
                        Severity::Error,
                        Category::Html,
                        format!("Attribute “xmlns:{local}” not allowed here."),
                        *span,
                    ));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use html_inspector::{Attribute, Config, InputFormat, Message, MessageSink};

    #[derive(Default)]
    struct Sink(Vec<Message>);
    impl MessageSink for Sink {
        fn push(&mut self, msg: Message) {
            self.0.push(msg);
        }
    }

    fn attr(name: &str, value: &str) -> Attribute {
        Attribute {
            name: name.to_string(),
            value: Some(value.to_string()),
            span: None,
        }
    }

    #[test]
    fn html_attribute_name_matching_is_ascii_case_insensitive() {
        let mut rule = SvgXmlnsConstraints::default();
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut out = Sink::default();
        rule.on_event(
            &ParseEvent::StartTag {
                name: "svg".to_string(),
                attrs: vec![
                    attr("XMLNS", "http://example.com/not-svg"),
                    attr("XMLNS:XLINK", "http://example.com/not-xlink"),
                ],
                self_closing: false,
                span: None,
            },
            &mut ctx,
            &mut out,
        );
        assert!(
            out.0
                .iter()
                .any(|m| m.code == "html.svg.xmlns.default.bad_value")
        );
        assert!(
            out.0
                .iter()
                .any(|m| m.code == "html.svg.xmlns.xlink.bad_value")
        );
    }

    #[test]
    fn xhtml_attribute_name_matching_is_case_sensitive() {
        let mut rule = SvgXmlnsConstraints::default();
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        let mut out = Sink::default();
        rule.on_event(
            &ParseEvent::StartTag {
                name: "svg".to_string(),
                attrs: vec![
                    attr("XMLNS", "http://example.com/not-svg"),
                    attr("XMLNS:XLINK", "http://example.com/not-xlink"),
                ],
                self_closing: false,
                span: None,
            },
            &mut ctx,
            &mut out,
        );
        assert!(out.0.is_empty());
    }
}
