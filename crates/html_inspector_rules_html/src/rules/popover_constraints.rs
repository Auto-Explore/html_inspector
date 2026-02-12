use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct PopoverConstraints;

impl Rule for PopoverConstraints {
    fn id(&self) -> &'static str {
        "html.popover.constraints"
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

        let popover = attrs.iter().find(|a| match ctx.format {
            html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case("popover"),
            html_inspector_core::InputFormat::Xhtml => a.name == "popover",
        });
        let Some(popover) = popover else { return };

        let value = popover.value.as_deref().unwrap_or("").trim();
        let ok = value.is_empty()
            || value.eq_ignore_ascii_case("auto")
            || value.eq_ignore_ascii_case("manual")
            || value.eq_ignore_ascii_case("hint");
        if !ok {
            out.push(Message::new(
                "html.popover.value.invalid",
                Severity::Error,
                Category::Html,
                format!("Bad value “{value}” for attribute “popover” on element “{name}”."),
                *span,
            ));
        }
    }
}
