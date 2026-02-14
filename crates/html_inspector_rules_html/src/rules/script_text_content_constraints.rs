use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct ScriptTextContentConstraints {
    in_script: bool,
    has_src: bool,
    saw_non_whitespace: bool,
    buf: String,
    span: Option<html_inspector::Span>,
}

impl Rule for ScriptTextContentConstraints {
    fn id(&self) -> &'static str {
        "html.script.text_content"
    }

    fn max_severity(&self) -> Severity {
        Severity::Warning
    }

    fn interest(&self) -> Interest {
        Interest::START_TAG | Interest::END_TAG | Interest::TEXT
    }

    fn on_event(
        &mut self,
        event: &ParseEvent,
        ctx: &mut ValidationContext,
        out: &mut dyn MessageSink,
    ) {
        match event {
            ParseEvent::StartTag {
                name, attrs, span, ..
            } => {
                if !ctx.name_is(name, "script") {
                    return;
                }
                self.in_script = true;
                self.has_src = ctx.has_attr(attrs, "src");
                self.saw_non_whitespace = false;
                self.buf.clear();
                self.span = *span;
            }
            ParseEvent::Text { text, .. } => {
                if !self.in_script || !self.has_src {
                    return;
                }
                if text.chars().any(|c| !c.is_whitespace()) {
                    self.saw_non_whitespace = true;
                }
                self.buf.push_str(text);
            }
            ParseEvent::EndTag { name, .. } => {
                if !ctx.name_is(name, "script") || !self.in_script {
                    return;
                }
                self.in_script = false;
                if self.has_src && self.saw_non_whitespace {
                    // Best-effort: warn if non-whitespace present in script w/ src.
                    out.push(Message::new(
                        "html.script.text_content.non_empty",
                        Severity::Warning,
                        Category::Html,
                        "The text content of element “script” should be empty when the “src” attribute is present.",
                        self.span,
                    ));
                }
                self.reset();
            }
            _ => {}
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, _out: &mut dyn MessageSink) {
        self.reset();
    }
}

impl ScriptTextContentConstraints {
    fn reset(&mut self) {
        self.in_script = false;
        self.has_src = false;
        self.saw_non_whitespace = false;
        self.buf.clear();
        self.span = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use html_inspector::{Config, InputFormat, RuleSet};
    use html_inspector_html::HtmlEventSource;

    #[test]
    fn script_with_src_and_text_warns() {
        let src = HtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<script src=\"x.js\">console.log(1)</script>",
        )
        .unwrap();
        let report = html_inspector::validate_events(
            src,
            RuleSet::new().push(ScriptTextContentConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.script.text_content.non_empty")
        );
    }

    #[test]
    fn script_with_src_and_whitespace_is_ok() {
        let src = HtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<script src=\"x.js\">   \n</script>",
        )
        .unwrap();
        let report = html_inspector::validate_events(
            src,
            RuleSet::new().push(ScriptTextContentConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(report.messages.is_empty());
    }
}
