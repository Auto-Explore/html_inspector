use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

use super::shared::tag_name_for_message;

#[derive(Default)]
pub struct AriaCheckedAllowedRole;

impl Rule for AriaCheckedAllowedRole {
    fn id(&self) -> &'static str {
        "aria.aria_checked.allowed_role"
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

        if !ctx.has_attr(attrs, "aria-checked") {
            return;
        }

        // The HTML/host-language disparity case is handled separately for checkbox inputs.
        if ctx.name_is(name, "input")
            && ctx.attr_value(attrs, "type").is_some_and(|t| {
                t.eq_ignore_ascii_case("checkbox") || t.eq_ignore_ascii_case("radio")
            })
        {
            return;
        }

        let role = ctx
            .attr_value(attrs, "role")
            .and_then(|v| v.split_ascii_whitespace().next());

        if role.is_some_and(is_checked_role) {
            return;
        }

        let el = tag_name_for_message(ctx, name);
        out.push(Message::new(
            "aria.aria_checked.not_allowed",
            Severity::Error,
            Category::Aria,
            format!("Attribute “aria-checked” not allowed on element “{el}” at this point."),
            *span,
        ));
    }
}

fn is_checked_role(role: &str) -> bool {
    role.eq_ignore_ascii_case("checkbox")
        || role.eq_ignore_ascii_case("menuitemcheckbox")
        || role.eq_ignore_ascii_case("menuitemradio")
        || role.eq_ignore_ascii_case("option")
        || role.eq_ignore_ascii_case("radio")
        || role.eq_ignore_ascii_case("switch")
        || role.eq_ignore_ascii_case("treeitem")
}
