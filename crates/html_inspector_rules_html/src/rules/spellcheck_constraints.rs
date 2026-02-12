use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct SpellcheckConstraints;

impl Rule for SpellcheckConstraints {
    fn id(&self) -> &'static str {
        "html.spellcheck.value"
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
        let attr = attrs.iter().find(|a| ctx.name_is(&a.name, "spellcheck"));
        let Some(attr) = attr else { return };

        let value = attr.value.as_deref().unwrap_or("").trim();
        // Per HTML, enumerated: "true" / "false" (and sometimes empty/missing treated as "true").
        let ok = value.is_empty()
            || value.eq_ignore_ascii_case("true")
            || value.eq_ignore_ascii_case("false");
        if !ok {
            out.push(Message::new(
                "html.spellcheck.invalid",
                Severity::Error,
                Category::Html,
                format!("Bad value “{value}” for attribute “spellcheck” on element “{name}”."),
                *span,
            ));
        }
    }
}
