use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct FooterConstraints;

impl Rule for FooterConstraints {
    fn id(&self) -> &'static str {
        "html.footer.constraints"
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

        if is(ctx, name, "footer") && ctx.has_ancestor("footer") {
            out.push(Message::new(
                "html.footer.descendant.footer",
                Severity::Error,
                Category::Html,
                "The element “footer” must not appear as a descendant of the “footer” element.",
                *span,
            ));
            return;
        }

        if is(ctx, name, "header") && ctx.has_ancestor("footer") {
            out.push(Message::new(
                "html.footer.descendant.header",
                Severity::Error,
                Category::Html,
                "The element “header” must not appear as a descendant of the “footer” element.",
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
