use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct AutofocusConstraints {
    root_stack: Vec<Root>,
}

#[derive(Clone, Debug)]
struct Root {
    tag: String,
    autofocus_count: u32,
}

impl Rule for AutofocusConstraints {
    fn id(&self) -> &'static str {
        "html.autofocus.constraints"
    }

    fn interest(&self) -> Interest {
        Interest::START_TAG | Interest::END_TAG
    }

    fn init(&mut self, _ctx: &ValidationContext) {
        self.root_stack.clear();
        self.root_stack.push(Root {
            tag: "#document".to_string(),
            autofocus_count: 0,
        });
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
                span,
                self_closing,
                ..
            } => {
                if is_scoping_root(ctx, name, attrs) {
                    self.root_stack.push(Root {
                        tag: name.to_string(),
                        autofocus_count: 0,
                    });
                }

                let autofocus = attrs.iter().any(|a| match ctx.format {
                    html_inspector_core::InputFormat::Html => {
                        a.name.eq_ignore_ascii_case("autofocus")
                    }
                    html_inspector_core::InputFormat::Xhtml => a.name == "autofocus",
                });
                if autofocus
                    && let Some(root) = self.root_stack.last_mut() {
                        root.autofocus_count += 1;
                        if root.autofocus_count > 1 {
                            out.push(Message::new(
                                "html.autofocus.multiple_in_scoping_root",
                                Severity::Error,
                                Category::Html,
                                "There must not be two elements with the same \"nearest ancestor autofocus scoping root element\" that both have the “autofocus” attribute specified.",
                                *span,
                            ));
                        }
                    }

                if *self_closing && is_scoping_root(ctx, name, attrs) && self.root_stack.len() > 1 {
                    self.root_stack.pop();
                }
            }
            ParseEvent::EndTag { name, .. } => {
                if let Some(top) = self.root_stack.last()
                    && matches_name(ctx, &top.tag, name) && self.root_stack.len() > 1 {
                        self.root_stack.pop();
                    }
            }
            _ => {}
        }
    }
}

fn is_scoping_root(
    ctx: &ValidationContext,
    name: &str,
    attrs: &[html_inspector_core::Attribute],
) -> bool {
    if matches_name(ctx, name, "dialog") {
        return true;
    }
    // Treat popovers as autofocus scoping roots (sufficient for the current suite cases).
    attrs.iter().any(|a| match ctx.format {
        html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case("popover"),
        html_inspector_core::InputFormat::Xhtml => a.name == "popover",
    })
}

fn matches_name(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    match ctx.format {
        html_inspector_core::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector_core::InputFormat::Xhtml => actual == expected,
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
    fn multiple_autofocus_in_scoping_root_emits_error() {
        let mut rule = AutofocusConstraints::default();
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink(Vec::new());
        rule.init(&ctx);

        rule.on_event(
            &ParseEvent::StartTag {
                name: "dialog".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            &mut ctx,
            &mut sink,
        );

        for name in ["input", "button"] {
            rule.on_event(
                &ParseEvent::StartTag {
                    name: name.to_string(),
                    attrs: vec![html_inspector_core::Attribute {
                        name: "autofocus".to_string(),
                        value: None,
                        span: None,
                    }],
                    self_closing: true,
                    span: None,
                },
                &mut ctx,
                &mut sink,
            );
        }

        assert!(sink
            .0
            .iter()
            .any(|m| m.code == "html.autofocus.multiple_in_scoping_root"));
    }

    #[test]
    fn self_closing_scoping_root_is_popped() {
        let mut rule = AutofocusConstraints::default();
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink(Vec::new());
        rule.init(&ctx);

        rule.on_event(
            &ParseEvent::StartTag {
                name: "dialog".to_string(),
                attrs: vec![html_inspector_core::Attribute {
                    name: "autofocus".to_string(),
                    value: None,
                    span: None,
                }],
                self_closing: true,
                span: None,
            },
            &mut ctx,
            &mut sink,
        );

        assert_eq!(rule.root_stack.len(), 1);
    }

    #[test]
    fn matching_end_tag_pops_scoping_root() {
        let mut rule = AutofocusConstraints::default();
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink(Vec::new());
        rule.init(&ctx);

        rule.on_event(
            &ParseEvent::StartTag {
                name: "dialog".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            &mut ctx,
            &mut sink,
        );
        assert_eq!(rule.root_stack.len(), 2);

        rule.on_event(
            &ParseEvent::EndTag {
                name: "div".to_string(),
                span: None,
            },
            &mut ctx,
            &mut sink,
        );
        assert_eq!(rule.root_stack.len(), 2);

        rule.on_event(
            &ParseEvent::EndTag {
                name: "dialog".to_string(),
                span: None,
            },
            &mut ctx,
            &mut sink,
        );
        assert_eq!(rule.root_stack.len(), 1);
    }

    #[test]
    fn ignores_non_tag_events() {
        let mut rule = AutofocusConstraints::default();
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink(Vec::new());
        rule.init(&ctx);

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
