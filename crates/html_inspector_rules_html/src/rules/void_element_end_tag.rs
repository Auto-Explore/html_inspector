use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct VoidElementEndTag;

impl Rule for VoidElementEndTag {
    fn id(&self) -> &'static str {
        "html.void_element.end_tag"
    }

    fn interest(&self) -> Interest {
        Interest::END_TAG
    }

    fn on_event(
        &mut self,
        event: &ParseEvent,
        ctx: &mut ValidationContext,
        out: &mut dyn MessageSink,
    ) {
        // In XML/XHTML syntax, `</img>` etc. are well-formed and VNU doesn't report them as errors.
        if ctx.format == html_inspector_core::InputFormat::Xhtml {
            return;
        }

        let ParseEvent::EndTag { name, span } = event else {
            return;
        };
        if !html_inspector_core::is_void_html_element(name) {
            return;
        }
        out.push(Message::new(
            "html.void_element.end_tag",
            Severity::Error,
            Category::Html,
            format!("End tag “{name}”."),
            *span,
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use html_inspector_core::{Config, InputFormat};

    #[test]
    fn void_end_tags_are_errors_in_html_but_not_in_xhtml() {
        struct Sink(Vec<html_inspector_core::Message>);
        impl html_inspector_core::MessageSink for Sink {
            fn push(&mut self, msg: html_inspector_core::Message) {
                self.0.push(msg);
            }
        }

        let mut rule = VoidElementEndTag::default();

        let mut ctx =
            html_inspector_core::ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink(Vec::new());
        rule.on_event(
            &ParseEvent::EndTag {
                name: "img".to_string(),
                span: None,
            },
            &mut ctx,
            &mut sink,
        );
        assert!(sink.0.iter().any(|m| m.code == "html.void_element.end_tag"));

        let mut ctx =
            html_inspector_core::ValidationContext::new(Config::default(), InputFormat::Xhtml);
        let mut sink = Sink(Vec::new());
        rule.on_event(
            &ParseEvent::EndTag {
                name: "img".to_string(),
                span: None,
            },
            &mut ctx,
            &mut sink,
        );
        assert!(sink.0.is_empty());
    }

    #[test]
    fn rule_on_event_has_non_end_tag_early_return() {
        struct Sink(Vec<html_inspector_core::Message>);
        impl html_inspector_core::MessageSink for Sink {
            fn push(&mut self, msg: html_inspector_core::Message) {
                self.0.push(msg);
            }
        }
        let mut ctx =
            html_inspector_core::ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink(Vec::new());
        let mut rule = VoidElementEndTag::default();
        rule.on_event(
            &ParseEvent::StartTag {
                name: "img".to_string(),
                attrs: vec![],
                self_closing: false,
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
}
