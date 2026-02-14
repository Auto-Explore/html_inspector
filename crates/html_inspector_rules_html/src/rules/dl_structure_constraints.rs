use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct DlStructureConstraints {
    stack: Vec<DlCtx>,
}

#[derive(Clone, Copy, Debug)]
struct DlCtx {
    mode: DlMode,
    saw_dt: bool,
    saw_dd: bool,
    kind: DlKind,
    pending_dd: bool,
    saw_any_dt: bool,
    dd_before_dt: bool,
    span: Option<html_inspector::Span>,
    saw_dt_end_tag: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DlMode {
    Dl,
    Dt,
    Dd,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DlKind {
    Unknown,
    DirectItems,
    DivGroups,
}

impl Rule for DlStructureConstraints {
    fn id(&self) -> &'static str {
        "html.dl.structure_constraints"
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
                    if let Some(parent) = self.stack.last()
                        && parent.mode == DlMode::Dl
                        && !ctx.has_ancestor("template")
                    {
                        let current_parent = ctx.current_parent().unwrap_or("");
                        let msg = if is(ctx, current_parent, "div") {
                            "Element “dl” not allowed as child of “div” in this context."
                                .to_string()
                        } else {
                            "Element “dl” not allowed as child of “dl” in this context.".to_string()
                        };
                        out.push(Message::new(
                            "html.dl.child.dl.disallowed",
                            Severity::Error,
                            Category::Html,
                            msg,
                            *span,
                        ));
                    }
                    if !*self_closing {
                        self.stack.push(DlCtx {
                            mode: DlMode::Dl,
                            saw_dt: false,
                            saw_dd: false,
                            kind: DlKind::Unknown,
                            pending_dd: false,
                            saw_any_dt: false,
                            dd_before_dt: false,
                            span: *span,
                            saw_dt_end_tag: false,
                        });
                    }
                    return;
                }

                let Some(top) = self.stack.last_mut() else {
                    return;
                };

                // Ignore template contents for dl structural requirements (suite coverage).
                if ctx.has_ancestor("template") {
                    return;
                }

                let in_div_group = in_dl_div_group(ctx);

                if is(ctx, name, "dt") {
                    top.saw_dt = true;
                    top.mode = DlMode::Dt;
                    if !in_div_group {
                        if top.kind == DlKind::DivGroups {
                            out.push(Message::new(
                                "html.dl.mixed.div_then_dt",
                                Severity::Error,
                                Category::Html,
                                "Element “dt” not allowed as child of “dl” in this context.",
                                *span,
                            ));
                        } else {
                            top.kind = DlKind::DirectItems;
                        }
                        top.mode = DlMode::Dt;
                        top.pending_dd = true;
                        top.saw_any_dt = true;
                    }
                    return;
                }

                if is(ctx, name, "dd") {
                    top.saw_dd = true;
                    top.mode = DlMode::Dd;
                    if !in_div_group {
                        if top.kind == DlKind::DivGroups {
                            out.push(Message::new(
                                "html.dl.mixed.div_then_dd",
                                Severity::Error,
                                Category::Html,
                                "Element “dd” not allowed as child of “dl” in this context.",
                                *span,
                            ));
                        } else {
                            top.kind = DlKind::DirectItems;
                        }
                        if !top.saw_any_dt {
                            top.dd_before_dt = true;
                        }
                        // A <dd> implicitly closes any open <dt>.
                        top.pending_dd = false;
                    }
                    return;
                }

                if is(ctx, name, "div") && top.mode == DlMode::Dl && !in_div_group {
                    if top.kind == DlKind::DirectItems {
                        out.push(Message::new(
                            "html.dl.mixed.dtdd_then_div",
                            Severity::Error,
                            Category::Html,
                            "Element “div” not allowed as child of “dl” in this context.",
                            *span,
                        ));
                    } else {
                        top.kind = DlKind::DivGroups;
                    }
                }

                // Child <dl> checks are handled in the <dl> branch above.
            }
            ParseEvent::EndTag { name, span } => {
                if is(ctx, name, "dl") {
                    if let Some(top) = self.stack.pop() {
                        if top.dd_before_dt {
                            out.push(Message::new(
                                "html.dl.dd_before_dt",
                                Severity::Error,
                                Category::Html,
                                "Element “dl” is missing a required child element.",
                                *span,
                            ));
                        } else if top.pending_dd {
                            let msg = if top.saw_dt_end_tag {
                                "Element “dl” is missing a required instance of one or more of the following child elements: “dd”."
                                    .to_string()
                            } else {
                                "Element “dl” is missing a required instance of child element “dd”."
                                    .to_string()
                            };
                            out.push(Message::new(
                                "html.dl.missing_dd",
                                Severity::Error,
                                Category::Html,
                                msg,
                                *span,
                            ));
                        } else if top.saw_dt && !top.saw_dd {
                            out.push(Message::new(
                                "html.dl.missing_dd",
                                Severity::Error,
                                Category::Html,
                                "Element “dl” is missing a required instance of one or more of the following child elements: “dd”.",
                                *span,
                            ));
                        } else if top.saw_dd && !top.saw_dt {
                            out.push(Message::new(
                                "html.dl.missing_dt",
                                Severity::Error,
                                Category::Html,
                                "Element “dl” is missing a required child element.",
                                *span,
                            ));
                        }
                    }
                    return;
                }

                if let Some(top) = self.stack.last_mut()
                    && (is(ctx, name, "dt") || is(ctx, name, "dd"))
                {
                    if is(ctx, name, "dt") && span.is_some() {
                        top.saw_dt_end_tag = true;
                    }
                    top.mode = DlMode::Dl;
                }
            }
            ParseEvent::Text { text, span } => {
                let Some(top) = self.stack.last() else { return };
                if top.mode != DlMode::Dl {
                    return;
                }
                if ctx.has_ancestor("template") {
                    return;
                }
                if !ctx.current_parent().is_some_and(|p| is(ctx, p, "dl")) {
                    return;
                }
                if text.chars().any(|c| !c.is_whitespace()) {
                    out.push(Message::new(
                        "html.dl.text.disallowed",
                        Severity::Error,
                        Category::Html,
                        "Text not allowed in “dl” in this context.",
                        span.or(top.span),
                    ));
                }
            }
            _ => {}
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, _out: &mut dyn MessageSink) {
        self.stack.clear();
    }
}

fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    match ctx.format {
        html_inspector::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector::InputFormat::Xhtml => actual == expected,
    }
}

fn in_dl_div_group(ctx: &ValidationContext) -> bool {
    let open = ctx.open_elements();
    let Some(div_pos) = open.iter().rposition(|n| is(ctx, n.as_str(), "div")) else {
        return false;
    };
    if div_pos == 0 {
        return false;
    }
    is(ctx, open[div_pos - 1].as_str(), "dl")
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::VecDeque;

    use html_inspector::{Config, EventSource, InputFormat, RuleSet, ValidatorError};

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

    fn start(name: &str) -> ParseEvent {
        ParseEvent::StartTag {
            name: name.to_string(),
            attrs: vec![],
            self_closing: false,
            span: None,
        }
    }

    fn end(name: &str) -> ParseEvent {
        ParseEvent::EndTag {
            name: name.to_string(),
            span: None,
        }
    }

    fn end_with_span(name: &str) -> ParseEvent {
        ParseEvent::EndTag {
            name: name.to_string(),
            span: Some(html_inspector::Span::new(0, 0, 1, 1)),
        }
    }

    #[test]
    fn dl_pushes_context_for_non_self_closing_dl() {
        let src = VecSource::new(InputFormat::Html, vec![start("dl"), end("dl")]);
        let report = html_inspector::validate_events(
            src,
            RuleSet::new().push(DlStructureConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(report.messages.is_empty());
    }

    #[test]
    fn xhtml_tag_name_matching_is_case_sensitive() {
        let ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        assert!(is(&ctx, "dl", "dl"));
        assert!(!is(&ctx, "DL", "dl"));
    }

    #[test]
    fn dl_div_group_dt_without_dd_emits_missing_dd() {
        let src = VecSource::new(
            InputFormat::Html,
            vec![start("dl"), start("div"), start("dt"), end("dl")],
        );
        let report = html_inspector::validate_events(
            src,
            RuleSet::new().push(DlStructureConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.dl.missing_dd")
        );
    }

    #[test]
    fn dl_div_group_dd_without_dt_emits_missing_dt() {
        let src = VecSource::new(
            InputFormat::Html,
            vec![start("dl"), start("div"), start("dd"), end("dl")],
        );
        let report = html_inspector::validate_events(
            src,
            RuleSet::new().push(DlStructureConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.dl.missing_dt")
        );
    }

    #[test]
    fn nested_dl_inside_div_in_dl_emits_expected_error_message_variant() {
        let src = VecSource::new(
            InputFormat::Html,
            vec![
                start("dl"),
                start("div"),
                start("dl"),
                end("dl"),
                end("div"),
                end("dl"),
            ],
        );
        let report = html_inspector::validate_events(
            src,
            RuleSet::new().push(DlStructureConstraints::default()),
            Config::default(),
        )
        .unwrap();
        let msg = report
            .messages
            .iter()
            .find(|m| m.code == "html.dl.child.dl.disallowed")
            .expect("expected nested <dl> error");
        assert!(msg.message.contains("child of “div”"));
    }

    #[test]
    fn mixed_dl_div_groups_then_dt_emits_mixed_error() {
        let src = VecSource::new(
            InputFormat::Html,
            vec![
                start("dl"),
                start("div"),
                end("div"),
                start("dt"),
                end("dl"),
            ],
        );
        let report = html_inspector::validate_events(
            src,
            RuleSet::new().push(DlStructureConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.dl.mixed.div_then_dt")
        );
    }

    #[test]
    fn mixed_dl_div_groups_then_dd_emits_mixed_error() {
        let src = VecSource::new(
            InputFormat::Html,
            vec![
                start("dl"),
                start("div"),
                end("div"),
                start("dd"),
                end("dl"),
            ],
        );
        let report = html_inspector::validate_events(
            src,
            RuleSet::new().push(DlStructureConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.dl.mixed.div_then_dd")
        );
    }

    #[test]
    fn mixed_dl_direct_dt_then_div_emits_mixed_error() {
        let src = VecSource::new(
            InputFormat::Html,
            vec![start("dl"), start("dt"), end("dt"), start("div"), end("dl")],
        );
        let report = html_inspector::validate_events(
            src,
            RuleSet::new().push(DlStructureConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.dl.mixed.dtdd_then_div")
        );
    }

    #[test]
    fn dd_before_dt_emits_missing_required_child_error() {
        let src = VecSource::new(InputFormat::Html, vec![start("dl"), start("dd"), end("dl")]);
        let report = html_inspector::validate_events(
            src,
            RuleSet::new().push(DlStructureConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.dl.dd_before_dt")
        );
    }

    #[test]
    fn dt_end_tag_span_changes_missing_dd_message_variant() {
        let src = VecSource::new(
            InputFormat::Html,
            vec![
                start("dl"),
                start("dt"),
                end_with_span("dt"),
                end_with_span("dl"),
            ],
        );
        let report = html_inspector::validate_events(
            src,
            RuleSet::new().push(DlStructureConstraints::default()),
            Config::default(),
        )
        .unwrap();
        let msg = report
            .messages
            .iter()
            .find(|m| m.code == "html.dl.missing_dd")
            .expect("expected missing <dd> message");
        assert!(
            msg.message
                .contains("one or more of the following child elements")
        );
    }

    #[test]
    fn text_inside_dt_does_not_trigger_dl_text_disallowed() {
        let src = VecSource::new(
            InputFormat::Html,
            vec![
                start("dl"),
                start("dt"),
                ParseEvent::Text {
                    text: "x".to_string(),
                    span: None,
                },
                end("dl"),
            ],
        );
        let report = html_inspector::validate_events(
            src,
            RuleSet::new().push(DlStructureConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(
            !report
                .messages
                .iter()
                .any(|m| m.code == "html.dl.text.disallowed")
        );
    }

    #[test]
    fn rule_covers_unmatched_event_branch() {
        struct Sink(Vec<html_inspector::Message>);
        impl html_inspector::MessageSink for Sink {
            fn push(&mut self, msg: html_inspector::Message) {
                self.0.push(msg);
            }
        }
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink(Vec::new());
        let mut rule = DlStructureConstraints::default();
        rule.on_event(
            &ParseEvent::Comment {
                text: "x".to_string(),
                span: None,
            },
            &mut ctx,
            &mut sink,
        );
        assert!(sink.0.is_empty());
        html_inspector::MessageSink::push(
            &mut sink,
            html_inspector::Message::new(
                "test.dummy",
                html_inspector::Severity::Info,
                html_inspector::Category::Html,
                "x".to_string(),
                None,
            ),
        );
        assert_eq!(sink.0.len(), 1);
    }

    #[test]
    fn in_dl_div_group_returns_false_when_open_stack_starts_with_div() {
        #[derive(Default)]
        struct Probe;
        impl html_inspector::Rule for Probe {
            fn id(&self) -> &'static str {
                "test.dl.in_div_group.probe"
            }
            fn interest(&self) -> html_inspector::Interest {
                html_inspector::Interest::TEXT
            }
            fn on_event(
                &mut self,
                event: &ParseEvent,
                ctx: &mut ValidationContext,
                _out: &mut dyn html_inspector::MessageSink,
            ) {
                if matches!(event, ParseEvent::Text { .. }) {
                    assert_eq!(ctx.open_elements(), ["div"]);
                    assert!(!in_dl_div_group(ctx));
                }
            }
        }

        let src = VecSource::new(
            InputFormat::Html,
            vec![
                start("div"),
                ParseEvent::Text {
                    text: "x".to_string(),
                    span: None,
                },
                end("div"),
            ],
        );
        let _report = html_inspector::validate_events(
            src,
            RuleSet::new().push(Probe::default()),
            Config::default(),
        )
        .unwrap();
    }
}
