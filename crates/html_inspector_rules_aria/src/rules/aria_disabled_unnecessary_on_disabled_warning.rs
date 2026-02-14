use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct AriaDisabledUnnecessaryOnDisabledWarning;

impl Rule for AriaDisabledUnnecessaryOnDisabledWarning {
    fn id(&self) -> &'static str {
        "aria.aria_disabled.unnecessary_on_disabled"
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
        let ParseEvent::StartTag { attrs, span, .. } = event else {
            return;
        };
        if !ctx.has_attr(attrs, "disabled") {
            return;
        }
        let aria_disabled = ctx.attr_value(attrs, "aria-disabled");
        if aria_disabled.is_some_and(|v| v.trim().eq_ignore_ascii_case("true")) {
            out.push(Message::new(
                "aria.aria_disabled.unnecessary_on_disabled",
                Severity::Warning,
                Category::Aria,
                "Attribute “aria-disabled” is unnecessary for elements that have attribute “disabled”.",
                *span,
            ));
        }
    }
}
