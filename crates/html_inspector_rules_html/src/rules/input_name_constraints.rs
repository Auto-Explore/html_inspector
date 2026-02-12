use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct InputNameConstraints;

impl Rule for InputNameConstraints {
    fn id(&self) -> &'static str {
        "html.input.name.non_empty"
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
        let n = attrs
            .iter()
            .find(|a| match ctx.format {
                html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case("name"),
                html_inspector_core::InputFormat::Xhtml => a.name == "name",
            })
            .and_then(|a| a.value.as_deref());
        let Some(n) = n else { return };
        if n.is_empty() {
            out.push(Message::new(
                "html.input.name.empty",
                Severity::Error,
                Category::Html,
                "Bad value “” for attribute “name” on element “input”.",
                *span,
            ));
            return;
        }

        let is_isindex = match ctx.format {
            html_inspector_core::InputFormat::Html => n.eq_ignore_ascii_case("isindex"),
            html_inspector_core::InputFormat::Xhtml => n == "isindex",
        };
        if is_isindex {
            out.push(Message::new(
                "html.input.name.isindex.disallowed",
                Severity::Error,
                Category::Html,
                "The value “isindex” for the “name” attribute of the “input” element is not allowed.",
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
