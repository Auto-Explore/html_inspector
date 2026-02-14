use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct AriaExpandedWithCommand;

impl Rule for AriaExpandedWithCommand {
    fn id(&self) -> &'static str {
        "aria.aria_expanded.disallowed_with_command"
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

        let has_command = attrs.iter().any(|a| match ctx.format {
            html_inspector::InputFormat::Html => a.name.eq_ignore_ascii_case("command"),
            html_inspector::InputFormat::Xhtml => a.name == "command",
        });
        if !has_command {
            return;
        }

        let has_aria_expanded = attrs.iter().any(|a| match ctx.format {
            html_inspector::InputFormat::Html => a.name.eq_ignore_ascii_case("aria-expanded"),
            html_inspector::InputFormat::Xhtml => a.name == "aria-expanded",
        });
        if has_aria_expanded {
            out.push(Message::new(
                "aria.aria_expanded.disallowed_with_command",
                Severity::Error,
                Category::Aria,
                "The “aria-expanded” attribute must not be used on any element which has a “command” attribute.",
                *span,
            ));
        }
    }
}
