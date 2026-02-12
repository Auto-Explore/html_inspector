use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, ValidationContext,
};

use super::url_attr;

#[derive(Default)]
pub struct AudioSrcConstraints;

impl Rule for AudioSrcConstraints {
    fn id(&self) -> &'static str {
        "html.audio.src.datatype"
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
        if !ctx.name_is(name, "audio") {
            return;
        }

        let src = ctx.attr_value(attrs, "src");
        let Some(src) = src else { return };

        if src.is_empty() {
            out.push(Message::new(
                "html.audio.src.empty",
                html_inspector_core::Severity::Error,
                Category::Html,
                "Bad value “” for attribute “src” on element “audio”.",
                *span,
            ));
            return;
        }

        let _ = url_attr::validate_url_attr_value(
            src,
            "src",
            "audio",
            "html.audio.src.invalid",
            *span,
            out,
        );
    }
}
