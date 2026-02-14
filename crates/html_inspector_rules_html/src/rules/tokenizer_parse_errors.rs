use html_inspector::{Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity};

#[derive(Default)]
pub struct TokenizerParseErrors;

impl Rule for TokenizerParseErrors {
    fn id(&self) -> &'static str {
        "html.parse.tokenizer_errors"
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

        // Avoid double-reporting for parse errors that are converted into dedicated
        // HTML rule messages elsewhere.
        if code == "html.doctype.missing" {
            return;
        }

        out.push(Message::new(
            code.clone(),
            Severity::Error,
            Category::Html,
            message.clone(),
            *span,
        ));
    }
}
