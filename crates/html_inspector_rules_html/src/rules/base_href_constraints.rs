use html_inspector_core::{Interest, MessageSink, ParseEvent, Rule, ValidationContext};

use super::url_attr;

#[derive(Default)]
pub struct BaseHrefConstraints;

impl Rule for BaseHrefConstraints {
    fn id(&self) -> &'static str {
        "html.base.href.datatype"
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
        if !ctx.name_is(name, "base") {
            return;
        }

        let _ = url_attr::validate_optional_url_attr(
            ctx,
            attrs,
            "href",
            "base",
            "html.base.href.invalid",
            *span,
            out,
        );
    }
}
