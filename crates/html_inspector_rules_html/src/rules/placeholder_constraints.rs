use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct PlaceholderConstraints;

impl Rule for PlaceholderConstraints {
    fn id(&self) -> &'static str {
        "html.placeholder.no_linebreaks"
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

        let placeholder = attrs
            .iter()
            .find(|a| match ctx.format {
                html_inspector_core::InputFormat::Html => {
                    a.name.eq_ignore_ascii_case("placeholder")
                }
                html_inspector_core::InputFormat::Xhtml => a.name == "placeholder",
            })
            .and_then(|a| a.value.as_deref());
        let Some(placeholder) = placeholder else {
            return;
        };
        if placeholder.contains('\n') || placeholder.contains('\r') {
            out.push(Message::new(
                "html.placeholder.linebreak",
                Severity::Error,
                Category::Html,
                format!(
                    "Bad value “{placeholder}” for attribute “placeholder” on element “input”."
                ),
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
