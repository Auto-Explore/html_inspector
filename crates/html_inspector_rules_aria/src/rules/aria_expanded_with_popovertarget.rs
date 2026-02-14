use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct AriaExpandedWithPopoverTarget;

impl Rule for AriaExpandedWithPopoverTarget {
    fn id(&self) -> &'static str {
        "aria.aria_expanded.disallowed_with_popovertarget"
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

        let has_popovertarget = attrs.iter().any(|a| match ctx.format {
            html_inspector::InputFormat::Html => a.name.eq_ignore_ascii_case("popovertarget"),
            html_inspector::InputFormat::Xhtml => a.name == "popovertarget",
        });
        if !has_popovertarget {
            return;
        }

        let has_aria_expanded = attrs.iter().any(|a| match ctx.format {
            html_inspector::InputFormat::Html => a.name.eq_ignore_ascii_case("aria-expanded"),
            html_inspector::InputFormat::Xhtml => a.name == "aria-expanded",
        });
        if has_aria_expanded {
            out.push(Message::new(
                "aria.aria_expanded.disallowed_with_popovertarget",
                Severity::Error,
                Category::Aria,
                "The “aria-expanded” attribute must not be used on any element which has a “popovertarget” attribute.",
                *span,
            ));
        }
    }
}
