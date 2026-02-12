use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct AriaDisabledOnAWithHrefWarning;

impl Rule for AriaDisabledOnAWithHrefWarning {
    fn id(&self) -> &'static str {
        "aria.a.href.aria_disabled_true"
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
        if !ctx.name_is(name, "a") {
            return;
        }

        if !ctx.has_attr(attrs, "href") {
            return;
        }

        let aria_disabled = ctx.attr_value(attrs, "aria-disabled");
        if aria_disabled.is_some_and(|v| v.trim().eq_ignore_ascii_case("true")) {
            out.push(Message::new(
                "aria.a.href.aria_disabled_true",
                Severity::Warning,
                Category::Aria,
                "An “aria-disabled” attribute whose value is “true” should not be specified on an “a” element that has an “href” attribute.",
                *span,
            ));
        }
    }
}
