use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

use super::url_attr;

#[derive(Default)]
pub struct IframeConstraints;

impl Rule for IframeConstraints {
    fn id(&self) -> &'static str {
        "html.iframe.constraints"
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
        if !ctx.name_is(name, "iframe") {
            return;
        }

        if ctx.has_attr(attrs, "allowpaymentrequest") {
            out.push(Message::new(
                "html.iframe.allowpaymentrequest.disallowed",
                Severity::Error,
                Category::Html,
                "Attribute “allowpaymentrequest” not allowed on element “iframe” at this point.",
                *span,
            ));
        }

        if ctx.has_attr(attrs, "seamless") {
            out.push(Message::new(
                "html.iframe.seamless.disallowed",
                Severity::Error,
                Category::Html,
                "Attribute “seamless” not allowed on element “iframe” at this point.",
                *span,
            ));
        }

        let src = ctx.attr_value(attrs, "src");
        let Some(src) = src else { return };

        if src.is_empty() {
            out.push(Message::new(
                "html.iframe.src.empty",
                Severity::Error,
                Category::Html,
                "Bad value “” for attribute “src” on element “iframe”.",
                *span,
            ));
            return;
        }

        let _ = url_attr::validate_url_attr_value(
            src,
            "src",
            "iframe",
            "html.iframe.src.invalid",
            *span,
            out,
        );
    }
}
