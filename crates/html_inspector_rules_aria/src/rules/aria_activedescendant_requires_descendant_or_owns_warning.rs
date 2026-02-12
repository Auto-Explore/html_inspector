use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, Span, ValidationContext,
};

#[derive(Default)]
pub struct AriaActivedescendantRequiresDescendantOrOwnsWarning {
    stack: Vec<OpenEntry>,
}

#[derive(Clone, Debug)]
struct OpenEntry {
    name: String,
    target_id: Option<String>,
    has_owns: bool,
    found_descendant: bool,
    span: Option<Span>,
}

impl Rule for AriaActivedescendantRequiresDescendantOrOwnsWarning {
    fn id(&self) -> &'static str {
        "aria.activedescendant.descendant_or_owns"
    }

    fn max_severity(&self) -> Severity {
        Severity::Warning
    }

    fn interest(&self) -> Interest {
        Interest::START_TAG | Interest::END_TAG | Interest::FINISH
    }

    fn init(&mut self, _ctx: &ValidationContext) {
        self.stack.clear();
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
                attrs,
                self_closing,
                span,
            } => {
                if let Some(id) = ctx.attr_value(attrs, "id").filter(|id| !id.is_empty()) {
                    for e in self.stack.iter_mut() {
                        if e.target_id.as_deref() == Some(id) {
                            e.found_descendant = true;
                        }
                    }
                }

                let has_owns = ctx.has_attr(attrs, "aria-owns");
                let target_id = ctx
                    .attr_value(attrs, "aria-activedescendant")
                    .and_then(|v| {
                        let v = v.trim();
                        (!v.is_empty()).then(|| v.to_string())
                    });

                let pushes = match ctx.format {
                    html_inspector_core::InputFormat::Html => {
                        !html_inspector_core::is_void_html_element(name)
                    }
                    html_inspector_core::InputFormat::Xhtml => !*self_closing,
                };
                if !pushes {
                    if target_id.is_some() && !has_owns {
                        out.push(Message::new(
                            "aria.activedescendant.descendant_or_owns",
                            Severity::Warning,
                            Category::Aria,
                            "Attribute “aria-activedescendant” value should either refer to a descendant element, or should be accompanied by attribute “aria-owns”.",
                            *span,
                        ));
                    }
                    return;
                }

                self.stack.push(OpenEntry {
                    name: name.clone(),
                    target_id,
                    has_owns,
                    found_descendant: false,
                    span: *span,
                });
            }
            ParseEvent::EndTag { name, .. } => {
                self.close_up_to(name, ctx, out);
            }
            _ => {}
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, out: &mut dyn MessageSink) {
        while let Some(e) = self.stack.pop() {
            maybe_emit_warning(&e, out);
        }
    }
}

impl AriaActivedescendantRequiresDescendantOrOwnsWarning {
    fn close_up_to(&mut self, name: &str, ctx: &ValidationContext, out: &mut dyn MessageSink) {
        let Some(pos) = self.stack.iter().rposition(|e| ctx.name_is(&e.name, name)) else {
            return;
        };

        for e in self.stack.drain(pos..) {
            maybe_emit_warning(&e, out);
        }
    }
}

