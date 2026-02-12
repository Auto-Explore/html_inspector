use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct InputWeekConstraints;

impl Rule for InputWeekConstraints {
    fn id(&self) -> &'static str {
        "html.input.week.datatype"
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
        if !t.eq_ignore_ascii_case("week") {
            return;
        }

        for attr_name in ["min", "max", "value"] {
            let Some(v) = ctx.attr_value(attrs, attr_name) else {
                continue;
            };
            if parse_week(v).is_none() {
                out.push(Message::new(
                    "html.input.week.invalid",
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

fn parse_week(v: &str) -> Option<()> {
    let v = v.trim();
    let (y, w) = v.split_once("-W")?;
    let _y = y.parse::<i32>().ok()?;
    let ww = w.parse::<u32>().ok()?;
    (1..=53).contains(&ww).then_some(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use html_inspector_core::{Config, InputFormat};

    struct Sink(Vec<html_inspector_core::Message>);
    impl html_inspector_core::MessageSink for Sink {
        fn push(&mut self, msg: html_inspector_core::Message) {
            self.0.push(msg);
        }
    }

    #[test]
    fn parse_week_accepts_valid_values() {
        assert!(parse_week("2024-W01").is_some());
        assert!(parse_week("2024-W53").is_some());
    }

    #[test]
    fn xhtml_invalid_min_emits_error() {
        let mut rule = InputWeekConstraints::default();
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        let mut sink = Sink(Vec::new());

        rule.on_event(
            &ParseEvent::StartTag {
                name: "input".to_string(),
                attrs: vec![
                    html_inspector_core::Attribute {
                        name: "type".to_string(),
                        value: Some("week".to_string()),
                        span: None,
                    },
                    html_inspector_core::Attribute {
                        name: "min".to_string(),
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

        assert!(sink.0.iter().any(|m| m.code == "html.input.week.invalid"));
    }

    #[test]
    fn html_attribute_name_matching_is_case_insensitive() {
        let mut rule = InputWeekConstraints::default();
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink(Vec::new());

        rule.on_event(
            &ParseEvent::StartTag {
                name: "input".to_string(),
                attrs: vec![
                    html_inspector_core::Attribute {
                        name: "TYPE".to_string(),
                        value: Some("week".to_string()),
                        span: None,
                    },
                    html_inspector_core::Attribute {
                        name: "MIN".to_string(),
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

        assert!(sink.0.iter().any(|m| m.code == "html.input.week.invalid"));
    }

    #[test]
    fn xhtml_attribute_name_matching_is_case_sensitive() {
        let mut rule = InputWeekConstraints::default();
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        let mut sink = Sink(Vec::new());

        rule.on_event(
            &ParseEvent::StartTag {
                name: "input".to_string(),
                attrs: vec![
                    html_inspector_core::Attribute {
                        name: "TYPE".to_string(),
                        value: Some("week".to_string()),
                        span: None,
                    },
                    html_inspector_core::Attribute {
                        name: "MIN".to_string(),
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

        assert!(!sink.0.iter().any(|m| m.code == "html.input.week.invalid"));
    }
}
