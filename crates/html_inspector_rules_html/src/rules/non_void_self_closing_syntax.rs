use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct NonVoidSelfClosingSyntax;

impl Rule for NonVoidSelfClosingSyntax {
    fn id(&self) -> &'static str {
        "html.parse.non_void_self_closing_syntax"
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
        if ctx.format != html_inspector_core::InputFormat::Html {
            return;
        }
        let ParseEvent::StartTag {
            name,
            self_closing,
            span,
            ..
        } = event
        else {
            return;
        };
        if !*self_closing {
            return;
        }
        if html_inspector_core::is_void_html_element(name) {
            return;
        }

        out.push(Message::new(
            "html.parse.self_closing.non_void",
            Severity::Error,
            Category::Html,
            "Self-closing syntax (“/>”) used on a non-void HTML element. Ignoring the slash and treating as a start tag.",
            *span,
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use html_inspector_core::{Config, InputFormat};

    #[test]
    fn rule_ignores_non_start_tag_events() {
        struct Sink(Vec<html_inspector_core::Message>);
        impl html_inspector_core::MessageSink for Sink {
            fn push(&mut self, msg: html_inspector_core::Message) {
                self.0.push(msg);
            }
        }

        let mut ctx =
            html_inspector_core::ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink(Vec::new());
        let mut rule = NonVoidSelfClosingSyntax::default();
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
                html_inspector_core::Category::Html,
                "x".to_string(),
                None,
            ),
        );
        assert_eq!(sink.0.len(), 1);
    }

    #[test]
    fn emits_error_for_non_void_self_closing_syntax_but_not_for_void_elements() {
        struct Sink(Vec<html_inspector_core::Message>);
        impl html_inspector_core::MessageSink for Sink {
            fn push(&mut self, msg: html_inspector_core::Message) {
                self.0.push(msg);
            }
        }

        let mut ctx =
            html_inspector_core::ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink(Vec::new());
        let mut rule = NonVoidSelfClosingSyntax::default();

        rule.on_event(
            &ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![],
                self_closing: true,
                span: None,
            },
            &mut ctx,
            &mut sink,
        );
        assert!(
            sink.0
                .iter()
                .any(|m| m.code == "html.parse.self_closing.non_void")
        );

        let mut sink = Sink(Vec::new());
        rule.on_event(
            &ParseEvent::StartTag {
                name: "br".to_string(),
                attrs: vec![],
                self_closing: true,
                span: None,
            },
            &mut ctx,
            &mut sink,
        );
        assert!(
            !sink
                .0
                .iter()
                .any(|m| m.code == "html.parse.self_closing.non_void")
        );
    }
}
