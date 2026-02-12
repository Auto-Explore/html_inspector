use std::collections::HashSet;

use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, Span, ValidationContext,
};

#[derive(Default)]
pub struct AriaIdrefExists {
    ids: HashSet<String>,
    refs: Vec<(String, String, Option<Span>)>,
}

impl Rule for AriaIdrefExists {
    fn id(&self) -> &'static str {
        "aria.idref.exists"
    }

    fn interest(&self) -> Interest {
        Interest::START_TAG | Interest::FINISH
    }

    fn init(&mut self, _ctx: &ValidationContext) {
        self.ids.clear();
        self.refs.clear();
    }

    fn on_event(
        &mut self,
        event: &ParseEvent,
        ctx: &mut ValidationContext,
        _out: &mut dyn MessageSink,
    ) {
        let ParseEvent::StartTag { attrs, span, .. } = event else {
            return;
        };

        if let Some(id) = ctx.attr_value(attrs, "id") {
            if !id.is_empty() {
                self.ids.insert(id.to_string());
            }
        }

        for attr in [
            "aria-controls",
            "aria-describedby",
            "aria-flowto",
            "aria-labelledby",
            "aria-owns",
        ] {
            let Some(v) = ctx.attr_value(attrs, attr) else {
                continue;
            };
            for token in v.split_ascii_whitespace() {
                self.refs.push((attr.to_string(), token.to_string(), *span));
            }
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, out: &mut dyn MessageSink) {
        for (attr, token, span) in self.refs.drain(..) {
            if !self.ids.contains(&token) {
                out.push(Message::new(
                    format!("aria.idref.missing.{attr}"),
                    Severity::Error,
                    Category::Aria,
                    format!(
                        "The “{attr}” attribute references “{token}”, which is not the ID of any element in this document."
                    ),
                    span,
                ));
            }
        }
        self.ids.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::AriaIdrefExists;
    use html_inspector_core::{
        validate_events, Attribute, Config, EventSource, InputFormat, ParseEvent, RuleSet, Span,
        ValidatorError,
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

    #[test]
    fn splits_idrefs_by_ascii_whitespace_without_emitting_empty_tokens() {
        let span = Some(Span::new(1, 2, 1, 1));
        let src = VecSource::new(
            InputFormat::Html,
            vec![
                ParseEvent::StartTag {
                    name: "div".to_string(),
                    attrs: vec![Attribute {
                        name: "id".to_string(),
                        value: Some("b".to_string()),
                        span: None,
                    }],
                    self_closing: true,
                    span: None,
                },
                ParseEvent::StartTag {
                    name: "div".to_string(),
                    attrs: vec![Attribute {
                        name: "aria-controls".to_string(),
                        value: Some("  a  b\tc  ".to_string()),
                        span: None,
                    }],
                    self_closing: true,
                    span,
                },
            ],
        );

        let report = validate_events(
            src,
            RuleSet::new().push(AriaIdrefExists::default()),
            Config::default(),
        )
        .unwrap();
        let codes: Vec<&str> = report.messages.iter().map(|m| m.code.as_str()).collect();
        assert_eq!(
            codes,
            vec![
                "aria.idref.missing.aria-controls",
                "aria.idref.missing.aria-controls"
            ]
        );
        assert!(report.messages[0].message.contains("references “a”"));
        assert!(report.messages[1].message.contains("references “c”"));
        assert_eq!(report.messages[0].span, span);
        assert_eq!(report.messages[1].span, span);
    }
}
