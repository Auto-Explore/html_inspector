use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct AriaHiddenConstraints;

impl Rule for AriaHiddenConstraints {
    fn id(&self) -> &'static str {
        "aria.hidden.constraints"
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

        if ctx.has_attr(attrs, "aria-hidden") && ctx.name_is(name, "meta") {
            out.push(Message::new(
                "aria.hidden.disallowed_on_meta",
                Severity::Error,
                Category::Aria,
                "Attribute “aria-hidden” not allowed on element “meta” at this point.",
                *span,
            ));
        }

        let aria_hidden_true = ctx
            .attr_value(attrs, "aria-hidden")
            .is_some_and(|v| v.trim().eq_ignore_ascii_case("true"));
        if !aria_hidden_true {
            return;
        }

        if ctx.name_is(name, "body") {
            out.push(Message::new(
                "aria.hidden.disallowed_on_body",
                Severity::Error,
                Category::Aria,
                "“aria-hidden=true” must not be used on the “body” element.",
                *span,
            ));
            return;
        }

        let hidden_is_until_found = ctx
            .attr_value(attrs, "hidden")
            .is_some_and(|v| v.trim().eq_ignore_ascii_case("until-found"));
        if hidden_is_until_found {
            out.push(Message::new(
                "aria.hidden.until_found.conflict",
                Severity::Error,
                Category::Aria,
                "Attribute “aria-hidden” with value “true” must not be specified on elements with “hidden” attribute value “until-found”. This combination prevents content from being accessible to assistive technology when revealed through search.",
                *span,
            ));
        }
    }
}
