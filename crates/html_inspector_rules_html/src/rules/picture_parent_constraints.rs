use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct PictureParentConstraints;

impl Rule for PictureParentConstraints {
    fn id(&self) -> &'static str {
        "html.picture.parent_constraints"
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
        let ParseEvent::StartTag { name, span, .. } = event else {
            return;
        };
        if !is(ctx, name, "picture") {
            return;
        }

        if ctx.document.section == html_inspector::DocumentSection::Head
            && ctx.has_ancestor("noscript")
        {
            out.push(Message::new(
                "html.picture.in_noscript_in_head.disallowed",
                Severity::Error,
                Category::Html,
                "Bad start tag in “picture” in “noscript” in “head”.",
                *span,
            ));
        }

        let Some(parent) = ctx.current_parent() else {
            return;
        };
        if is(ctx, parent, "dl") {
            out.push(Message::new(
                "html.picture.parent.dl.disallowed",
                Severity::Error,
                Category::Html,
                "Element “picture” not allowed as child of “dl” in this context.",
                *span,
            ));
        } else if is(ctx, parent, "hgroup") {
            out.push(Message::new(
                "html.picture.parent.hgroup.disallowed",
                Severity::Error,
                Category::Html,
                "Element “picture” not allowed as child of “hgroup” in this context.",
                *span,
            ));
        } else if is(ctx, parent, "rp") {
            out.push(Message::new(
                "html.picture.parent.rp.disallowed",
                Severity::Error,
                Category::Html,
                "Element “picture” not allowed as child of “rp” in this context.",
                *span,
            ));
        } else if is(ctx, parent, "ul") {
            out.push(Message::new(
                "html.picture.parent.ul.disallowed",
                Severity::Error,
                Category::Html,
                "Element “picture” not allowed as child of “ul” in this context.",
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

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::VecDeque;

    use html_inspector::{Config, EventSource, InputFormat, RuleSet, ValidatorError};
    use html_inspector_html::HtmlEventSource;

    struct VecSource {
        name: String,
        format: InputFormat,
        events: VecDeque<ParseEvent>,
    }

    impl VecSource {
        fn new(format: InputFormat, events: Vec<ParseEvent>) -> Self {
            Self {
                name: "t".to_string(),
                format,
                events: events.into(),
            }
        }
    }

    impl EventSource for VecSource {
        fn source_name(&self) -> &str {
            &self.name
        }

        fn format(&self) -> InputFormat {
            self.format
        }

        fn next_event(&mut self) -> Result<Option<ParseEvent>, ValidatorError> {
            Ok(self.events.pop_front())
        }
    }

    #[test]
    fn picture_with_no_parent_does_not_emit_parent_errors() {
        let src = HtmlEventSource::from_str("t", InputFormat::Html, "<picture></picture>").unwrap();
        let rules = RuleSet::new().push(PictureParentConstraints::default());
        let mut report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
        report.messages.push(html_inspector::Message::new(
            "test.dummy",
            html_inspector::Severity::Info,
            html_inspector::Category::Html,
            "x".to_string(),
            None,
        ));
        assert!(
            !report
                .messages
                .iter()
                .any(|m| m.code == "html.picture.parent.dl.disallowed")
        );
    }

    #[test]
    fn picture_in_dl_is_disallowed_and_noscript_in_head_is_disallowed() {
        let src = HtmlEventSource::from_str("t", InputFormat::Html, "<dl><picture></picture></dl>")
            .unwrap();
        let rules = RuleSet::new().push(PictureParentConstraints::default());
        let report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.picture.parent.dl.disallowed")
        );

        let src = VecSource::new(
            InputFormat::Html,
            vec![
                ParseEvent::StartTag {
                    name: "noscript".to_string(),
                    attrs: vec![],
                    self_closing: false,
                    span: None,
                },
                ParseEvent::StartTag {
                    name: "picture".to_string(),
                    attrs: vec![],
                    self_closing: false,
                    span: None,
                },
            ],
        );
        let rules = RuleSet::new().push(PictureParentConstraints::default());
        let report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.picture.in_noscript_in_head.disallowed")
        );
    }
}
