use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct IdDatatypeConstraints;

impl Rule for IdDatatypeConstraints {
    fn id(&self) -> &'static str {
        "html.id.datatype"
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
        let id = attrs
            .iter()
            .find(|a| match ctx.format {
                html_inspector::InputFormat::Html => a.name.eq_ignore_ascii_case("id"),
                html_inspector::InputFormat::Xhtml => a.name == "id",
            })
            .and_then(|a| a.value.as_deref());
        let Some(id) = id else { return };
        let v = id;

        if v.is_empty() || v.chars().any(|c| c.is_ascii_whitespace()) {
            out.push(Message::new(
                "html.id.invalid",
                Severity::Error,
                Category::Html,
                format!("Bad value “{v}” for attribute “id” on element “{name}”."),
                *span,
            ));
        }
    }
}
