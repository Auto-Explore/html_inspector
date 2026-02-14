use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct InputMonthConstraints;

impl Rule for InputMonthConstraints {
    fn id(&self) -> &'static str {
        "html.input.month.datatype"
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
        if !ctx.name_is(name, "input") {
            return;
        }

        let t = ctx.attr_value(attrs, "type").unwrap_or("");
        if !t.eq_ignore_ascii_case("month") {
            return;
        }

        for attr_name in ["min", "max", "value"] {
            let Some(v) = ctx.attr_value(attrs, attr_name) else {
                continue;
            };
            if parse_month(v).is_none() {
                out.push(Message::new(
                    "html.input.month.invalid",
                    Severity::Error,
                    Category::Html,
                    format!("Bad value “{v}” for attribute “{attr_name}” on element “input”."),
                    *span,
                ));
                return;
            }
        }
    }
}

fn parse_month(v: &str) -> Option<()> {
    let v = v.trim();
    let mut it = v.split('-');
    let _y = it.next()?.parse::<i32>().ok()?;
    let m = it.next()?.parse::<u32>().ok()?;
    if it.next().is_some() {
        return None;
    }
    (1..=12).contains(&m).then_some(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use html_inspector::{Config, InputFormat};

    struct Sink(Vec<html_inspector::Message>);
    impl html_inspector::MessageSink for Sink {
        fn push(&mut self, msg: html_inspector::Message) {
            self.0.push(msg);
        }
    }

    #[test]
    fn parse_month_accepts_valid_values() {
        assert!(parse_month("2024-01").is_some());
    }

    #[test]
    fn parse_month_rejects_extra_components() {
        assert!(parse_month("2024-01-02").is_none());
    }

    #[test]
    fn xhtml_invalid_value_emits_error() {
        let mut rule = InputMonthConstraints::default();
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        let mut sink = Sink(Vec::new());

        rule.on_event(
            &ParseEvent::StartTag {
                name: "input".to_string(),
                attrs: vec![
                    html_inspector::Attribute {
                        name: "type".to_string(),
                        value: Some("month".to_string()),
                        span: None,
                    },
                    html_inspector::Attribute {
                        name: "value".to_string(),
                        value: Some("bad".to_string()),
                        span: None,
                    },
                ],
                self_closing: true,
                span: None,
            },
            &mut ctx,
            &mut sink,
        );

        assert!(sink.0.iter().any(|m| m.code == "html.input.month.invalid"));
    }

    #[test]
    fn html_attribute_name_matching_is_case_insensitive() {
        let mut rule = InputMonthConstraints::default();
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink(Vec::new());

        rule.on_event(
            &ParseEvent::StartTag {
                name: "input".to_string(),
                attrs: vec![
                    html_inspector::Attribute {
                        name: "TYPE".to_string(),
                        value: Some("month".to_string()),
                        span: None,
                    },
                    html_inspector::Attribute {
                        name: "VALUE".to_string(),
                        value: Some("bad".to_string()),
                        span: None,
                    },
                ],
                self_closing: true,
                span: None,
            },
            &mut ctx,
            &mut sink,
        );

        assert!(sink.0.iter().any(|m| m.code == "html.input.month.invalid"));
    }

    #[test]
    fn xhtml_attribute_name_matching_is_case_sensitive() {
        let mut rule = InputMonthConstraints::default();
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        let mut sink = Sink(Vec::new());

        rule.on_event(
            &ParseEvent::StartTag {
                name: "input".to_string(),
                attrs: vec![
                    html_inspector::Attribute {
                        name: "TYPE".to_string(),
                        value: Some("month".to_string()),
                        span: None,
                    },
                    html_inspector::Attribute {
                        name: "VALUE".to_string(),
                        value: Some("bad".to_string()),
                        span: None,
                    },
                ],
                self_closing: true,
                span: None,
            },
            &mut ctx,
            &mut sink,
        );

        assert!(!sink.0.iter().any(|m| m.code == "html.input.month.invalid"));
    }
}
