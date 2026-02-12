use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct LiParentConstraints;

impl Rule for LiParentConstraints {
    fn id(&self) -> &'static str {
        "html.li.parent_constraints"
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
        if !is(ctx, name, "li") {
            return;
        }

        let parent = ctx.current_parent().unwrap_or("");
        let allowed = is(ctx, parent, "ul") || is(ctx, parent, "ol") || is(ctx, parent, "menu");
        if !allowed {
            out.push(Message::new(
                "html.li.parent.disallowed",
                Severity::Error,
                Category::Html,
                format!("Element “li” not allowed as child of “{parent}” in this context."),
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
