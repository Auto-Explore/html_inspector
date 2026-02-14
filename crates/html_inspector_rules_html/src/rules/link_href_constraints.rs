use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, ValidationContext,
};

use super::url_attr;

#[derive(Default)]
pub struct LinkHrefConstraints;

impl Rule for LinkHrefConstraints {
    fn id(&self) -> &'static str {
        "html.link.href.datatype"
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
        if !ctx.name_is(name, "link") {
            return;
        }

        let href = ctx.attr_value(attrs, "href");
        let Some(href) = href else { return };

        if href.is_empty() {
            // Covered by UrlConstraints too, but keep a dedicated code for link/href tests.
            out.push(Message::new(
                "html.link.href.empty",
                html_inspector::Severity::Error,
                Category::Html,
                "Bad value “” for attribute “href” on element “link”.",
                *span,
            ));
            return;
        }

        let _ = url_attr::validate_url_attr_value(
            href,
            "href",
            "link",
            "html.link.href.invalid",
            *span,
            out,
        );
    }
}
