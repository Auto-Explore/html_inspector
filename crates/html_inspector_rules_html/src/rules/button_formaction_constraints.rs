use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, ValidationContext,
};

use super::url_attr;

#[derive(Default)]
pub struct ButtonFormactionConstraints;

impl Rule for ButtonFormactionConstraints {
    fn id(&self) -> &'static str {
        "html.button.formaction.datatype"
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
        if !ctx.name_is(name, "button") {
            return;
        }

        let formaction = ctx.attr_value(attrs, "formaction");
        let Some(formaction) = formaction else { return };

        if formaction.is_empty() {
            out.push(Message::new(
                "html.button.formaction.empty",
                html_inspector::Severity::Error,
                Category::Html,
                "Bad value “” for attribute “formaction” on element “button”.",
                *span,
            ));
            return;
        }

        let _ = url_attr::validate_url_attr_value(
            formaction,
            "formaction",
            "button",
            "html.button.formaction.invalid",
            *span,
            out,
        );
    }
}
