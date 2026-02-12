use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct DoctypeRequired {
    saw_doctype: bool,
    parser_reported_missing_doctype: bool,
}

impl Rule for DoctypeRequired {
    fn id(&self) -> &'static str {
        "html.doctype.required"
    }

    fn interest(&self) -> Interest {
        Interest::DOCTYPE | Interest::PARSE_ERROR | Interest::FINISH
    }

    fn on_event(
        &mut self,
        event: &ParseEvent,
        _ctx: &mut ValidationContext,
        _out: &mut dyn MessageSink,
    ) {
        match event {
            ParseEvent::Doctype { .. } => {
                self.saw_doctype = true;
            }
            ParseEvent::ParseError { code, message, .. } => {
                // vnu.jar reports missing-doctype conditions as parser errors (e.g. "Start tag
                // seen without seeing a doctype first..."). Avoid emitting an additional
                // end-of-document "missing doctype" error in those cases.
                if matches!(
                    code.as_str(),
                    "html.parser.start_tag_without_doctype" | "html.parser.end_tag_without_doctype"
                ) || message.contains("without seeing a doctype first")
                {
                    self.parser_reported_missing_doctype = true;
                }
            }
            _ => {}
        }
    }

    fn on_finish(&mut self, ctx: &mut ValidationContext, out: &mut dyn MessageSink) {
        if ctx.format == html_inspector_core::InputFormat::Html
            && !self.saw_doctype
            && !self.parser_reported_missing_doctype
        {
            out.push(Message::new(
                "html.doctype.missing",
                Severity::Error,
                Category::Html,
                "Missing document type declaration (doctype).",
                None,
            ));
        }
    }
}
