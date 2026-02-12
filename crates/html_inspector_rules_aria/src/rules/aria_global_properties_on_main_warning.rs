use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct AriaGlobalPropertiesOnMainWarning;

impl Rule for AriaGlobalPropertiesOnMainWarning {
    fn id(&self) -> &'static str {
        "aria.role.main.global_properties_warning"
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

        let has_main_role = ctx.attr_value(attrs, "role").is_some_and(|v| {
            v.split_ascii_whitespace()
                .any(|t| t.eq_ignore_ascii_case("main"))
        });
        if !has_main_role {
            return;
        }

        if ctx.has_attr(attrs, "aria-disabled") {
            out.push(Message::new(
                "aria.role.main.aria_disabled.discouraged",
                Severity::Warning,
                Category::Aria,
                "The “aria-disabled” attribute should not be used on any element which has “role=main”.",
                *span,
            ));
        }
        if ctx.has_attr(attrs, "aria-haspopup") {
            out.push(Message::new(
                "aria.role.main.aria_haspopup.discouraged",
                Severity::Warning,
                Category::Aria,
                "The “aria-haspopup” attribute should not be used on any element which has “role=main”.",
                *span,
            ));
        }
        if ctx.has_attr(attrs, "aria-invalid") {
            out.push(Message::new(
                "aria.role.main.aria_invalid.discouraged",
                Severity::Warning,
                Category::Aria,
                "The “aria-invalid” attribute should not be used on any element which has “role=main”.",
                *span,
            ));
        }
    }
}
