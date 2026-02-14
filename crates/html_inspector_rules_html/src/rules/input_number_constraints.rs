use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct InputNumberConstraints;

impl Rule for InputNumberConstraints {
    fn id(&self) -> &'static str {
        "html.input.number.constraints"
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
        if !is(ctx, name, "input") {
            return;
        }

        let input_type = attr_value(ctx, attrs, "type").unwrap_or("text");
        if !input_type.eq_ignore_ascii_case("number") {
            return;
        }

        if has_attr(ctx, attrs, "multiple") {
            out.push(Message::new(
                "html.input.number.multiple.disallowed",
                Severity::Error,
                Category::Html,
                "Attribute “multiple” not allowed on element “input” at this point.",
                *span,
            ));
        }

        if let Some(value) = attr_value(ctx, attrs, "value")
            && (value.is_empty() || value.parse::<f64>().is_err())
        {
            out.push(Message::new(
                "html.input.number.value.invalid",
                Severity::Error,
                Category::Html,
                format!("Bad value “{value}” for attribute “value” on element “input”."),
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

fn has_attr(
    ctx: &ValidationContext,
    attrs: &[html_inspector::Attribute],
    needle: &str,
) -> bool {
    attrs.iter().any(|a| match ctx.format {
        html_inspector::InputFormat::Html => a.name.eq_ignore_ascii_case(needle),
        html_inspector::InputFormat::Xhtml => a.name == needle,
    })
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
