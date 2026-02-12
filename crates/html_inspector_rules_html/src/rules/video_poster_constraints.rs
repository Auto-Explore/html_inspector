use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

use super::url_attr;

#[derive(Default)]
pub struct VideoPosterConstraints;

impl Rule for VideoPosterConstraints {
    fn id(&self) -> &'static str {
        "html.video.poster.datatype"
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
        if !ctx.name_is(name, "video") {
            return;
        }

        let poster = ctx.attr_value(attrs, "poster");
        let Some(poster) = poster else { return };

        if poster.is_empty() {
            out.push(Message::new(
                "html.video.poster.empty",
                Severity::Error,
                Category::Html,
                "Bad value “” for attribute “poster” on element “video”.",
                *span,
            ));
            return;
        }

        let _ = url_attr::validate_url_attr_value(
            poster,
            "poster",
            "video",
            "html.video.poster.invalid",
            *span,
            out,
        );
    }
}
