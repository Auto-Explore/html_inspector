use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct IframeTextContentModel;

impl Rule for IframeTextContentModel {
    fn id(&self) -> &'static str {
        "html.iframe.text_content_model"
    }

    fn interest(&self) -> Interest {
        Interest::TEXT
    }

    fn on_event(
        &mut self,
        event: &ParseEvent,
        ctx: &mut ValidationContext,
        out: &mut dyn MessageSink,
    ) {
        let ParseEvent::Text { text, span } = event else {
            return;
        };
        if !ctx.current_parent().is_some_and(|p| is(ctx, p, "iframe")) {
            return;
        }
        if text.chars().any(|c| !c.is_whitespace()) {
            out.push(Message::new(
                "html.iframe.text.disallowed",
                Severity::Error,
                Category::Html,
                "Text not allowed in “iframe” in this context.",
                *span,
            ));
        }
    }
}

fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    match ctx.format {
        html_inspector::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector::InputFormat::Xhtml => actual == expected,
    }
}
