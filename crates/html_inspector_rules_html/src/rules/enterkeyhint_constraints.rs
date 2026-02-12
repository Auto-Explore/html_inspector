use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct EnterkeyhintConstraints;

impl Rule for EnterkeyhintConstraints {
    fn id(&self) -> &'static str {
        "html.enterkeyhint.constraints"
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

        let enterkeyhint = attrs.iter().find(|a| ctx.name_is(&a.name, "enterkeyhint"));
        let Some(attr) = enterkeyhint else { return };

        let value = attr.value.as_deref().unwrap_or("").trim();
        let ok = matches!(
            value,
            "enter" | "done" | "go" | "next" | "previous" | "search" | "send"
        );
        if !ok {
            out.push(Message::new(
                "html.enterkeyhint.value.invalid",
                Severity::Error,
                Category::Html,
                format!("Bad value “{value}” for attribute “enterkeyhint” on element “{name}”."),
                *span,
            ));
        }
    }
}
