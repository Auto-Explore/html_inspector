use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

use super::url_attr;

#[derive(Default)]
pub struct VideoSrcConstraints;

impl Rule for VideoSrcConstraints {
    fn id(&self) -> &'static str {
        "html.video.src.datatype"
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
        if !ctx.name_is(name, "video") {
            return;
        }

        let src = ctx.attr_value(attrs, "src");
        let Some(src) = src else { return };

        if src.is_empty() {
            out.push(Message::new(
                "html.video.src.empty",
                Severity::Error,
                Category::Html,
                "Bad value “” for attribute “src” on element “video”.",
                *span,
            ));
            return;
        }

        let _ = url_attr::validate_url_attr_value(
            src,
            "src",
            "video",
            "html.video.src.invalid",
            *span,
            out,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use html_inspector_core::{Config, InputFormat, RuleSet};
    use html_inspector_html::HtmlEventSource;

    #[test]
    fn video_src_empty_emits_error() {
        let src =
            HtmlEventSource::from_str("t", InputFormat::Html, "<video src=\"\"></video>").unwrap();
        let rules = RuleSet::new().push(VideoSrcConstraints::default());
        let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.video.src.empty")
        );
    }

    #[test]
    fn html_src_attribute_name_matching_is_case_insensitive() {
        struct Sink(Vec<html_inspector_core::Message>);
        impl html_inspector_core::MessageSink for Sink {
            fn push(&mut self, msg: html_inspector_core::Message) {
                self.0.push(msg);
            }
        }

        let mut ctx =
            html_inspector_core::ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink(Vec::new());
        let mut rule = VideoSrcConstraints::default();
        rule.on_event(
            &ParseEvent::StartTag {
                name: "video".to_string(),
                attrs: vec![html_inspector_core::Attribute {
                    name: "SRC".to_string(),
                    value: Some("".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            &mut ctx,
            &mut sink,
        );

        assert!(sink.0.iter().any(|m| m.code == "html.video.src.empty"));
    }

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
        let mut rule = VideoSrcConstraints::default();
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
    fn video_src_forbidden_code_point_emits_url_error() {
        let src = HtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<video src=\"https://example.com/\u{FDD0}\"></video>",
        )
        .unwrap();
        let rules = RuleSet::new().push(VideoSrcConstraints::default());
        let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.url.forbidden_code_point")
        );
    }

    #[test]
    fn xhtml_invalid_src_emits_error() {
        struct Sink(Vec<html_inspector_core::Message>);
        impl html_inspector_core::MessageSink for Sink {
            fn push(&mut self, msg: html_inspector_core::Message) {
                self.0.push(msg);
            }
        }

        let mut ctx =
            html_inspector_core::ValidationContext::new(Config::default(), InputFormat::Xhtml);
        let mut sink = Sink(Vec::new());
        let mut rule = VideoSrcConstraints::default();
        rule.on_event(
            &ParseEvent::StartTag {
                name: "video".to_string(),
                attrs: vec![html_inspector_core::Attribute {
                    name: "src".to_string(),
                    value: Some("http:example.com".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            &mut ctx,
            &mut sink,
        );

        assert!(sink.0.iter().any(|m| m.code == "html.video.src.invalid"));
    }

    #[test]
    fn xhtml_src_attribute_name_matching_is_case_sensitive() {
        struct Sink(Vec<html_inspector_core::Message>);
        impl html_inspector_core::MessageSink for Sink {
            fn push(&mut self, msg: html_inspector_core::Message) {
                self.0.push(msg);
            }
        }

        let mut ctx =
            html_inspector_core::ValidationContext::new(Config::default(), InputFormat::Xhtml);
        let mut sink = Sink(Vec::new());
        let mut rule = VideoSrcConstraints::default();
        rule.on_event(
            &ParseEvent::StartTag {
                name: "video".to_string(),
                attrs: vec![html_inspector_core::Attribute {
                    name: "SRC".to_string(),
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
    }
}
