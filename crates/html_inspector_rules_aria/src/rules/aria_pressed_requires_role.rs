use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct AriaPressedRequiresRole;

impl Rule for AriaPressedRequiresRole {
    fn id(&self) -> &'static str {
        "aria.aria_pressed.requires_role"
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

        if !ctx.name_is(name, "output") {
            return;
        }

        if !ctx.has_attr(attrs, "aria-pressed") {
            return;
        }

        if ctx.has_attr(attrs, "role") {
            return;
        }

        out.push(Message::new(
            "aria.element.output.missing_role",
            Severity::Error,
            Category::Aria,
            "Element “output” is missing required attribute “role”.",
            *span,
        ));
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
        let mut rule = AriaPressedRequiresRole;
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink(Vec::new());

        rule.on_event(
            &ParseEvent::Text {
                text: "x".to_string(),
                span: None,
            },
            &mut ctx,
            &mut sink,
        );

        assert!(sink.0.is_empty());
        html_inspector_core::MessageSink::push(
            &mut sink,
            html_inspector_core::Message::new(
                "test.dummy",
                html_inspector_core::Severity::Info,
                html_inspector_core::Category::Aria,
                "x".to_string(),
                None,
            ),
        );
        assert_eq!(sink.0.len(), 1);
    }

    #[test]
    fn role_attribute_suppresses_error_in_xhtml() {
        let mut rule = AriaPressedRequiresRole;
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        let mut sink = Sink(Vec::new());

        rule.on_event(
            &ParseEvent::StartTag {
                name: "output".to_string(),
                attrs: vec![
                    html_inspector_core::Attribute {
                        name: "aria-pressed".to_string(),
                        value: Some("true".to_string()),
                        span: None,
                    },
                    html_inspector_core::Attribute {
                        name: "role".to_string(),
                        value: Some("button".to_string()),
                        span: None,
                    },
                ],
                self_closing: false,
                span: None,
            },
            &mut ctx,
            &mut sink,
        );

        assert!(sink.0.is_empty());
    }
}
