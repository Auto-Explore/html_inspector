use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct MainConstraints {
    visible_main_count: u32,
}

impl Rule for MainConstraints {
    fn id(&self) -> &'static str {
        "html.main.not_in_sectioning"
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
        if !ctx.name_is(name, "main") {
            return;
        }

        let hidden = ctx.has_attr(attrs, "hidden");

        if !hidden {
            self.visible_main_count += 1;
            if self.visible_main_count > 1 {
                out.push(Message::new(
                    "html.main.visible.multiple",
                    Severity::Error,
                    Category::Html,
                    "A document must not include more than one visible “main” element.",
                    *span,
                ));
            }
        }

        for ancestor in ["article", "aside", "footer", "header", "nav"] {
            if ctx.has_ancestor(ancestor) {
                out.push(Message::new(
                    "html.main.disallowed_descendant",
                    Severity::Error,
                    Category::Html,
                    format!(
                        "The “main” element must not appear as a descendant of the “{ancestor}” element."
                    ),
                    *span,
                ));
                break;
            }
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
    fn visible_main_elements_are_counted_in_xhtml() {
        let mut rule = MainConstraints::default();
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        let mut sink = Sink(Vec::new());

        for _ in 0..2 {
            rule.on_event(
                &ParseEvent::StartTag {
                    name: "main".to_string(),
                    attrs: vec![html_inspector_core::Attribute {
                        name: "id".to_string(),
                        value: Some("x".to_string()),
                        span: None,
                    }],
                    self_closing: false,
                    span: None,
                },
                &mut ctx,
                &mut sink,
            );
        }

        assert!(sink
            .0
            .iter()
            .any(|m| m.code == "html.main.visible.multiple"));
    }

    #[test]
    fn hidden_main_does_not_count_in_html_case_insensitively() {
        let mut rule = MainConstraints::default();
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink(Vec::new());

        rule.on_event(
            &ParseEvent::StartTag {
                name: "MAIN".to_string(),
                attrs: vec![html_inspector_core::Attribute {
                    name: "HIDDEN".to_string(),
                    value: None,
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            &mut ctx,
            &mut sink,
        );
        rule.on_event(
            &ParseEvent::StartTag {
                name: "main".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            &mut ctx,
            &mut sink,
        );

        assert!(!sink
            .0
            .iter()
            .any(|m| m.code == "html.main.visible.multiple"));
    }

    #[test]
    fn hidden_main_counts_in_xhtml_case_sensitively() {
        let mut rule = MainConstraints::default();
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        let mut sink = Sink(Vec::new());

        // Uppercase attribute names are not treated as `hidden` in XHTML.
        for _ in 0..2 {
            rule.on_event(
                &ParseEvent::StartTag {
                    name: "main".to_string(),
                    attrs: vec![html_inspector_core::Attribute {
                        name: "HIDDEN".to_string(),
                        value: None,
                        span: None,
                    }],
                    self_closing: false,
                    span: None,
                },
                &mut ctx,
                &mut sink,
            );
        }

        assert!(sink
            .0
            .iter()
            .any(|m| m.code == "html.main.visible.multiple"));
    }
}
