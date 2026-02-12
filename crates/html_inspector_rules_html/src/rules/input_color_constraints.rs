use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct InputColorConstraints;

impl Rule for InputColorConstraints {
    fn id(&self) -> &'static str {
        "html.input.color.value"
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

        let t = attrs
            .iter()
            .find(|a| match ctx.format {
                html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case("type"),
                html_inspector_core::InputFormat::Xhtml => a.name == "type",
            })
            .and_then(|a| a.value.as_deref())
            .unwrap_or("");

        if !t.eq_ignore_ascii_case("color") {
            return;
        }

        let value = attrs
            .iter()
            .find(|a| match ctx.format {
                html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case("value"),
                html_inspector_core::InputFormat::Xhtml => a.name == "value",
            })
            .and_then(|a| a.value.as_deref());

        let Some(value) = value else { return };
        if !is_simple_color(value) {
            out.push(Message::new(
                "html.input.color.value.invalid",
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
        html_inspector_core::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector_core::InputFormat::Xhtml => actual == expected,
    }
}

fn is_simple_color(v: &str) -> bool {
    let v = v.trim();
    let Some(hex) = v.strip_prefix('#') else {
        return false;
    };
    if hex.len() != 6 {
        return false;
    }
    hex.as_bytes().iter().all(|b| b.is_ascii_hexdigit())
}
