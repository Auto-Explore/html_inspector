use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, ValidationContext,
};

use super::url_attr;

#[derive(Default)]
pub struct ObjectDataConstraints;

impl Rule for ObjectDataConstraints {
    fn id(&self) -> &'static str {
        "html.object.data.datatype"
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
        if !ctx.name_is(name, "object") {
            return;
        }

        let data = ctx.attr_value(attrs, "data");
        let Some(data) = data else {
            out.push(Message::new(
                "html.object.data.missing",
                html_inspector::Severity::Error,
                Category::Html,
                "Element “object” is missing required attribute “data”.",
                *span,
            ));
            return;
        };

        if data.is_empty() {
            out.push(Message::new(
                "html.object.data.empty",
                html_inspector::Severity::Error,
                Category::Html,
                "Bad value “” for attribute “data” on element “object”.",
                *span,
            ));
            return;
        }

        let _ = url_attr::validate_url_attr_value(
            data,
            "data",
            "object",
            "html.object.data.invalid",
            *span,
            out,
        );
    }
}
