use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

use super::shared::starts_with_ascii_ci;

#[derive(Default)]
pub struct ImgAltRequired;

impl Rule for ImgAltRequired {
    fn id(&self) -> &'static str {
        "html.img.alt.required"
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
        let is_img = match ctx.format {
            html_inspector_core::InputFormat::Html => name.eq_ignore_ascii_case("img"),
            html_inspector_core::InputFormat::Xhtml => name == "img",
        };
        if !is_img {
            return;
        }

        let has_alt = ctx.has_attr(attrs, "alt");
        if !has_alt {
            if has_aria_attrs_other_than_hidden(ctx, attrs) && !has_accessible_name(ctx, attrs) {
                out.push(Message::new(
                    "html.img.aria.accessible_name.missing",
                    Severity::Error,
                    Category::Html,
                    "An “img” element with any “aria-*” attributes other than “aria-hidden” must also have an accessible name. (e.g., an “alt” attribute).",
                    *span,
                ));
            } else {
                out.push(Message::new(
                    "html.img.alt.missing",
                    Severity::Error,
                    Category::Html,
                    "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
                    *span,
                ));
            }
        }
    }
}

fn has_accessible_name(ctx: &ValidationContext, attrs: &[html_inspector_core::Attribute]) -> bool {
    if ctx
        .attr_value(attrs, "alt")
        .is_some_and(|alt| !alt.is_empty())
    {
        return true;
    }
    ctx.attr_value(attrs, "aria-label")
        .is_some_and(|v| !v.trim().is_empty())
        || ctx
            .attr_value(attrs, "aria-labelledby")
            .is_some_and(|v| !v.trim().is_empty())
}

fn has_aria_attrs_other_than_hidden(
    ctx: &ValidationContext,
    attrs: &[html_inspector_core::Attribute],
) -> bool {
    attrs.iter().any(|a| match ctx.format {
        html_inspector_core::InputFormat::Html => {
            starts_with_ascii_ci(&a.name, "aria-") && !a.name.eq_ignore_ascii_case("aria-hidden")
        }
        html_inspector_core::InputFormat::Xhtml => {
            a.name.starts_with("aria-") && a.name != "aria-hidden"
        }
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
        let mut rule = ImgAltRequired::default();
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
                html_inspector_core::Category::Html,
                "x".to_string(),
                None,
            ),
        );
        assert_eq!(sink.0.len(), 1);
    }

    #[test]
    fn has_accessible_name_returns_true_for_non_empty_alt() {
        let ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let attrs = vec![html_inspector_core::Attribute {
            name: "alt".to_string(),
            value: Some("ok".to_string()),
            span: None,
        }];
        assert!(has_accessible_name(&ctx, &attrs));
    }

    #[test]
    fn has_accessible_name_with_empty_alt_falls_through_to_aria_label() {
        let ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let attrs = vec![
            html_inspector_core::Attribute {
                name: "alt".to_string(),
                value: Some("".to_string()),
                span: None,
            },
            html_inspector_core::Attribute {
                name: "aria-label".to_string(),
                value: Some("name".to_string()),
                span: None,
            },
        ];
        assert!(has_accessible_name(&ctx, &attrs));
    }

    #[test]
    fn attr_value_is_case_sensitive_in_xhtml() {
        let ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        let attrs = vec![
            html_inspector_core::Attribute {
                name: "alt".to_string(),
                value: Some("x".to_string()),
                span: None,
            },
            html_inspector_core::Attribute {
                name: "ALT".to_string(),
                value: Some("y".to_string()),
                span: None,
            },
        ];
        assert_eq!(ctx.attr_value(&attrs, "alt"), Some("x"));
        assert_eq!(ctx.attr_value(&attrs, "ALT"), Some("y"));
        assert_eq!(ctx.attr_value(&attrs, "Alt"), None);
    }
}
