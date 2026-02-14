use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct ProgressConstraints;

impl Rule for ProgressConstraints {
    fn id(&self) -> &'static str {
        "html.progress.constraints"
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
        if !is(ctx, name, "progress") {
            return;
        }

        let max_attr = attr_value(ctx, attrs, "max");
        let value_attr = attr_value(ctx, attrs, "value");

        let max_value = max_attr.and_then(|v| v.trim().parse::<f64>().ok());
        if let Some(max_raw) = max_attr {
            let ok = max_value.is_some_and(|n| n > 0.0);
            if !ok {
                out.push(Message::new(
                    "html.progress.max.positive",
                    Severity::Error,
                    Category::Html,
                    format!("Bad value “{max_raw}” for attribute “max” on element “progress”."),
                    *span,
                ));
            }
        }

        let Some(value_raw) = value_attr else { return };
        let Some(value_num) = value_raw.trim().parse::<f64>().ok() else {
            out.push(Message::new(
                "html.progress.value.invalid",
                Severity::Error,
                Category::Html,
                format!("Bad value “{value_raw}” for attribute “value” on element “progress”."),
                *span,
            ));
            return;
        };
        if !value_num.is_finite() || value_num < 0.0 {
            out.push(Message::new(
                "html.progress.value.non_negative",
                Severity::Error,
                Category::Html,
                format!("Bad value “{value_raw}” for attribute “value” on element “progress”."),
                *span,
            ));
            return;
        }

        if let Some(max_num) = max_value {
            if max_num > 0.0 && value_num > max_num {
                out.push(Message::new(
                    "html.progress.value.exceeds_max",
                    Severity::Error,
                    Category::Html,
                    "The value of the  “value” attribute must be less than or equal to the value of the “max” attribute.",
                    *span,
                ));
            }
        } else if value_num > 1.0 {
            out.push(Message::new(
                "html.progress.value.exceeds_one",
                Severity::Error,
                Category::Html,
                "The value of the  “value” attribute must be less than or equal to one when the “max” attribute is absent.",
                *span,
            ));
        }
    }
}

fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    match ctx.format {
        html_inspector::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector::InputFormat::Xhtml => actual == expected,
    }
}

fn attr_value<'a>(
    ctx: &ValidationContext,
    attrs: &'a [html_inspector::Attribute],
    needle: &str,
) -> Option<&'a str> {
    attrs
        .iter()
        .find(|a| match ctx.format {
            html_inspector::InputFormat::Html => a.name.eq_ignore_ascii_case(needle),
            html_inspector::InputFormat::Xhtml => a.name == needle,
        })
        .and_then(|a| a.value.as_deref())
}
