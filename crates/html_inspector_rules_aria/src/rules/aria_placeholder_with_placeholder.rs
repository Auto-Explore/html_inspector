use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct AriaPlaceholderWithPlaceholder;

impl Rule for AriaPlaceholderWithPlaceholder {
    fn id(&self) -> &'static str {
        "aria.aria_placeholder.disallowed_with_placeholder"
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
        let ParseEvent::StartTag { attrs, span, .. } = event else {
            return;
        };

        let has_aria_placeholder = attrs.iter().any(|a| match ctx.format {
            html_inspector_core::InputFormat::Html => {
                a.name.eq_ignore_ascii_case("aria-placeholder")
            }
            html_inspector_core::InputFormat::Xhtml => a.name == "aria-placeholder",
        });
        if !has_aria_placeholder {
            return;
        }

        let has_placeholder = attrs.iter().any(|a| match ctx.format {
            html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case("placeholder"),
            html_inspector_core::InputFormat::Xhtml => a.name == "placeholder",
        });
        if has_placeholder {
            out.push(Message::new(
                "aria.aria_placeholder.disallowed_with_placeholder",
                Severity::Error,
                Category::Aria,
                "The “aria-placeholder” attribute must not be specified on elements that have a “placeholder” attribute.",
                *span,
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use html_inspector_core::{Config, InputFormat};

    struct Sink(Vec<html_inspector_core::Message>);
    impl html_inspector_core::MessageSink for Sink {
        fn push(&mut self, msg: html_inspector_core::Message) {
            self.0.push(msg);
        }
    }

    #[test]
    fn rule_ignores_non_start_tag_events() {
        let mut rule = AriaPlaceholderWithPlaceholder::default();
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink(Vec::new());

        rule.on_event(
            &ParseEvent::EndTag {
                name: "input".to_string(),
                span: None,
            },
            &mut ctx,
            &mut sink,
        );

        assert!(sink.0.is_empty());
    }

    #[test]
    fn xhtml_placeholder_conflict_emits_error() {
        let mut rule = AriaPlaceholderWithPlaceholder::default();
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        let mut sink = Sink(Vec::new());

        rule.on_event(
            &ParseEvent::StartTag {
                name: "input".to_string(),
                attrs: vec![
                    html_inspector_core::Attribute {
                        name: "aria-placeholder".to_string(),
                        value: Some("x".to_string()),
                        span: None,
                    },
                    html_inspector_core::Attribute {
                        name: "placeholder".to_string(),
                        value: Some("y".to_string()),
                        span: None,
                    },
                ],
                self_closing: true,
                span: None,
            },
            &mut ctx,
            &mut sink,
        );

        assert!(sink
            .0
            .iter()
            .any(|m| m.code == "aria.aria_placeholder.disallowed_with_placeholder"));
    }
}
