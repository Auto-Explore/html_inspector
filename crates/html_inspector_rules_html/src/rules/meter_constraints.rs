use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

use super::shared::{attr_value, eq_name};

#[derive(Default)]
pub struct MeterConstraints;

impl Rule for MeterConstraints {
    fn id(&self) -> &'static str {
        "html.meter.constraints"
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
        if !eq_name(ctx, name, "meter") {
            return;
        }

        let Some(value_str) = attr_value(ctx, attrs, "value") else {
            out.push(Message::new(
                "html.meter.missing_value",
                Severity::Error,
                Category::Html,
                "Element “meter” is missing required attribute “value”.",
                *span,
            ));
            return;
        };

        let value_attr = parse_number(value_str);
        let max_attr = attr_value(ctx, attrs, "max").and_then(parse_number);
        let min_attr = attr_value(ctx, attrs, "min").and_then(parse_number);
        let high_attr = attr_value(ctx, attrs, "high").and_then(parse_number);
        let low_attr = attr_value(ctx, attrs, "low").and_then(parse_number);
        let optimum_attr = attr_value(ctx, attrs, "optimum").and_then(parse_number);

        // Defaults per HTML: min=0, max=1 (when absent).
        let max = max_attr.unwrap_or(1.0);
        let min = min_attr.unwrap_or(0.0);
        let value = value_attr.unwrap_or(0.0);

        if max_attr.is_none() && value > 1.0 {
            out.push(Message::new(
                "html.meter.value.le_one_when_max_absent",
                Severity::Error,
                Category::Html,
                "The value of the “value” attribute must be less than or equal to one when the “max” attribute is absent.",
                *span,
            ));
            return;
        }

        if min_attr.is_none() && value < 0.0 {
            out.push(Message::new(
                "html.meter.value.ge_zero_when_min_absent",
                Severity::Error,
                Category::Html,
                "The value of the “value” attribute must be greater than or equal to zero when the “min” attribute is absent.",
                *span,
            ));
            return;
        }

        if value < min {
            out.push(Message::new(
                "html.meter.min_le_value",
                Severity::Error,
                Category::Html,
                "The value of the “min” attribute must be less than or equal to the value of the “value” attribute.",
                *span,
            ));
            return;
        }

        if value > max {
            out.push(Message::new(
                "html.meter.value_le_max",
                Severity::Error,
                Category::Html,
                "The value of the “value” attribute must be less than or equal to the value of the “max” attribute.",
                *span,
            ));
            return;
        }

        if min > max {
            // vnu suite seems to accept multiple possible first errors; keep one stable.
            out.push(Message::new(
                "html.meter.min_le_max",
                Severity::Error,
                Category::Html,
                "The value of the “min” attribute must be less than or equal to the value of the “max” attribute.",
                *span,
            ));
            return;
        }

        if let Some(high) = high_attr {
            if high < min {
                out.push(Message::new(
                    "html.meter.min_le_high",
                    Severity::Error,
                    Category::Html,
                    "The value of the “min” attribute must be less than or equal to the value of the “high” attribute.",
                    *span,
                ));
                return;
            }
        }

        if let Some(low) = low_attr {
            if low < min {
                out.push(Message::new(
                    "html.meter.min_le_low",
                    Severity::Error,
                    Category::Html,
                    "The value of the “min” attribute must be less than or equal to the value of the “low” attribute.",
                    *span,
                ));
                return;
            }
            if low > max {
                out.push(Message::new(
                    "html.meter.low_le_max",
                    Severity::Error,
                    Category::Html,
                    "The value of the “low” attribute must be less than or equal to the value of the “max” attribute.",
                    *span,
                ));
                return;
            }
        }

        if let Some(high) = high_attr {
            if high > max {
                out.push(Message::new(
                    "html.meter.high_le_max",
                    Severity::Error,
                    Category::Html,
                    "The value of the “high” attribute must be less than or equal to the value of the “max” attribute.",
                    *span,
                ));
                return;
            }
        }

        if let Some(opt) = optimum_attr {
            if opt > max {
                out.push(Message::new(
                    "html.meter.optimum_le_max",
                    Severity::Error,
                    Category::Html,
                    "The value of the “optimum” attribute must be less than or equal to the value of the “max” attribute.",
                    *span,
                ));
                return;
            }
            if opt < min {
                out.push(Message::new(
                    "html.meter.min_le_optimum",
                    Severity::Error,
                    Category::Html,
                    "The value of the “min” attribute must be less than or equal to the value of the “optimum” attribute.",
                    *span,
                ));
                return;
            }
        }

        if let Some((low, high)) = low_attr.zip(high_attr) {
            if low > high {
                out.push(Message::new(
                    "html.meter.low_le_high",
                    Severity::Error,
                    Category::Html,
                    "The value of the “low” attribute must be less than or equal to the value of the “high” attribute.",
                    *span,
                ));
            }
        }
    }
}

fn parse_number(s: &str) -> Option<f64> {
    s.trim().parse::<f64>().ok()
}

#[cfg(test)]
mod tests {
    use html_inspector_core::{Config, InputFormat, RuleSet};
    use html_inspector_html::HtmlEventSource;

    use super::MeterConstraints;

    fn validate(html: &str) -> Vec<String> {
        let src = HtmlEventSource::from_str("t", InputFormat::Html, html).unwrap();
        let report = html_inspector_core::validate_events(
            src,
            RuleSet::new().push(MeterConstraints::default()),
            Config::default(),
        )
        .unwrap();
        report.messages.into_iter().map(|m| m.code).collect()
    }

    #[test]
    fn meter_missing_value_is_reported() {
        assert_eq!(
            validate("<meter></meter>"),
            vec!["html.meter.missing_value"]
        );
    }

    #[test]
    fn meter_invalid_value_is_not_treated_as_missing() {
        assert!(validate(r#"<meter value="foo"></meter>"#).is_empty());
    }

    #[test]
    fn meter_negative_value_without_min_is_reported() {
        assert_eq!(
            validate(r#"<meter value="-1"></meter>"#),
            vec!["html.meter.value.ge_zero_when_min_absent"]
        );
    }
}
