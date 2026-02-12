use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, ValidationContext,
};

use super::url_attr;

#[derive(Default)]
pub struct InputFormactionConstraints;

impl Rule for InputFormactionConstraints {
    fn id(&self) -> &'static str {
        "html.input.formaction.datatype"
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
        if !ctx.name_is(name, "input") {
            return;
        }

        let input_type = ctx.attr_value(attrs, "type").unwrap_or("text");

        if !(input_type.eq_ignore_ascii_case("image") || input_type.eq_ignore_ascii_case("submit"))
        {
            return;
        }

        let formaction = ctx.attr_value(attrs, "formaction");
        let Some(formaction) = formaction else { return };

        if formaction.is_empty() {
            out.push(Message::new(
                "html.input.formaction.empty",
                html_inspector_core::Severity::Error,
                Category::Html,
                "Bad value “” for attribute “formaction” on element “input”.",
                *span,
            ));
            return;
        }

        let _ = url_attr::validate_url_attr_value(
            formaction,
            "formaction",
            "input",
            "html.input.formaction.invalid",
            *span,
            out,
        );
    }
}
