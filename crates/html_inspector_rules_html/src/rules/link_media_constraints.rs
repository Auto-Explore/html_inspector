use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

use super::media_query::is_invalid_media_query_list;

#[derive(Default)]
pub struct LinkMediaConstraints;

impl Rule for LinkMediaConstraints {
    fn id(&self) -> &'static str {
        "html.link.media.datatype"
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
        if !is(ctx, name, "link") {
            return;
        }

        let media = attrs
            .iter()
            .find(|a| match ctx.format {
                html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case("media"),
                html_inspector_core::InputFormat::Xhtml => a.name == "media",
            })
            .and_then(|a| a.value.as_deref());
        let Some(media) = media else { return };

        if is_invalid_media_query_list(media) {
            out.push(Message::new(
                "html.link.media.invalid",
                Severity::Error,
                Category::Html,
                format!("Bad value “{media}” for attribute “media” on element “link”."),
                *span,
            ));
        }
    }
}

fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    match ctx.format {
        html_inspector_core::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector_core::InputFormat::Xhtml => actual == expected,
    }
}
