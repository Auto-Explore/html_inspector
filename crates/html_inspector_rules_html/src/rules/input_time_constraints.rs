use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct InputTimeConstraints;

impl Rule for InputTimeConstraints {
    fn id(&self) -> &'static str {
        "html.input.time.datatype"
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
        if !t.eq_ignore_ascii_case("time") {
            return;
        }

        for attr_name in ["min", "max", "value"] {
            let Some(v) = ctx.attr_value(attrs, attr_name) else {
                continue;
            };
            if parse_time(v).is_none() {
                out.push(Message::new(
                    "html.input.time.invalid",
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

fn parse_time(v: &str) -> Option<()> {
    let v = v.trim();
    let (base, _frac) = v.split_once('.').unwrap_or((v, ""));
    let mut parts = base.split(':');
    let hh = parts.next()?.parse::<u32>().ok()?;
    let mm = parts.next()?.parse::<u32>().ok()?;
    let ss = if let Some(s) = parts.next() {
        s.parse::<u32>().ok()?
    } else {
        0
    };
    if parts.next().is_some() {
        return None;
    }
    if hh > 23 || mm > 59 || ss > 59 {
        return None;
    }
    Some(())
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
    fn parse_time_accepts_valid_values() {
        assert!(parse_time("23:59:59").is_some());
        assert!(parse_time("00:00").is_some());
    }

    #[test]
    fn parse_time_rejects_too_many_parts() {
        assert!(parse_time("10:20:30:40").is_none());
    }

    #[test]
    fn parse_time_rejects_out_of_range_values() {
        assert!(parse_time("24:00").is_none());
        assert!(parse_time("00:60").is_none());
    }

    #[test]
    fn xhtml_invalid_min_emits_error() {
        let mut rule = InputTimeConstraints::default();
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        let mut sink = Sink(Vec::new());

        rule.on_event(
            &ParseEvent::StartTag {
                name: "input".to_string(),
                attrs: vec![
                    html_inspector_core::Attribute {
                        name: "type".to_string(),
                        value: Some("time".to_string()),
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

        assert!(sink.0.iter().any(|m| m.code == "html.input.time.invalid"));
    }

    #[test]
    fn html_attribute_name_matching_is_case_insensitive() {
        let mut rule = InputTimeConstraints::default();
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink(Vec::new());

        rule.on_event(
            &ParseEvent::StartTag {
                name: "input".to_string(),
                attrs: vec![
                    html_inspector_core::Attribute {
                        name: "TYPE".to_string(),
                        value: Some("time".to_string()),
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

        assert!(sink.0.iter().any(|m| m.code == "html.input.time.invalid"));
    }

    #[test]
    fn xhtml_attribute_name_matching_is_case_sensitive() {
        let mut rule = InputTimeConstraints::default();
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        let mut sink = Sink(Vec::new());

        rule.on_event(
            &ParseEvent::StartTag {
                name: "input".to_string(),
                attrs: vec![
                    html_inspector_core::Attribute {
                        name: "TYPE".to_string(),
                        value: Some("time".to_string()),
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

        assert!(!sink.0.iter().any(|m| m.code == "html.input.time.invalid"));
    }
}
