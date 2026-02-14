use std::collections::BTreeMap;

use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct DlDuplicateDtName {
    dl_stack: Vec<BTreeMap<String, Option<html_inspector::Span>>>,
    dt_text: Option<String>,
    dt_span: Option<html_inspector::Span>,
}

impl Rule for DlDuplicateDtName {
    fn id(&self) -> &'static str {
        "html.dl.duplicate_dt_name"
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
                name,
                span,
                self_closing,
                ..
            } => {
                if is(ctx, name, "dl") {
                    if !*self_closing {
                        self.dl_stack.push(BTreeMap::new());
                    }
                    return;
                }

                if is(ctx, name, "dt") {
                    self.finish_dt(ctx, out);
                    if !self_closing {
                        self.dt_text = Some(String::new());
                        self.dt_span = *span;
                    }
                    return;
                }

                if is(ctx, name, "dd") {
                    // <dd> implicitly closes an open <dt>.
                    self.finish_dt(ctx, out);
                }
            }
            ParseEvent::Text { text, .. } => {
                if let Some(buf) = self.dt_text.as_mut() {
                    buf.push_str(text);
                }
            }
            ParseEvent::EndTag { name, .. } => {
                if is(ctx, name, "dt") {
                    self.finish_dt(ctx, out);
                    return;
                }
                if is(ctx, name, "dl") {
                    self.finish_dt(ctx, out);
                    self.dl_stack.pop();
                }
            }
            _ => {}
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, _out: &mut dyn MessageSink) {
        self.dl_stack.clear();
        self.dt_text = None;
        self.dt_span = None;
    }
}

impl DlDuplicateDtName {
    fn finish_dt(&mut self, _ctx: &ValidationContext, out: &mut dyn MessageSink) {
        let Some(raw) = self.dt_text.take() else {
            return;
        };
        let name = normalize_dt_name(&raw);
        let span = self.dt_span.take();

        if name.is_empty() {
            return;
        }
        let Some(seen) = self.dl_stack.last_mut() else {
            return;
        };

        if let Some(first_span) = seen.get(&name).copied() {
            out.push(Message::new(
                "html.dl.duplicate_dt_name",
                Severity::Warning,
                Category::Html,
                format!("Duplicate “dt” name “{name}” in “dl” element. Within a single “dl” element, there should not be more than one “dt” element for each name."),
                span,
            ));
            let note = Message::new(
                "html.dl.duplicate_dt_name.first_occurrence",
                Severity::Info,
                Category::Html,
                format!("The first occurrence of “dt” name “{name}” was here."),
                first_span,
            );
            out.push(note);
        } else {
            seen.insert(name, span);
        }
    }
}

fn normalize_dt_name(s: &str) -> String {
    let mut words = s.split_whitespace();
    let Some(first) = words.next() else {
        return String::new();
    };

    // Upper bound; avoids reallocation while still being a single pass.
    let mut out = String::with_capacity(s.len());
    out.push_str(first);
    for word in words {
        out.push(' ');
        out.push_str(word);
    }
    out
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

    use html_inspector::{
        Config, EventSource, InputFormat, ParseEvent, RuleSet, Span, ValidatorError,
    };

    struct VecSource {
        name: String,
        format: InputFormat,
        events: std::vec::IntoIter<ParseEvent>,
    }

    impl VecSource {
        fn new(format: InputFormat, events: Vec<ParseEvent>) -> Self {
            Self {
                name: "vec".to_string(),
                format,
                events: events.into_iter(),
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
            Ok(self.events.next())
        }
    }

    fn start(name: &str) -> ParseEvent {
        ParseEvent::StartTag {
            name: name.to_string(),
            attrs: vec![],
            self_closing: false,
            span: Some(Span::new(0, 0, 1, 1)),
        }
    }

    fn end(name: &str) -> ParseEvent {
        ParseEvent::EndTag {
            name: name.to_string(),
            span: None,
        }
    }

    fn text(t: &str) -> ParseEvent {
        ParseEvent::Text {
            text: t.to_string(),
            span: None,
        }
    }

    #[test]
    fn normalize_dt_name_collapses_whitespace() {
        assert_eq!(normalize_dt_name(""), "");
        assert_eq!(normalize_dt_name("   \n\t  "), "");
        assert_eq!(normalize_dt_name("a"), "a");
        assert_eq!(normalize_dt_name("  a  b\tc \n d "), "a b c d");
    }

    #[test]
    fn duplicate_dt_names_are_detected_after_normalization() {
        let src = VecSource::new(
            InputFormat::Html,
            vec![
                start("dl"),
                start("dt"),
                text(" A \n  B "),
                end("dt"),
                start("dt"),
                text("A B"),
                end("dt"),
                end("dl"),
            ],
        );
        let rules = RuleSet::new().push(DlDuplicateDtName::default());
        let report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.dl.duplicate_dt_name")
        );
    }

    #[test]
    fn dt_names_do_not_carry_over_between_dl_elements() {
        let src = VecSource::new(
            InputFormat::Html,
            vec![
                start("dl"),
                start("dt"),
                text("A"),
                end("dt"),
                end("dl"),
                start("dl"),
                start("dt"),
                text("A"),
                end("dt"),
                end("dl"),
            ],
        );
        let rules = RuleSet::new().push(DlDuplicateDtName::default());
        let report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
        assert!(
            !report
                .messages
                .iter()
                .any(|m| m.code == "html.dl.duplicate_dt_name")
        );
    }

    #[test]
    fn warns_for_each_duplicate_after_first() {
        let src = VecSource::new(
            InputFormat::Html,
            vec![
                start("dl"),
                start("dt"),
                text("A"),
                end("dt"),
                start("dt"),
                text("A"),
                end("dt"),
                start("dt"),
                text("A"),
                end("dt"),
                end("dl"),
            ],
        );
        let rules = RuleSet::new().push(DlDuplicateDtName::default());
        let report = html_inspector::validate_events(src, rules, Config::default()).unwrap();
        let warnings = report
            .messages
            .iter()
            .filter(|m| m.code == "html.dl.duplicate_dt_name")
            .count();
        assert_eq!(warnings, 2);
    }

    #[test]
    fn duplicate_dt_emits_warning_and_info_for_first_occurrence() {
        let span1 = Some(Span::new(0, 0, 1, 1));
        let span2 = Some(Span::new(0, 0, 2, 1));
        let src = VecSource::new(
            InputFormat::Html,
            vec![
                start("dl"),
                ParseEvent::StartTag {
                    name: "dt".to_string(),
                    attrs: vec![],
                    self_closing: false,
                    span: span1,
                },
                text("Term"),
                end("dt"),
                ParseEvent::StartTag {
                    name: "dt".to_string(),
                    attrs: vec![],
                    self_closing: false,
                    span: span2,
                },
                text("Term"),
                end("dt"),
                end("dl"),
            ],
        );

        let report = html_inspector::validate_events(
            src,
            RuleSet::new().push(DlDuplicateDtName::default()),
            Config::default(),
        )
        .unwrap();

        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.dl.duplicate_dt_name")
        );
        let info = report
            .messages
            .iter()
            .find(|m| m.code == "html.dl.duplicate_dt_name.first_occurrence")
            .expect("missing info message");
        assert_eq!(info.span, span1);
    }
}
