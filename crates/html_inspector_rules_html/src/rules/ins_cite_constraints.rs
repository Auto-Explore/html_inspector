use html_inspector::{Interest, MessageSink, ParseEvent, Rule, ValidationContext};

use super::url_attr;

#[derive(Default)]
pub struct InsCiteConstraints;

impl Rule for InsCiteConstraints {
    fn id(&self) -> &'static str {
        "html.ins.cite.datatype"
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
        if !ctx.name_is(name, "ins") {
            return;
        }

        let _ = url_attr::validate_optional_url_attr(
            ctx,
            attrs,
            "cite",
            "ins",
            "html.ins.cite.invalid",
            *span,
            out,
        );
    }
}