fn maybe_emit_warning(e: &OpenEntry, out: &mut dyn MessageSink) {
    if e.target_id.is_some() && !e.has_owns && !e.found_descendant {
        out.push(Message::new(
            "aria.activedescendant.descendant_or_owns",
            Severity::Warning,
            Category::Aria,
            "Attribute “aria-activedescendant” value should either refer to a descendant element, or should be accompanied by attribute “aria-owns”.",
            e.span,
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::VecDeque;

    use html_inspector_core::{
        Attribute, Config, EventSource, InputFormat, RuleSet, ValidationContext, ValidatorError,
    };

    #[derive(Default)]
    struct Sink(Vec<Message>);
    impl MessageSink for Sink {
        fn push(&mut self, msg: Message) {
            self.0.push(msg);
        }
    }

    fn attr(name: &str, value: Option<&str>) -> Attribute {
        Attribute {
            name: name.to_string(),
            value: value.map(|v| v.to_string()),
            span: None,
        }
    }

    #[test]
    fn void_elements_emit_warning_immediately_when_no_owns() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink::default();
        let mut rule = AriaActivedescendantRequiresDescendantOrOwnsWarning::default();

        rule.on_event(
            &ParseEvent::StartTag {
                name: "img".to_string(),
                attrs: vec![attr("aria-activedescendant", Some("x"))],
                self_closing: true,
                span: None,
            },
            &mut ctx,
            &mut sink,
        );
        assert!(sink
            .0
            .iter()
            .any(|m| m.code == "aria.activedescendant.descendant_or_owns"));
    }

    #[test]
    fn whitespace_only_activedescendant_is_ignored() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink::default();
        let mut rule = AriaActivedescendantRequiresDescendantOrOwnsWarning::default();

        rule.on_event(
            &ParseEvent::StartTag {
                name: "img".to_string(),
                attrs: vec![attr("aria-activedescendant", Some(" \t\n "))],
                self_closing: true,
                span: None,
            },
            &mut ctx,
            &mut sink,
        );
        assert!(sink.0.is_empty());
    }

    #[test]
    fn finish_emits_warning_when_target_is_not_a_descendant() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink::default();
        let mut rule = AriaActivedescendantRequiresDescendantOrOwnsWarning::default();

        rule.on_event(
            &ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![attr("aria-activedescendant", Some("x"))],
                self_closing: false,
                span: None,
            },
            &mut ctx,
            &mut sink,
        );
        rule.on_finish(&mut ctx, &mut sink);
        assert!(sink
            .0
            .iter()
            .any(|m| m.code == "aria.activedescendant.descendant_or_owns"));
    }

    #[test]
    fn descendant_id_suppresses_warning() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink::default();
        let mut rule = AriaActivedescendantRequiresDescendantOrOwnsWarning::default();

        rule.on_event(
            &ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![attr("aria-activedescendant", Some("x"))],
                self_closing: false,
                span: None,
            },
            &mut ctx,
            &mut sink,
        );
        rule.on_event(
            &ParseEvent::StartTag {
                name: "span".to_string(),
                attrs: vec![attr("id", Some("x"))],
                self_closing: false,
                span: None,
            },
            &mut ctx,
            &mut sink,
        );
        rule.on_finish(&mut ctx, &mut sink);
        assert!(sink.0.is_empty());
    }

    #[test]
    fn end_tag_closes_stack_and_can_emit_warning() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink::default();
        let mut rule = AriaActivedescendantRequiresDescendantOrOwnsWarning::default();

        rule.on_event(
            &ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![attr("aria-activedescendant", Some("x"))],
                self_closing: false,
                span: None,
            },
            &mut ctx,
            &mut sink,
        );
        rule.on_event(
            &ParseEvent::EndTag {
                name: "div".to_string(),
                span: None,
            },
            &mut ctx,
            &mut sink,
        );
        assert!(sink
            .0
            .iter()
            .any(|m| m.code == "aria.activedescendant.descendant_or_owns"));
    }

    #[test]
    fn end_tag_matching_is_case_insensitive_in_html() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink::default();
        let mut rule = AriaActivedescendantRequiresDescendantOrOwnsWarning::default();

        rule.on_event(
            &ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![attr("aria-activedescendant", Some("x"))],
                self_closing: false,
                span: None,
            },
            &mut ctx,
            &mut sink,
        );
        rule.on_event(
            &ParseEvent::EndTag {
                name: "DIV".to_string(),
                span: None,
            },
            &mut ctx,
            &mut sink,
        );
        assert_eq!(
            sink.0
                .iter()
                .filter(|m| m.code == "aria.activedescendant.descendant_or_owns")
                .count(),
            1
        );

        // The stack should already be closed; finish must not emit a duplicate warning.
        rule.on_finish(&mut ctx, &mut sink);
        assert_eq!(
            sink.0
                .iter()
                .filter(|m| m.code == "aria.activedescendant.descendant_or_owns")
                .count(),
            1
        );
    }

    #[test]
    fn end_tag_matching_is_case_sensitive_in_xhtml() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        let mut sink = Sink::default();
        let mut rule = AriaActivedescendantRequiresDescendantOrOwnsWarning::default();

        rule.on_event(
            &ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![attr("aria-activedescendant", Some("x"))],
                self_closing: false,
                span: None,
            },
            &mut ctx,
            &mut sink,
        );
        rule.on_event(
            &ParseEvent::EndTag {
                name: "DIV".to_string(),
                span: None,
            },
            &mut ctx,
            &mut sink,
        );
        assert!(sink.0.is_empty());

        rule.on_finish(&mut ctx, &mut sink);
        assert_eq!(
            sink.0
                .iter()
                .filter(|m| m.code == "aria.activedescendant.descendant_or_owns")
                .count(),
            1
        );
    }

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
    fn xhtml_self_closing_elements_emit_warning_immediately() {
        let src = VecSource::new(
            InputFormat::Xhtml,
            vec![ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![attr("aria-activedescendant", Some("x"))],
                self_closing: true,
                span: None,
            }],
        );
        let report = html_inspector_core::validate_events(
            src,
            RuleSet::new().push(AriaActivedescendantRequiresDescendantOrOwnsWarning::default()),
            Config::default(),
        )
        .unwrap();
        assert!(report
            .messages
            .iter()
            .any(|m| m.code == "aria.activedescendant.descendant_or_owns"));
    }

    #[test]
    fn rule_covers_unmatched_event_branch() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink::default();
        let mut rule = AriaActivedescendantRequiresDescendantOrOwnsWarning::default();
        rule.on_event(
            &ParseEvent::Text {
                text: "x".to_string(),
                span: None,
            },
            &mut ctx,
            &mut sink,
        );
        assert!(sink.0.is_empty());
    }
}
