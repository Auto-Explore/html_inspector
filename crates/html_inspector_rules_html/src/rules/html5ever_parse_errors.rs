use html_inspector::{Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity};

#[derive(Default)]
pub struct Html5EverParseErrors;

impl Rule for Html5EverParseErrors {
    fn id(&self) -> &'static str {
        "html.parse.html5ever_errors"
    }

    fn interest(&self) -> Interest {
        Interest::PARSE_ERROR
    }

    fn on_event(
        &mut self,
        event: &ParseEvent,
        _ctx: &mut html_inspector::ValidationContext,
        out: &mut dyn MessageSink,
    ) {
        let ParseEvent::ParseError {
            code,
            message,
            span,
        } = event
        else {
            return;
        };
        if code != "html5.parse_error" {
            return;
        }
        out.push(Message::new(
            "html.parse.error",
            Severity::Error,
            Category::Html,
            message.clone(),
            *span,
        ));
    }
}
