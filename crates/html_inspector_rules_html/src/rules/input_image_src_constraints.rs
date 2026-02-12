use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, ValidationContext,
};

use super::url_attr;

#[derive(Default)]
pub struct InputImageSrcConstraints;

impl Rule for InputImageSrcConstraints {
    fn id(&self) -> &'static str {
        "html.input.image.src.datatype"
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
        if !input_type.eq_ignore_ascii_case("image") {
            return;
        }

        let src = ctx.attr_value(attrs, "src");
        let Some(src) = src else { return };

        if src.is_empty() {
            out.push(Message::new(
                "html.input.image.src.empty",
                html_inspector_core::Severity::Error,
                Category::Html,
                "Bad value “” for attribute “src” on element “input”.",
                *span,
            ));
            return;
        }

        let _ = url_attr::validate_url_attr_value(
            src,
            "src",
            "input",
            "html.input.image.src.invalid",
            *span,
            out,
        );
    }
}
