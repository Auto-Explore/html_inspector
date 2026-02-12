use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct AddressConstraints;

impl Rule for AddressConstraints {
    fn id(&self) -> &'static str {
        "html.address.constraints"
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
        if !is(ctx, name, "address") {
            return;
        }
        if ctx.has_ancestor("address") {
            out.push(Message::new(
                "html.address.descendant.disallowed",
                Severity::Error,
                Category::Html,
                "The element “address” must not appear as a descendant of the “address” element.",
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
