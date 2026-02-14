use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct BdoDir;

impl Rule for BdoDir {
    fn id(&self) -> &'static str {
        "html.bdo.dir"
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
        if !ctx.name_is(name, "bdo") {
            return;
        }

        let dir = attrs.iter().find(|a| ctx.name_is(&a.name, "dir"));

        let Some(dir) = dir else {
            out.push(Message::new(
                "html.bdo.dir.missing",
                Severity::Error,
                Category::Html,
                "Element “bdo” must have attribute “dir”.",
                *span,
            ));
            return;
        };

        if let Some(value) = dir.value.as_deref()
            && value.eq_ignore_ascii_case("auto")
        {
            out.push(Message::new(
                "html.bdo.dir.auto",
                Severity::Error,
                Category::Html,
                "The value of “dir” attribute for the “bdo” element must not be “auto”.",
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
        let mut rule = BdoDir::default();
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink(Vec::new());

        rule.on_event(
            &ParseEvent::EndTag {
                name: "bdo".to_string(),
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
    fn xhtml_bdo_with_dir_value_is_ok() {
        let mut rule = BdoDir::default();
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        let mut sink = Sink(Vec::new());

        rule.on_event(
            &ParseEvent::StartTag {
                name: "bdo".to_string(),
                attrs: vec![html_inspector_core::Attribute {
                    name: "dir".to_string(),
                    value: Some("ltr".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            &mut ctx,
            &mut sink,
        );

        assert!(sink.0.is_empty());
    }

    #[test]
    fn xhtml_bdo_with_dir_without_value_is_ok() {
        let mut rule = BdoDir::default();
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        let mut sink = Sink(Vec::new());

        rule.on_event(
            &ParseEvent::StartTag {
                name: "bdo".to_string(),
                attrs: vec![html_inspector_core::Attribute {
                    name: "dir".to_string(),
                    value: None,
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            &mut ctx,
            &mut sink,
        );

        assert!(sink.0.is_empty());
    }
}
