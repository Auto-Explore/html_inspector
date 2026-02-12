use html_inspector_core::{Category, Message, MessageSink, Rule, Severity, ValidationContext};

#[derive(Default)]
pub struct PictureUnclosedEndOfFile;

impl Rule for PictureUnclosedEndOfFile {
    fn id(&self) -> &'static str {
        "html.parse.picture_unclosed_eof"
    }

    fn on_finish(&mut self, ctx: &mut ValidationContext, out: &mut dyn MessageSink) {
        if ctx.format != html_inspector_core::InputFormat::Html {
            return;
        }
        if ctx.has_ancestor("picture") {
            out.push(Message::new(
                "html.parse.eof.open_elements",
                Severity::Error,
                Category::Html,
                "End of file seen and there were open elements.",
                None,
            ));
        }
    }
}
