use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct SvgImageSrcsetConstraints;

impl Rule for SvgImageSrcsetConstraints {
    fn id(&self) -> &'static str {
        "html.svg.image.srcset_constraints"
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

        // In foreign content, <image> is an SVG element; the HTML <image> tokenization parse error
        // is handled separately by the tokenizer. Here we only apply constraints for SVG descendants.
        if !ctx.has_ancestor("svg") {
            return;
        }
        if !is(ctx, name, "image") {
            return;
        }
        if !has_attr(ctx, attrs, "srcset") {
            return;
        }

        out.push(Message::new(
            "html.svg.image.srcset.disallowed",
            Severity::Error,
            Category::Html,
            "Attribute “srcset” not allowed on element “image” at this point.",
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
        let mut rule = SvgImageSrcsetConstraints::default();
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink(Vec::new());

        rule.on_event(
            &ParseEvent::EndTag {
                name: "image".to_string(),
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
    fn helper_predicates_are_case_sensitive_in_xhtml() {
        let ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        assert!(is(&ctx, "image", "image"));
        assert!(!is(&ctx, "Image", "image"));

        let attrs = vec![html_inspector_core::Attribute {
            name: "srcset".to_string(),
            value: None,
            span: None,
        }];
        assert!(has_attr(&ctx, &attrs, "srcset"));
        assert!(!has_attr(&ctx, &attrs, "SRCSET"));
    }
}
