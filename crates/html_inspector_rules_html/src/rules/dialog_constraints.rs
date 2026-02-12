use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct DialogConstraints;

impl Rule for DialogConstraints {
    fn id(&self) -> &'static str {
        "html.dialog.constraints"
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

        if !(is(ctx, name, "dt") || is(ctx, name, "dd")) {
            return;
        }

        if is(ctx, ctx.current_parent().unwrap_or(""), "dialog") {
            out.push(Message::new(
                "html.dialog.disallowed_dt_dd",
                Severity::Error,
                Category::Html,
                format!("Element “{name}” not allowed as child of “dialog” in this context."),
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
