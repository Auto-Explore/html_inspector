use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

use super::shared::tag_name_for_message;

#[derive(Default)]
pub struct AriaBrWbrConstraints;

impl Rule for AriaBrWbrConstraints {
    fn id(&self) -> &'static str {
        "aria.br_wbr.constraints"
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
        if !(ctx.name_is(name, "br") || ctx.name_is(name, "wbr")) {
            return;
        }

        let el = tag_name_for_message(ctx, name);

        if ctx.has_attr(attrs, "aria-atomic") {
            out.push(Message::new(
                "aria.aria_atomic.disallowed_on_br_wbr",
                Severity::Error,
                Category::Aria,
                format!("Attribute “aria-atomic” not allowed on element “{el}” at this point."),
                *span,
            ));
        }

        if let Some(role) = ctx.attr_value(attrs, "role") {
            let role_trim = role.trim();
            if role_trim.eq_ignore_ascii_case("separator") {
                out.push(Message::new(
                    "aria.role.separator.disallowed_on_br_wbr",
                    Severity::Error,
                    Category::Aria,
                    format!("Bad value “separator” for attribute “role” on element “{el}”."),
                    *span,
                ));
            }
        }
    }
}
