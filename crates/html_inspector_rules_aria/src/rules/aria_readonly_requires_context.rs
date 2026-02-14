use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct AriaReadonlyRequiresContext;

impl Rule for AriaReadonlyRequiresContext {
    fn id(&self) -> &'static str {
        "aria.aria_readonly.requires_role_or_state"
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

        let has_aria_readonly = attrs.iter().any(|a| match ctx.format {
            html_inspector::InputFormat::Html => a.name.eq_ignore_ascii_case("aria-readonly"),
            html_inspector::InputFormat::Xhtml => a.name == "aria-readonly",
        });
        if !has_aria_readonly {
            return;
        }

        let has_context = attrs.iter().any(|a| match ctx.format {
            html_inspector::InputFormat::Html => {
                a.name.eq_ignore_ascii_case("role")
                    || a.name.eq_ignore_ascii_case("aria-checked")
                    || a.name.eq_ignore_ascii_case("aria-expanded")
                    || a.name.eq_ignore_ascii_case("aria-valuenow")
            }
            html_inspector::InputFormat::Xhtml => {
                a.name == "role"
                    || a.name == "aria-checked"
                    || a.name == "aria-expanded"
                    || a.name == "aria-valuenow"
            }
        });

        if !has_context {
            out.push(Message::new(
                "aria.aria_readonly.requires_role_or_state",
                Severity::Error,
                Category::Aria,
                "Element “div” is missing one or more of the following attributes: “aria-checked”, “aria-expanded”, “aria-valuenow”, “role”.",
                *span,
            ));
        }
    }
}
