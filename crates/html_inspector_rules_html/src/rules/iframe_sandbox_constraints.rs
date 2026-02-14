use std::borrow::Cow;

use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};
use rustc_hash::FxHashSet;

const ALLOWED_SANDBOX_TOKENS: [&str; 12] = [
    "allow-downloads",
    "allow-forms",
    "allow-modals",
    "allow-orientation-lock",
    "allow-pointer-lock",
    "allow-popups",
    "allow-popups-to-escape-sandbox",
    "allow-presentation",
    "allow-same-origin",
    "allow-scripts",
    "allow-top-navigation",
    "allow-top-navigation-by-user-activation",
];

#[derive(Default)]
pub struct IFrameSandboxConstraints;

impl Rule for IFrameSandboxConstraints {
    fn id(&self) -> &'static str {
        "html.iframe.sandbox"
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
        if !ctx.name_is(name, "iframe") {
            return;
        }

        let Some(sandbox) = ctx.attr_value(attrs, "sandbox") else {
            return;
        };

        let mut seen: FxHashSet<Cow<'_, str>> =
            FxHashSet::with_capacity_and_hasher(ALLOWED_SANDBOX_TOKENS.len(), Default::default());
        let mut has_scripts = false;
        let mut has_same_origin = false;

        for token in sandbox.split_ascii_whitespace() {
            let token_lc = html_inspector_core::ascii_lowercase_cow_if_needed(token);
            let token_lc_str = token_lc.as_ref();
            if !ALLOWED_SANDBOX_TOKENS.contains(&token_lc_str) {
                out.push(Message::new(
                    "html.iframe.sandbox.invalid_token",
                    Severity::Error,
                    Category::Html,
                    format!("Bad value “{sandbox}” for attribute “sandbox” on element “iframe”."),
                    *span,
                ));
                return;
            }
            let is_scripts = token_lc_str == "allow-scripts";
            let is_same_origin = token_lc_str == "allow-same-origin";
            if !seen.insert(token_lc) {
                out.push(Message::new(
                    "html.iframe.sandbox.duplicate_token",
                    Severity::Error,
                    Category::Html,
                    format!("Bad value “{sandbox}” for attribute “sandbox” on element “iframe”."),
                    *span,
                ));
                return;
            }
            has_scripts |= is_scripts;
            has_same_origin |= is_same_origin;
        }

        if has_scripts && has_same_origin {
            out.push(Message::new(
                "html.iframe.sandbox.scripts_and_same_origin",
                Severity::Warning,
                Category::Html,
                format!("Bad value “{sandbox}” for attribute “sandbox” on element “iframe”."),
                *span,
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use html_inspector_core::{Attribute, Config, InputFormat, RuleSet};
    use html_inspector_html::HtmlEventSource;

    #[test]
    fn warns_on_scripts_and_same_origin_case_insensitively() {
        let src = HtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            r#"<iframe sandbox="ALLOW-SCRIPTS allow-same-origin"></iframe>"#,
        )
        .unwrap();
        let rules = RuleSet::new().push(IFrameSandboxConstraints::default());
        let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
        assert!(report
            .messages
            .iter()
            .any(|m| m.code == "html.iframe.sandbox.scripts_and_same_origin"));
    }

    #[test]
    fn errors_on_duplicate_tokens_case_insensitively() {
        let src = HtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            r#"<iframe sandbox="allow-scripts ALLOW-SCRIPTS"></iframe>"#,
        )
        .unwrap();
        let rules = RuleSet::new().push(IFrameSandboxConstraints::default());
        let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
        assert!(report
            .messages
            .iter()
            .any(|m| m.code == "html.iframe.sandbox.duplicate_token"));
    }

    #[test]
    fn xhtml_attribute_name_matching_is_case_sensitive() {
        struct Sink(Vec<Message>);
        impl MessageSink for Sink {
            fn push(&mut self, msg: Message) {
                self.0.push(msg);
            }
        }

        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        let mut sink = Sink(Vec::new());
        let mut rule = IFrameSandboxConstraints::default();
        rule.on_event(
            &ParseEvent::StartTag {
                name: "iframe".to_string(),
                attrs: vec![Attribute {
                    name: "SANDBOX".to_string(),
                    value: Some("allow-scripts allow-same-origin".to_string()),
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
