use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, ValidationContext,
};

use super::url_attr;

#[derive(Default)]
pub struct AreaHrefConstraints;

impl Rule for AreaHrefConstraints {
    fn id(&self) -> &'static str {
        "html.area.href.datatype"
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
        if !ctx.name_is(name, "area") {
            return;
        }

        let href = ctx.attr_value(attrs, "href");

        let Some(href) = href else {
            // VNU suite: an <area> without href but with a non-empty alt triggers a required-href error.
            let alt = ctx.attr_value(attrs, "alt");
            if alt.is_some_and(|v| !v.is_empty()) {
                out.push(Message::new(
                    "html.area.href.missing",
                    html_inspector_core::Severity::Error,
                    Category::Html,
                    "Element “area” is missing required attribute “href”.",
                    *span,
                ));
            }
            return;
        };

        let _ = url_attr::validate_url_attr_value(
            href,
            "href",
            "area",
            "html.area.href.invalid",
            *span,
            out,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct Sink(Vec<Message>);
    impl MessageSink for Sink {
        fn push(&mut self, msg: Message) {
            self.0.push(msg);
        }
    }

    #[test]
    fn missing_href_with_nonempty_alt_is_error() {
        let mut ctx = ValidationContext::new(
            html_inspector_core::Config::default(),
            html_inspector_core::InputFormat::Html,
        );
        let mut sink = Sink::default();
        let mut rule = AreaHrefConstraints::default();

        rule.on_event(
            &ParseEvent::StartTag {
                name: "area".to_string(),
                attrs: vec![html_inspector_core::Attribute {
                    name: "alt".to_string(),
                    value: Some("x".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            &mut ctx,
            &mut sink,
        );
        assert!(sink.0.iter().any(|m| m.code == "html.area.href.missing"));
    }

    #[test]
    fn missing_href_with_empty_or_missing_alt_is_ok() {
        let mut ctx = ValidationContext::new(
            html_inspector_core::Config::default(),
            html_inspector_core::InputFormat::Html,
        );
        let mut rule = AreaHrefConstraints::default();

        let mut sink = Sink::default();
        rule.on_event(
            &ParseEvent::StartTag {
                name: "area".to_string(),
                attrs: vec![html_inspector_core::Attribute {
                    name: "alt".to_string(),
                    value: Some("".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            &mut ctx,
            &mut sink,
        );
        assert!(sink.0.is_empty());

        let mut sink = Sink::default();
        rule.on_event(
            &ParseEvent::StartTag {
                name: "area".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            &mut ctx,
            &mut sink,
        );
        assert!(sink.0.is_empty());
    }
}
