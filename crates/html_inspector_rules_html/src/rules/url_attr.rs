use html_inspector_core::{Category, Message, MessageSink, Span, ValidationContext};

use super::a_href_constraints::{emit_forbidden_url_code_point, href_issue_severity};

pub(super) fn validate_url_attr_value(
    value: &str,
    attr: &str,
    element: &str,
    invalid_code: &'static str,
    span: Option<Span>,
    out: &mut dyn MessageSink,
) -> bool {
    if emit_forbidden_url_code_point(value, span, out) {
        return true;
    }
    let Some(sev) = href_issue_severity(value) else {
        return false;
    };

    out.push(Message::new(
        invalid_code,
        sev,
        Category::Html,
        format!("Bad value “{value}” for attribute “{attr}” on element “{element}”."),
        span,
    ));
    true
}

pub(super) fn validate_optional_url_attr(
    ctx: &ValidationContext,
    attrs: &[html_inspector_core::Attribute],
    attr: &str,
    element: &str,
    invalid_code: &'static str,
    span: Option<Span>,
    out: &mut dyn MessageSink,
) -> bool {
    let Some(value) = ctx.attr_value(attrs, attr) else {
        return false;
    };
    validate_url_attr_value(value, attr, element, invalid_code, span, out)
}

#[cfg(test)]
mod tests {
    use super::*;

    use html_inspector_core::{Config, InputFormat, Message, Severity};

    #[derive(Default)]
    struct Sink(Vec<Message>);
    impl MessageSink for Sink {
        fn push(&mut self, msg: Message) {
            self.0.push(msg);
        }
    }

    #[test]
    fn validate_url_attr_value_emits_invalid_code_when_href_issue_detected() {
        let mut sink = Sink::default();
        assert!(validate_url_attr_value(
            "http:example.com",
            "href",
            "a",
            "test.invalid",
            None,
            &mut sink,
        ));
        assert!(sink.0.iter().any(|m| m.code == "test.invalid"));
        assert!(sink.0.iter().any(|m| m.severity == Severity::Error));
    }

    #[test]
    fn validate_optional_url_attr_reads_attribute_per_input_format() {
        let html = ValidationContext::new(Config::default(), InputFormat::Html);
        let xhtml = ValidationContext::new(Config::default(), InputFormat::Xhtml);

        let attrs = vec![html_inspector_core::Attribute {
            name: "HREF".to_string(),
            value: Some("http:example.com".to_string()),
            span: None,
        }];

        let mut sink = Sink::default();
        assert!(validate_optional_url_attr(
            &html,
            &attrs,
            "href",
            "a",
            "test.invalid",
            None,
            &mut sink,
        ));
        let mut sink = Sink::default();
        assert!(!validate_optional_url_attr(
            &xhtml,
            &attrs,
            "href",
            "a",
            "test.invalid",
            None,
            &mut sink,
        ));
    }
}
