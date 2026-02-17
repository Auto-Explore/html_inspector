use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct InputCheckboxRoleButtonAriaPressed;

impl Rule for InputCheckboxRoleButtonAriaPressed {
    fn id(&self) -> &'static str {
        "html.input.checkbox.role_button.aria_pressed"
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
        if !input_type.eq_ignore_ascii_case("checkbox") {
            return;
        }

        let role = attr_value(ctx, attrs, "role").unwrap_or("");
        if !role.eq_ignore_ascii_case("button") {
            return;
        }

        if !has_attr(ctx, attrs, "aria-pressed") {
            out.push(Message::new(
                "html.input.checkbox.role_button.missing_aria_pressed",
                Severity::Error,
                Category::Html,
                "An “input” element with a “type” attribute whose value is “checkbox” and with a “role” attribute whose value is “button” must have an “aria-pressed” attribute.",
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

fn has_attr(ctx: &ValidationContext, attrs: &[html_inspector::Attribute], needle: &str) -> bool {
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
