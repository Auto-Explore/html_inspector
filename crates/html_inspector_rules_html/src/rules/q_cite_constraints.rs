use html_inspector::{Interest, MessageSink, ParseEvent, Rule, ValidationContext};

use super::url_attr;

#[derive(Default)]
pub struct QCiteConstraints;

impl Rule for QCiteConstraints {
    fn id(&self) -> &'static str {
        "html.q.cite.datatype"
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
        if !ctx.name_is(name, "q") {
            return;
        }

        let _ = url_attr::validate_optional_url_attr(
            ctx,
            attrs,
            "cite",
            "q",
            "html.q.cite.invalid",
            *span,
            out,
        );
    }
}
