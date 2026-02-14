use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct ObjectMimetypeConstraints;

#[derive(Default)]
pub struct LinkMimetypeConstraints;

impl Rule for ObjectMimetypeConstraints {
    fn id(&self) -> &'static str {
        "html.object.type.mimetype"
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
        if !ctx.name_is(name, "object") {
            return;
        }
        let Some(t) = ctx.attr_value(attrs, "type") else {
            return;
        };
        if !is_mime_type(t) {
            out.push(Message::new(
                "html.object.type.invalid",
                Severity::Error,
                Category::Html,
                format!("Bad value “{t}” for attribute “type” on element “object”."),
                *span,
            ));
        }
    }
}

impl Rule for LinkMimetypeConstraints {
    fn id(&self) -> &'static str {
        "html.link.type.mimetype"
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
        if !ctx.name_is(name, "link") {
            return;
        }
        let Some(t) = ctx.attr_value(attrs, "type") else {
            return;
        };
        if !is_mime_type(t) {
            out.push(Message::new(
                "html.link.type.invalid",
                Severity::Error,
                Category::Html,
                format!("Bad value “{t}” for attribute “type” on element “link”."),
                *span,
            ));
        }
    }
}

pub(crate) fn is_mime_type(v: &str) -> bool {
    if v.is_empty() {
        return false;
    }
    let bytes = v.as_bytes();
    if bytes.first().is_some_and(|b| b.is_ascii_whitespace())
        || bytes.last().is_some_and(|b| b.is_ascii_whitespace())
    {
        return false;
    }

    let mut i = 0usize;
    let allowed = |c: u8| {
        c.is_ascii_alphanumeric()
            || matches!(
                c,
                b'!' | b'#' | b'$' | b'&' | b'^' | b'_' | b'.' | b'+' | b'-'
            )
    };

    // type
    let type_start = i;
    while i < bytes.len() && allowed(bytes[i]) {
        i += 1;
    }
    if i == type_start {
        return false;
    }
    if i >= bytes.len() || bytes[i] != b'/' {
        return false;
    }
    i += 1;

    // subtype
    let subtype_start = i;
    while i < bytes.len() && allowed(bytes[i]) {
        i += 1;
    }
    if i == subtype_start {
        return false;
    }

    // parameters: allow OWS around `;` and `=`.
    loop {
        if i == bytes.len() {
            return true;
        }

        // Allow OWS before semicolon, but only if followed by `;`. Anything else is invalid.
        if bytes[i].is_ascii_whitespace() {
            let mut j = i;
            while j < bytes.len() && bytes[j].is_ascii_whitespace() {
                j += 1;
            }
            if j == bytes.len() || bytes[j] != b';' {
                return false;
            }
            i = j;
        }

        if i >= bytes.len() || bytes[i] != b';' {
            return false;
        }

        // consume `;`
        i += 1;
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        if i == bytes.len() {
            return false; // trailing `;` or `;` + OWS
        }

        // parameter name token
        let name_start = i;
        while i < bytes.len() && allowed(bytes[i]) {
            i += 1;
        }
        if i == name_start {
            return false;
        }
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        if i >= bytes.len() || bytes[i] != b'=' {
            return false;
        }
        i += 1;
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        if i == bytes.len() {
            return false;
        }

        // parameter value: token or quoted-string
        if bytes[i] == b'"' {
            i += 1;
            while i < bytes.len() {
                let b = bytes[i];
                if b == b'"' {
                    i += 1;
                    break;
                }
                if b == b'\\' {
                    i += 1;
                    if i == bytes.len() {
                        return false;
                    }
                    // accept any escaped byte
                    i += 1;
                    continue;
                }
                // Disallow bare CR/LF (matches the suite behavior for common parse errors).
                if b == b'\r' || b == b'\n' {
                    return false;
                }
                i += 1;
            }
            // must have consumed a closing quote
            if i > bytes.len() || bytes.get(i - 1) != Some(&b'"') {
                return false;
            }
        } else {
            let val_start = i;
            while i < bytes.len() && allowed(bytes[i]) {
                i += 1;
            }
            if i == val_start {
                return false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use html_inspector::{Config, InputFormat, RuleSet};
    use html_inspector_html::HtmlEventSource;

    #[test]
    fn mime_type_lexing_accepts_common_valid_values() {
        assert!(is_mime_type("text/html"));
        assert!(is_mime_type("application/json; charset=utf-8"));
        assert!(is_mime_type("application/json; charset = \"utf-8\""));
        assert!(is_mime_type("text/plain; a=\"\\\\\""));
    }

    #[test]
    fn mime_type_lexing_rejects_common_invalid_values() {
        assert!(!is_mime_type(""));
        assert!(!is_mime_type(" text/html"));
        assert!(!is_mime_type("text/html "));
        assert!(!is_mime_type("text/html x"));
        assert!(!is_mime_type("text"));
        assert!(!is_mime_type("text/"));
        assert!(!is_mime_type("text/html;"));
        assert!(!is_mime_type("text/html; =x"));
        assert!(!is_mime_type("text/html; a=="));
        assert!(!is_mime_type("text/html; a="));
        assert!(!is_mime_type("text/html; a=\"unterminated"));
        assert!(!is_mime_type("text/html; a=\"bad\\"));
        assert!(!is_mime_type("text/html; a=\"x\ny\""));
    }

    #[test]
    fn rules_emit_errors_for_invalid_type_attribute() {
        let src = HtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<object type=\"not a mime\"></object><link type=\"not a mime\">",
        )
        .unwrap();
        let rules = RuleSet::new()
            .push(ObjectMimetypeConstraints::default())
            .push(LinkMimetypeConstraints::default());
        let report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.object.type.invalid")
        );
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.link.type.invalid")
        );
    }

    #[test]
    fn rules_match_attribute_names_case_insensitively_in_html() {
        let src = HtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<object TYPE=\"not a mime\"></object><link TYPE=\"not a mime\">",
        )
        .unwrap();
        let rules = RuleSet::new()
            .push(ObjectMimetypeConstraints::default())
            .push(LinkMimetypeConstraints::default());
        let report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.object.type.invalid")
        );
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.link.type.invalid")
        );
    }

    #[test]
    fn rules_match_attribute_names_case_sensitively_in_xhtml() {
        let src = HtmlEventSource::from_str(
            "t",
            InputFormat::Xhtml,
            "<object TYPE=\"not a mime\"></object><link TYPE=\"not a mime\"/>",
        )
        .unwrap();
        let rules = RuleSet::new()
            .push(ObjectMimetypeConstraints::default())
            .push(LinkMimetypeConstraints::default());
        let report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
        assert!(
            !report
                .messages
                .iter()
                .any(|m| m.code == "html.object.type.invalid")
        );
        assert!(
            !report
                .messages
                .iter()
                .any(|m| m.code == "html.link.type.invalid")
        );
    }
}
