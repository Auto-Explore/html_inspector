use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct ObjElementConstraints;

impl Rule for ObjElementConstraints {
    fn id(&self) -> &'static str {
        "html.obj.constraints"
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
        let ParseEvent::StartTag { name, span, .. } = event else {
            return;
        };
        if !is(ctx, name, "obj") {
            return;
        }

        let parent = ctx.current_parent();
        if parent.is_some_and(|p| is(ctx, p, "p")) {
            out.push(Message::new(
                "html.element.obj.not_allowed_in_p",
                Severity::Error,
                Category::Html,
                "Element “obj” not allowed as child of “p” in this context.",
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
