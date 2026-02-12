use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct AriaSelectRoleConstraints;

impl Rule for AriaSelectRoleConstraints {
    fn id(&self) -> &'static str {
        "aria.select.role_constraints"
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
        if !ctx.name_is(name, "select") {
            return;
        }

        let role = ctx
            .attr_value(attrs, "role")
            .and_then(|v| v.split_ascii_whitespace().next());
        let Some(role) = role else { return };

        if role.eq_ignore_ascii_case("button") {
            out.push(Message::new(
                "aria.select.role.button.invalid",
                Severity::Error,
                Category::Aria,
                "Bad value “button” for attribute “role” on element “select”.",
                *span,
            ));
            return;
        }

        if role.eq_ignore_ascii_case("combobox") && !ctx.has_attr(attrs, "aria-expanded") {
            out.push(Message::new(
                "aria.select.role.combobox.missing_aria_expanded",
                Severity::Error,
                Category::Aria,
                "Element “select” is missing required attribute “aria-expanded”.",
                *span,
            ));
        }

        if role.eq_ignore_ascii_case("listbox") {
            let has_multiple = ctx.has_attr(attrs, "multiple");
            let size_is_gt_one = ctx
                .attr_value(attrs, "size")
                .and_then(|s| s.trim().parse::<u32>().ok())
                .is_some_and(|n| n > 1);
            if !has_multiple && !size_is_gt_one {
                out.push(Message::new(
                    "aria.select.role.listbox.disallowed_without_multiple_or_size",
                    Severity::Error,
                    Category::Aria,
                    "The “listbox” role is not allowed for element “select” without a “multiple” attribute and without a “size” attribute whose value is greater than 1.",
                    *span,
                ));
            }
        }
    }
}
