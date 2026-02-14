use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct AriaSelectedOnOption;

impl Rule for AriaSelectedOnOption {
    fn id(&self) -> &'static str {
        "aria.aria_selected.on_option"
    }

    fn max_severity(&self) -> Severity {
        Severity::Warning
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
        let ParseEvent::StartTag {
            name, attrs, span, ..
        } = event
        else {
            return;
        };
        if !ctx.name_is(name, "option") {
            return;
        }
        if !ctx.has_attr(attrs, "aria-selected") {
            return;
        }
        out.push(Message::new(
            "aria.aria_selected.option.discouraged",
            Severity::Warning,
            Category::Aria,
            "The “aria-selected” attribute should not be used on the “option” element.",
            *span,
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use html_inspector::{Config, InputFormat};

    struct Sink(Vec<html_inspector::Message>);
    impl html_inspector::MessageSink for Sink {
        fn push(&mut self, msg: html_inspector::Message) {
            self.0.push(msg);
        }
    }

    #[test]
    fn rule_ignores_non_start_tag_events() {
        let mut rule = AriaSelectedOnOption;
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink(Vec::new());

        rule.on_event(
            &ParseEvent::Comment {
                text: "x".to_string(),
                span: None,
            },
            &mut ctx,
            &mut sink,
        );

        assert!(sink.0.is_empty());
    }

    #[test]
    fn xhtml_option_with_aria_selected_emits_warning() {
        let mut rule = AriaSelectedOnOption;
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        let mut sink = Sink(Vec::new());

        rule.on_event(
            &ParseEvent::StartTag {
                name: "option".to_string(),
                attrs: vec![html_inspector::Attribute {
                    name: "aria-selected".to_string(),
                    value: Some("true".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            &mut ctx,
            &mut sink,
        );

        assert!(
            sink.0
                .iter()
                .any(|m| m.code == "aria.aria_selected.option.discouraged")
        );
    }
}
