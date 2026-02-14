use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct AriaHaspopupOnDatalistInputWarning;

impl Rule for AriaHaspopupOnDatalistInputWarning {
    fn id(&self) -> &'static str {
        "aria.haspopup.datalist_input"
    }

    fn max_severity(&self) -> Severity {
        Severity::Warning
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
        if !ctx.name_is(name, "input") {
            return;
        }

        if !ctx.has_attr(attrs, "list") {
            return;
        }

        let ty = ctx.attr_value(attrs, "type").unwrap_or("text");
        if !ty.eq_ignore_ascii_case("text") {
            return;
        }

        if ctx.has_attr(attrs, "aria-haspopup") {
            out.push(Message::new(
                "aria.haspopup.datalist_input",
                Severity::Warning,
                Category::Aria,
                "The “aria-haspopup” attribute should not be used on an “input” element that has a “list” attribute and whose type is “text”.",
                *span,
            ));
        }
    }
}
