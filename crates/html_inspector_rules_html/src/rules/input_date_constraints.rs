use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct InputDateConstraints;

impl Rule for InputDateConstraints {
    fn id(&self) -> &'static str {
        "html.input.date.value"
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

        if !t.eq_ignore_ascii_case("date") {
            return;
        }

        for attr_name in ["min", "max", "value"] {
            let Some(value) = ctx.attr_value(attrs, attr_name) else {
                continue;
            };
            if parse_date(value).is_none() {
                out.push(Message::new(
                    "html.input.date.invalid",
                    Severity::Error,
                    Category::Html,
                    format!("Bad value “{value}” for attribute “{attr_name}” on element “input”."),
                    *span,
                ));
                return;
            }
        }
    }
}

fn parse_date(v: &str) -> Option<(i32, u32, u32)> {
    let v = v.trim();
    let mut it = v.split('-');
    let y = it.next()?.parse::<i32>().ok()?;
    let m = it.next()?.parse::<u32>().ok()?;
    let d = it.next()?.parse::<u32>().ok()?;
    if it.next().is_some() {
        return None;
    }
    if m == 0 || m > 12 {
        return None;
    }
    let max_day = match m {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(y) {
                29
            } else {
                28
            }
        }
        _ => return None,
    };
    if d == 0 || d > max_day {
        return None;
    }
    Some((y, m, d))
}

fn is_leap_year(y: i32) -> bool {
    (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    use html_inspector_core::{Config, InputFormat, RuleSet, Span};
    use html_inspector_html::HtmlEventSource;

    #[test]
    fn parse_date_rejects_out_of_range_values_and_accepts_leap_day() {
        assert!(parse_date("2020-02-29").is_some());
        assert!(parse_date("2019-02-29").is_none());
        assert!(parse_date("2020-00-01").is_none());
        assert!(parse_date("2020-13-01").is_none());
        assert!(parse_date("2020-01-00").is_none());
        assert!(parse_date("2020-01-32").is_none());
        assert!(parse_date("2020-01-01-01").is_none());
        assert!(parse_date(" 2020-01-01 ").is_some());
    }

    #[test]
    fn rule_reports_invalid_date_values_on_date_inputs() {
        let src = HtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<input type=\"date\" value=\"2020-02-30\">",
        )
        .unwrap();
        let rules = RuleSet::new().push(InputDateConstraints::default());
        let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
        assert!(report
            .messages
            .iter()
            .any(|m| m.code == "html.input.date.invalid"));

        // Also cover the non-StartTag path by calling the rule directly.
        struct Sink(Vec<html_inspector_core::Message>);
        impl html_inspector_core::MessageSink for Sink {
            fn push(&mut self, msg: html_inspector_core::Message) {
                self.0.push(msg);
            }
        }
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink(Vec::new());
        let mut rule = InputDateConstraints::default();
        rule.on_event(
            &ParseEvent::Text {
                text: "x".to_string(),
                span: Some(Span::new(0, 0, 1, 1)),
            },
            &mut ctx,
            &mut sink,
        );
        assert!(sink.0.is_empty());
    }

    #[test]
    fn html_attribute_name_matching_is_case_insensitive() {
        struct Sink(Vec<html_inspector_core::Message>);
        impl html_inspector_core::MessageSink for Sink {
            fn push(&mut self, msg: html_inspector_core::Message) {
                self.0.push(msg);
            }
        }

        let mut rule = InputDateConstraints::default();
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink(Vec::new());

        rule.on_event(
            &ParseEvent::StartTag {
                name: "input".to_string(),
                attrs: vec![
                    html_inspector_core::Attribute {
                        name: "TYPE".to_string(),
                        value: Some("date".to_string()),
                        span: None,
                    },
                    html_inspector_core::Attribute {
                        name: "VALUE".to_string(),
                        value: Some("2020-02-30".to_string()),
                        span: None,
                    },
                ],
                self_closing: true,
                span: None,
            },
            &mut ctx,
            &mut sink,
        );

        assert!(sink.0.iter().any(|m| m.code == "html.input.date.invalid"));
    }

    #[test]
    fn xhtml_type_and_date_attrs_are_matched_case_sensitively() {
        let src = HtmlEventSource::from_str(
            "t",
            InputFormat::Xhtml,
            "<input type=\"date\" value=\"2020-02-30\"/>",
        )
        .unwrap();
        let report = html_inspector_core::validate_events(
            src,
            RuleSet::new().push(InputDateConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(report
            .messages
            .iter()
            .any(|m| m.code == "html.input.date.invalid"));
    }

    #[test]
    fn xhtml_attribute_name_matching_is_case_sensitive() {
        let src = HtmlEventSource::from_str(
            "t",
            InputFormat::Xhtml,
            "<input TYPE=\"date\" VALUE=\"2020-02-30\"/>",
        )
        .unwrap();
        let report = html_inspector_core::validate_events(
            src,
            RuleSet::new().push(InputDateConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(!report
            .messages
            .iter()
            .any(|m| m.code == "html.input.date.invalid"));
    }
}
