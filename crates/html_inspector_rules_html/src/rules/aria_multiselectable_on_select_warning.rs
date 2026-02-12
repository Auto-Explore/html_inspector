use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct AriaMultiselectableOnSelectWarning;

impl Rule for AriaMultiselectableOnSelectWarning {
    fn id(&self) -> &'static str {
        "html.aria.multiselectable_on_select"
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
        if !is(ctx, name, "select") {
            return;
        }
        if !has_attr(ctx, attrs, "aria-multiselectable") {
            return;
        }

        out.push(Message::new(
            "html.aria.multiselectable_on_select",
            Severity::Warning,
            Category::Html,
            "The “aria-multiselectable” attribute should not be used with the “select ” element.",
            *span,
        ));
    }
}

fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    match ctx.format {
        html_inspector_core::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector_core::InputFormat::Xhtml => actual == expected,
    }
}

fn has_attr(
    ctx: &ValidationContext,
    attrs: &[html_inspector_core::Attribute],
    needle: &str,
) -> bool {
    attrs.iter().any(|a| match ctx.format {
        html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case(needle),
        html_inspector_core::InputFormat::Xhtml => a.name == needle,
    })
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
        let mut rule = AriaMultiselectableOnSelectWarning::default();
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
    fn xhtml_attr_matching_finds_aria_multiselectable() {
        let mut rule = AriaMultiselectableOnSelectWarning::default();
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        let mut sink = Sink(Vec::new());

        rule.on_event(
            &ParseEvent::StartTag {
                name: "select".to_string(),
                attrs: vec![html_inspector_core::Attribute {
                    name: "aria-multiselectable".to_string(),
                    value: Some("true".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            &mut ctx,
            &mut sink,
        );

        assert!(sink
            .0
            .iter()
            .any(|m| m.code == "html.aria.multiselectable_on_select"));
    }
}
