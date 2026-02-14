use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

use super::csp;
use super::csp::{CspExternalKind, CspKind, CspPolicy};

#[derive(Default)]
pub struct ContentSecurityPolicyWarnings {
    meta_policies: Vec<CspPolicy>,
    http_policies: Vec<CspPolicy>,
    collecting_inline_script: bool,
    inline_script_content: String,
    inline_script_span: Option<html_inspector_core::Span>,
    inline_script_nonce: Option<String>,
    collecting_inline_style: bool,
    inline_style_content: String,
    inline_style_span: Option<html_inspector_core::Span>,
    inline_style_nonce: Option<String>,
}

impl Rule for ContentSecurityPolicyWarnings {
    fn id(&self) -> &'static str {
        "html.csp.warnings"
    }

    fn max_severity(&self) -> Severity {
        Severity::Warning
    }

    fn interest(&self) -> Interest {
        Interest::START_TAG | Interest::TEXT | Interest::END_TAG
    }

    fn init(&mut self, ctx: &ValidationContext) {
        self.meta_policies.clear();
        self.http_policies.clear();
        self.collecting_inline_script = false;
        self.inline_script_content.clear();
        self.inline_script_span = None;
        self.inline_script_nonce = None;
        self.collecting_inline_style = false;
        self.inline_style_content.clear();
        self.inline_style_span = None;
        self.inline_style_nonce = None;
        if let Some(v) = ctx.config.csp_header.as_deref()
            && !v.trim().is_empty()
        {
            self.http_policies = csp::parse_csp_policies(v);
        }
    }

    fn on_event(
        &mut self,
        event: &ParseEvent,
        ctx: &mut ValidationContext,
        out: &mut dyn MessageSink,
    ) {
        let (name, attrs, span) = match event {
            ParseEvent::StartTag {
                name, attrs, span, ..
            } => (name, attrs, span),
            ParseEvent::Text { text, .. } => {
                if self.collecting_inline_script {
                    self.inline_script_content.push_str(text);
                }
                if self.collecting_inline_style {
                    self.inline_style_content.push_str(text);
                }
                return;
            }
            ParseEvent::EndTag { name, .. } => {
                if self.collecting_inline_script && ctx.name_is(name, "script") {
                    self.finish_inline_script(out);
                }
                if self.collecting_inline_style && ctx.name_is(name, "style") {
                    self.finish_inline_style(out);
                }
                return;
            }
            _ => return,
        };

        if ctx.name_is(name, "meta") {
            let http_equiv = ctx.attr_value(attrs, "http-equiv").unwrap_or("");
            if !http_equiv.eq_ignore_ascii_case("content-security-policy") {
                return;
            }
            let content = ctx.attr_value(attrs, "content").unwrap_or("");
            if content.trim().is_empty() {
                return;
            }
            self.meta_policies.extend(csp::parse_csp_policies(content));
            return;
        }

        if self.meta_policies.is_empty() && self.http_policies.is_empty() {
            return;
        }

        for a in attrs
            .iter()
            .filter(|a| csp::is_event_handler_attr(ctx, &a.name))
        {
            let Some(value) = a.value.as_deref() else {
                continue;
            };
            if value.is_empty() {
                continue;
            }
            if let Some((source, violating_directive)) =
                self.first_blocking(CspKind::ScriptEventHandler, None, Some(value))
            {
                out.push(Message::new(
                    "html.csp.event_handler.blocked",
                    Severity::Warning,
                    Category::Html,
                    format!(
                        "Event handler attribute “{}” violates Content Security Policy ({source}): blocked by “{}” directive.",
                        a.name, violating_directive,
                    ),
                    *span,
                ));
            }
        }

        if ctx.name_is(name, "script") && !ctx.has_attr(attrs, "src") {
            self.collecting_inline_script = true;
            self.inline_script_content.clear();
            self.inline_script_span = *span;
            self.inline_script_nonce = ctx.attr_value(attrs, "nonce").map(str::to_owned);
        }

        if ctx.name_is(name, "style") {
            self.collecting_inline_style = true;
            self.inline_style_content.clear();
            self.inline_style_span = *span;
            self.inline_style_nonce = ctx.attr_value(attrs, "nonce").map(str::to_owned);
        }

        if let Some(style_value) = ctx.attr_value(attrs, "style")
            && !style_value.is_empty()
            && let Some((source, violating_directive)) =
                self.first_blocking(CspKind::StyleAttribute, None, Some(style_value))
        {
            out.push(Message::new(
                        "html.csp.style_attribute.blocked",
                        Severity::Warning,
                        Category::Html,
                        format!(
                            "The “style” attribute violates Content Security Policy ({source}): blocked by “{}” directive.",
                            violating_directive
                        ),
                        *span,
                    ));
        }

        if ctx.name_is(name, "script")
            && ctx.has_attr(attrs, "src")
            && let Some(src) = ctx
                .attr_value(attrs, "src")
                .map(str::trim)
                .filter(|s| !s.is_empty())
            && let Some((source, violating_directive)) = self.first_blocking_external(
                CspExternalKind::Script,
                src,
                ctx.config.base_uri.as_deref(),
            )
        {
            out.push(Message::new(
                        "html.csp.external_script.blocked",
                        Severity::Warning,
                        Category::Html,
                        format!(
                            "Resource violates Content Security Policy ({source}): external script “{}” blocked by “{}” directive.",
                            csp::truncate(src, 50),
                            violating_directive
                        ),
                        *span,
                    ));
        }

        if ctx.name_is(name, "link") {
            let rel = ctx.attr_value(attrs, "rel").unwrap_or("");
            if rel
                .split_ascii_whitespace()
                .any(|t| t.eq_ignore_ascii_case("stylesheet"))
                && let Some(href) = ctx
                    .attr_value(attrs, "href")
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                && let Some((source, violating_directive)) = self.first_blocking_external(
                    CspExternalKind::Style,
                    href,
                    ctx.config.base_uri.as_deref(),
                )
            {
                out.push(Message::new(
                            "html.csp.external_style.blocked",
                            Severity::Warning,
                            Category::Html,
                            format!(
                                "Resource violates Content Security Policy ({source}): external stylesheet “{}” blocked by “{}” directive.",
                                csp::truncate(href, 50),
                                violating_directive
                            ),
                            *span,
                        ));
            }
        }

        if ctx.name_is(name, "img")
            && let Some(src) = ctx
                .attr_value(attrs, "src")
                .map(str::trim)
                .filter(|s| !s.is_empty())
            && let Some((source, violating_directive)) = self.first_blocking_external(
                CspExternalKind::Image,
                src,
                ctx.config.base_uri.as_deref(),
            )
        {
            out.push(Message::new(
                        "html.csp.image.blocked",
                        Severity::Warning,
                        Category::Html,
                        format!(
                            "Resource violates Content Security Policy ({source}): image “{}” blocked by “{}” directive.",
                            csp::truncate(src, 50),
                            violating_directive
                        ),
                        *span,
                    ));
        }

        if ctx.name_is(name, "iframe")
            && let Some(src) = ctx
                .attr_value(attrs, "src")
                .map(str::trim)
                .filter(|s| !s.is_empty())
            && let Some((source, violating_directive)) = self.first_blocking_external(
                CspExternalKind::Frame,
                src,
                ctx.config.base_uri.as_deref(),
            )
        {
            out.push(Message::new(
                        "html.csp.frame.blocked",
                        Severity::Warning,
                        Category::Html,
                        format!(
                            "Resource violates Content Security Policy ({source}): frame “{}” blocked by “{}” directive.",
                            csp::truncate(src, 50),
                            violating_directive
                        ),
                        *span,
                    ));
        }

        if ctx.name_is(name, "object")
            && let Some(data) = ctx
                .attr_value(attrs, "data")
                .map(str::trim)
                .filter(|s| !s.is_empty())
            && let Some((source, violating_directive)) = self.first_blocking_external(
                CspExternalKind::Object,
                data,
                ctx.config.base_uri.as_deref(),
            )
        {
            out.push(Message::new(
                        "html.csp.object.blocked",
                        Severity::Warning,
                        Category::Html,
                        format!(
                            "Resource violates Content Security Policy ({source}): object/embed “{}” blocked by “{}” directive.",
                            csp::truncate(data, 50),
                            violating_directive
                        ),
                        *span,
                    ));
        }

        if ctx.name_is(name, "embed")
            && let Some(src) = ctx
                .attr_value(attrs, "src")
                .map(str::trim)
                .filter(|s| !s.is_empty())
            && let Some((source, violating_directive)) = self.first_blocking_external(
                CspExternalKind::Object,
                src,
                ctx.config.base_uri.as_deref(),
            )
        {
            out.push(Message::new(
                        "html.csp.object.blocked",
                        Severity::Warning,
                        Category::Html,
                        format!(
                            "Resource violates Content Security Policy ({source}): object/embed “{}” blocked by “{}” directive.",
                            csp::truncate(src, 50),
                            violating_directive
                        ),
                        *span,
                    ));
        }

        if (ctx.name_is(name, "audio") || ctx.name_is(name, "video"))
            && let Some(src) = ctx
                .attr_value(attrs, "src")
                .map(str::trim)
                .filter(|s| !s.is_empty())
            && let Some((source, violating_directive)) = self.first_blocking_external(
                CspExternalKind::Media,
                src,
                ctx.config.base_uri.as_deref(),
            )
        {
            out.push(Message::new(
                        "html.csp.media.blocked",
                        Severity::Warning,
                        Category::Html,
                        format!(
                            "Resource violates Content Security Policy ({source}): media “{}” blocked by “{}” directive.",
                            csp::truncate(src, 50),
                            violating_directive
                        ),
                        *span,
                    ));
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, out: &mut dyn MessageSink) {
        if self.collecting_inline_script {
            self.finish_inline_script(out);
        }
        if self.collecting_inline_style {
            self.finish_inline_style(out);
        }
        self.meta_policies.clear();
        self.http_policies.clear();
    }
}

impl ContentSecurityPolicyWarnings {
    fn finish_inline_script(&mut self, out: &mut dyn MessageSink) {
        let content = self.inline_script_content.trim();
        if !content.is_empty() {
            let nonce = self.inline_script_nonce.as_deref();
            if let Some((source, violating_directive)) =
                self.first_blocking(CspKind::InlineScript, nonce, Some(content))
            {
                out.push(Message::new(
                    "html.csp.inline_script.blocked",
                    Severity::Warning,
                    Category::Html,
                    format!(
                        "Inline script violates Content Security Policy ({source}): blocked by “{}” directive (missing “‘unsafe-inline’” or nonce/hash).",
                        violating_directive
                    ),
                    self.inline_script_span,
                ));
            }
        }

        self.collecting_inline_script = false;
        self.inline_script_content.clear();
        self.inline_script_span = None;
        self.inline_script_nonce = None;
    }

    fn finish_inline_style(&mut self, out: &mut dyn MessageSink) {
        let content = self.inline_style_content.trim();
        if !content.is_empty() {
            let nonce = self.inline_style_nonce.as_deref();
            if let Some((source, violating_directive)) =
                self.first_blocking(CspKind::InlineStyle, nonce, Some(content))
            {
                out.push(Message::new(
                    "html.csp.inline_style.blocked",
                    Severity::Warning,
                    Category::Html,
                    format!(
                        "Inline style violates Content Security Policy ({source}): blocked by “{}” directive (missing “‘unsafe-inline’” or nonce/hash).",
                        violating_directive
                    ),
                    self.inline_style_span,
                ));
            }
        }

        self.collecting_inline_style = false;
        self.inline_style_content.clear();
        self.inline_style_span = None;
        self.inline_style_nonce = None;
    }

    fn first_blocking<'a>(
        &'a self,
        kind: CspKind,
        nonce: Option<&'a str>,
        content: Option<&'a str>,
    ) -> Option<(&'static str, &'a str)> {
        if let Some(d) = self
            .http_policies
            .iter()
            .filter_map(|p| p.blocking_directive(kind, nonce, content))
            .next()
        {
            return Some(("HTTP header", d));
        }
        self.meta_policies
            .iter()
            .filter_map(|p| p.blocking_directive(kind, nonce, content))
            .next()
            .map(|d| ("meta tag", d))
    }

    fn first_blocking_external<'a>(
        &'a self,
        kind: CspExternalKind,
        url: &'a str,
        base_uri: Option<&'a str>,
    ) -> Option<(&'static str, &'a str)> {
        if let Some(d) = self
            .http_policies
            .iter()
            .filter_map(|p| p.blocking_external_directive(kind, url, base_uri))
            .next()
        {
            return Some(("HTTP header", d));
        }
        self.meta_policies
            .iter()
            .filter_map(|p| p.blocking_external_directive(kind, url, base_uri))
            .next()
            .map(|d| ("meta tag", d))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use base64::Engine as _;
    use html_inspector_core::{Attribute, Config, InputFormat, Span};
    use sha2::{Digest, Sha256};

    use super::csp::{is_event_handler_attr, nonce_matches_sources, parse_csp_policies};

    #[derive(Default)]
    struct Sink(Vec<Message>);
    impl MessageSink for Sink {
        fn push(&mut self, msg: Message) {
            self.0.push(msg);
        }
    }

    fn attr(name: &str, value: Option<&str>) -> Attribute {
        Attribute {
            name: name.to_string(),
            value: value.map(str::to_owned),
            span: None,
        }
    }

    fn start_tag(name: &str, attrs: Vec<Attribute>) -> ParseEvent {
        ParseEvent::StartTag {
            name: name.to_string(),
            attrs,
            self_closing: false,
            span: Some(Span::new(0, 0, 1, 1)),
        }
    }

    fn end_tag(name: &str) -> ParseEvent {
        ParseEvent::EndTag {
            name: name.to_string(),
            span: Some(Span::new(0, 0, 1, 1)),
        }
    }

    fn text(s: &str) -> ParseEvent {
        ParseEvent::Text {
            text: s.to_string(),
            span: Some(Span::new(0, 0, 1, 1)),
        }
    }

    #[test]
    fn nonce_matches_sources_requires_non_empty_nonce_and_matching_token() {
        assert!(!nonce_matches_sources(None, &["'nonce-abc'".to_string()]));
        assert!(!nonce_matches_sources(
            Some(""),
            &["'nonce-abc'".to_string()]
        ));
        assert!(!nonce_matches_sources(
            Some("   "),
            &["'nonce-abc'".to_string()]
        ));
        assert!(!nonce_matches_sources(
            Some("abc"),
            &["'nonce-def'".to_string()]
        ));
        assert!(!nonce_matches_sources(
            Some("abc"),
            &["'nonce-abc".to_string()]
        ));
        assert!(!nonce_matches_sources(
            Some("abc"),
            &["nonce-abc'".to_string()]
        ));
        assert!(nonce_matches_sources(
            Some("abc"),
            &["  'nonce-abc'  ".to_string(), "'self'".to_string()]
        ));
    }

    #[test]
    fn is_event_handler_attr_matches_known_handler_names_with_correct_case_rules() {
        let html_ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let xhtml_ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);

        assert!(!is_event_handler_attr(&html_ctx, "on"));
        assert!(!is_event_handler_attr(&html_ctx, "one"));
        assert!(is_event_handler_attr(&html_ctx, "onclick"));
        assert!(is_event_handler_attr(&html_ctx, "OnCliCk"));

        assert!(is_event_handler_attr(&xhtml_ctx, "onclick"));
        assert!(!is_event_handler_attr(&xhtml_ctx, "OnCliCk"));
        assert!(!is_event_handler_attr(&xhtml_ctx, "one"));
    }

    #[test]
    fn parse_csp_policies_splits_policies_and_directives() {
        let policies = parse_csp_policies(
            "default-src 'self'; script-src 'self', , style-src 'unsafe-inline'",
        );
        assert_eq!(policies.len(), 2);
        assert!(policies[0].directives.contains_key("default-src"));
        assert!(policies[0].directives.contains_key("script-src"));
        assert!(policies[1].directives.contains_key("style-src"));
    }

    #[test]
    fn emits_warnings_for_blocked_event_handlers_and_inline_script() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink::default();
        let mut rule = ContentSecurityPolicyWarnings::default();

        rule.on_event(
            &start_tag(
                "meta",
                vec![
                    attr("http-equiv", Some("Content-Security-Policy")),
                    attr("content", Some("default-src 'self'; script-src 'self'")),
                ],
            ),
            &mut ctx,
            &mut sink,
        );

        rule.on_event(
            &start_tag("div", vec![attr("onclick", Some("x"))]),
            &mut ctx,
            &mut sink,
        );
        rule.on_event(&start_tag("script", vec![]), &mut ctx, &mut sink);
        rule.on_event(&text("alert(1)"), &mut ctx, &mut sink);
        rule.on_event(&end_tag("script"), &mut ctx, &mut sink);

        assert!(
            sink.0
                .iter()
                .any(|m| m.code == "html.csp.event_handler.blocked")
        );
        assert!(
            sink.0
                .iter()
                .any(|m| m.code == "html.csp.inline_script.blocked")
        );
    }

    #[test]
    fn nonce_allows_inline_script_but_not_event_handlers() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink::default();
        let mut rule = ContentSecurityPolicyWarnings::default();

        rule.on_event(
            &start_tag(
                "meta",
                vec![
                    attr("http-equiv", Some("Content-Security-Policy")),
                    attr("content", Some("script-src 'nonce-abc'")),
                ],
            ),
            &mut ctx,
            &mut sink,
        );

        rule.on_event(
            &start_tag("script", vec![attr("nonce", Some("abc"))]),
            &mut ctx,
            &mut sink,
        );
        rule.on_event(&text("alert(1)"), &mut ctx, &mut sink);
        rule.on_event(&end_tag("script"), &mut ctx, &mut sink);
        assert!(
            !sink
                .0
                .iter()
                .any(|m| m.code == "html.csp.inline_script.blocked")
        );

        rule.on_event(
            &start_tag("div", vec![attr("onclick", Some("x"))]),
            &mut ctx,
            &mut sink,
        );
        assert!(
            sink.0
                .iter()
                .any(|m| m.code == "html.csp.event_handler.blocked")
        );
    }

    #[test]
    fn unsafe_hashes_allows_event_handlers_with_matching_hash() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink::default();
        let mut rule = ContentSecurityPolicyWarnings::default();

        let handler = "alert(1)";
        let hash = base64::engine::general_purpose::STANDARD.encode(Sha256::digest(handler));

        rule.on_event(
            &start_tag(
                "meta",
                vec![
                    attr("http-equiv", Some("Content-Security-Policy")),
                    attr(
                        "content",
                        Some(&format!(
                            "script-src 'self' 'unsafe-hashes' 'sha256-{hash}'"
                        )),
                    ),
                ],
            ),
            &mut ctx,
            &mut sink,
        );

        rule.on_event(
            &start_tag("div", vec![attr("onclick", Some(handler))]),
            &mut ctx,
            &mut sink,
        );
        assert!(
            !sink
                .0
                .iter()
                .any(|m| m.code == "html.csp.event_handler.blocked")
        );
    }

    #[test]
    fn event_handler_hash_does_not_allow_without_unsafe_hashes() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink::default();
        let mut rule = ContentSecurityPolicyWarnings::default();

        let handler = "alert(1)";
        let hash = base64::engine::general_purpose::STANDARD.encode(Sha256::digest(handler));

        rule.on_event(
            &start_tag(
                "meta",
                vec![
                    attr("http-equiv", Some("Content-Security-Policy")),
                    attr("content", Some(&format!("script-src 'sha256-{hash}'"))),
                ],
            ),
            &mut ctx,
            &mut sink,
        );

        rule.on_event(
            &start_tag("div", vec![attr("onclick", Some(handler))]),
            &mut ctx,
            &mut sink,
        );
        assert!(
            sink.0
                .iter()
                .any(|m| m.code == "html.csp.event_handler.blocked")
        );
    }

    #[test]
    fn unsafe_hashes_allows_style_attribute_with_matching_hash() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink::default();
        let mut rule = ContentSecurityPolicyWarnings::default();

        let style_attr = "color:red";
        let hash = base64::engine::general_purpose::STANDARD.encode(Sha256::digest(style_attr));

        rule.on_event(
            &start_tag(
                "meta",
                vec![
                    attr("http-equiv", Some("Content-Security-Policy")),
                    attr(
                        "content",
                        Some(&format!("style-src 'self' 'unsafe-hashes' 'sha256-{hash}'")),
                    ),
                ],
            ),
            &mut ctx,
            &mut sink,
        );

        rule.on_event(
            &start_tag("div", vec![attr("style", Some(style_attr))]),
            &mut ctx,
            &mut sink,
        );
        assert!(
            !sink
                .0
                .iter()
                .any(|m| m.code == "html.csp.style_attribute.blocked")
        );
    }

    #[test]
    fn hash_allows_inline_script() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink::default();
        let mut rule = ContentSecurityPolicyWarnings::default();

        let content = "alert(1)";
        let hash = base64::engine::general_purpose::STANDARD.encode(Sha256::digest(content));

        rule.on_event(
            &start_tag(
                "meta",
                vec![
                    attr("http-equiv", Some("Content-Security-Policy")),
                    attr("content", Some(&format!("script-src 'sha256-{hash}'"))),
                ],
            ),
            &mut ctx,
            &mut sink,
        );

        rule.on_event(&start_tag("script", vec![]), &mut ctx, &mut sink);
        rule.on_event(&text(content), &mut ctx, &mut sink);
        rule.on_event(&end_tag("script"), &mut ctx, &mut sink);

        assert!(
            !sink
                .0
                .iter()
                .any(|m| m.code == "html.csp.inline_script.blocked")
        );
    }

    #[test]
    fn hash_allows_inline_style() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink::default();
        let mut rule = ContentSecurityPolicyWarnings::default();

        let content = "body{color:red}";
        let hash = base64::engine::general_purpose::STANDARD.encode(Sha256::digest(content));

        rule.on_event(
            &start_tag(
                "meta",
                vec![
                    attr("http-equiv", Some("Content-Security-Policy")),
                    attr("content", Some(&format!("style-src 'sha256-{hash}'"))),
                ],
            ),
            &mut ctx,
            &mut sink,
        );

        rule.on_event(&start_tag("style", vec![]), &mut ctx, &mut sink);
        rule.on_event(&text(content), &mut ctx, &mut sink);
        rule.on_event(&end_tag("style"), &mut ctx, &mut sink);

        assert!(
            !sink
                .0
                .iter()
                .any(|m| m.code == "html.csp.inline_style.blocked")
        );
    }

    #[test]
    fn style_attribute_is_checked_against_style_sources() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink::default();
        let mut rule = ContentSecurityPolicyWarnings::default();

        rule.on_event(
            &start_tag(
                "meta",
                vec![
                    attr("http-equiv", Some("Content-Security-Policy")),
                    attr("content", Some("default-src 'self'; style-src 'self'")),
                ],
            ),
            &mut ctx,
            &mut sink,
        );
        rule.on_event(
            &start_tag("div", vec![attr("style", Some("color: red"))]),
            &mut ctx,
            &mut sink,
        );
        assert!(
            sink.0
                .iter()
                .any(|m| m.code == "html.csp.style_attribute.blocked")
        );
    }

    #[test]
    fn xhtml_event_handler_name_matching_is_case_sensitive() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        let mut sink = Sink::default();
        let mut rule = ContentSecurityPolicyWarnings::default();

        rule.on_event(
            &start_tag(
                "meta",
                vec![
                    attr("http-equiv", Some("content-security-policy")),
                    attr("content", Some("script-src 'self'")),
                ],
            ),
            &mut ctx,
            &mut sink,
        );
        rule.on_event(
            &start_tag("div", vec![attr("OnClick", Some("x"))]),
            &mut ctx,
            &mut sink,
        );
        assert!(
            !sink
                .0
                .iter()
                .any(|m| m.code == "html.csp.event_handler.blocked")
        );
    }

    #[test]
    fn on_event_ignores_non_start_tag_events() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink::default();
        let mut rule = ContentSecurityPolicyWarnings::default();
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
    fn policy_allows_unsafe_inline_for_event_handlers_and_style_attrs() {
        let mut p = CspPolicy::default();
        p.directives.insert(
            "script-src".to_string(),
            vec!["'unsafe-inline'".to_string()],
        );
        p.directives
            .insert("style-src".to_string(), vec!["'unsafe-inline'".to_string()]);
        assert_eq!(
            p.blocking_directive(CspKind::ScriptEventHandler, None, None),
            None
        );
        assert_eq!(
            p.blocking_directive(CspKind::StyleAttribute, None, None),
            None
        );
        assert_eq!(
            p.blocking_directive(CspKind::InlineStyle, None, Some("x")),
            None
        );
    }

    #[test]
    fn effective_sources_falls_back_to_default_name_for_other_directives() {
        let mut p = CspPolicy::default();
        p.directives
            .insert("img-src".to_string(), vec!["'self'".to_string()]);
        let (name, sources) = p
            .effective_sources_prefer(&["img-src", "default-src"])
            .unwrap();
        assert_eq!(name, "img-src");
        assert_eq!(sources.len(), 1);
        assert_eq!(sources[0], "'self'");
    }

    #[test]
    fn parse_csp_policies_skips_empty_directives() {
        let policies = parse_csp_policies("default-src 'self'; ; script-src 'self'");
        assert_eq!(policies.len(), 1);
        assert!(policies[0].directives.contains_key("default-src"));
        assert!(policies[0].directives.contains_key("script-src"));
    }

    #[test]
    fn parse_csp_policies_lowercases_directive_names_and_trims_tokens() {
        let policies = parse_csp_policies(" DEFAULT-SRC   'self'  ;  SCRIPT-SRC  'nonce-abc'  ,  ");
        assert_eq!(policies.len(), 1);
        let p = &policies[0];
        assert!(p.directives.contains_key("default-src"));
        assert!(p.directives.contains_key("script-src"));
        assert_eq!(p.directives["default-src"], vec!["'self'"]);
        assert_eq!(p.directives["script-src"], vec!["'nonce-abc'"]);
    }

    #[test]
    fn blocking_directive_is_none_when_policy_has_no_relevant_directives() {
        let p = CspPolicy::default();
        assert_eq!(
            p.blocking_directive(CspKind::ScriptEventHandler, None, None),
            None
        );
        assert_eq!(
            p.blocking_directive(CspKind::InlineScript, Some("abc"), Some("x")),
            None
        );
        assert_eq!(
            p.blocking_directive(CspKind::InlineStyle, Some("abc"), Some("x")),
            None
        );
        assert_eq!(
            p.blocking_directive(CspKind::StyleAttribute, None, None),
            None
        );
    }

    #[test]
    fn external_script_blocks_when_not_self_and_not_listed() {
        let mut ctx = ValidationContext::new(
            Config {
                base_uri: Some("https://example.com/".to_string()),
                ..Config::default()
            },
            InputFormat::Html,
        );
        let mut sink = Sink::default();
        let mut rule = ContentSecurityPolicyWarnings::default();

        rule.on_event(
            &start_tag(
                "meta",
                vec![
                    attr("http-equiv", Some("Content-Security-Policy")),
                    attr("content", Some("script-src 'self'")),
                ],
            ),
            &mut ctx,
            &mut sink,
        );
        rule.on_event(
            &start_tag("script", vec![attr("src", Some("https://evil.com/x.js"))]),
            &mut ctx,
            &mut sink,
        );
        assert!(
            sink.0
                .iter()
                .any(|m| m.code == "html.csp.external_script.blocked")
        );
    }

    #[test]
    fn external_script_allows_listed_host_source() {
        let mut ctx = ValidationContext::new(
            Config {
                base_uri: Some("https://example.com/".to_string()),
                ..Config::default()
            },
            InputFormat::Html,
        );
        let mut sink = Sink::default();
        let mut rule = ContentSecurityPolicyWarnings::default();

        rule.on_event(
            &start_tag(
                "meta",
                vec![
                    attr("http-equiv", Some("Content-Security-Policy")),
                    attr("content", Some("script-src 'self' cdn.example.com")),
                ],
            ),
            &mut ctx,
            &mut sink,
        );
        rule.on_event(
            &start_tag(
                "script",
                vec![attr("src", Some("https://cdn.example.com/script.js"))],
            ),
            &mut ctx,
            &mut sink,
        );
        assert!(
            !sink
                .0
                .iter()
                .any(|m| m.code == "html.csp.external_script.blocked")
        );
    }

    #[test]
    fn default_src_is_used_for_img_when_img_src_missing() {
        let mut ctx = ValidationContext::new(
            Config {
                base_uri: Some("https://example.com/".to_string()),
                ..Config::default()
            },
            InputFormat::Html,
        );
        let mut sink = Sink::default();
        let mut rule = ContentSecurityPolicyWarnings::default();

        rule.on_event(
            &start_tag(
                "meta",
                vec![
                    attr("http-equiv", Some("Content-Security-Policy")),
                    attr("content", Some("default-src 'none'")),
                ],
            ),
            &mut ctx,
            &mut sink,
        );
        rule.on_event(
            &start_tag("img", vec![attr("src", Some("https://example.com/x.png"))]),
            &mut ctx,
            &mut sink,
        );
        assert!(sink.0.iter().any(|m| m.code == "html.csp.image.blocked"));
    }

    #[test]
    fn multiple_meta_tags_accumulate_policies() {
        let mut ctx = ValidationContext::new(
            Config {
                base_uri: Some("https://example.com/".to_string()),
                ..Config::default()
            },
            InputFormat::Html,
        );
        let mut sink = Sink::default();
        let mut rule = ContentSecurityPolicyWarnings::default();

        rule.on_event(
            &start_tag(
                "meta",
                vec![
                    attr("http-equiv", Some("Content-Security-Policy")),
                    attr("content", Some("script-src 'self'")),
                ],
            ),
            &mut ctx,
            &mut sink,
        );
        rule.on_event(
            &start_tag(
                "meta",
                vec![
                    attr("http-equiv", Some("Content-Security-Policy")),
                    attr("content", Some("script-src 'none'")),
                ],
            ),
            &mut ctx,
            &mut sink,
        );

        rule.on_event(
            &start_tag("script", vec![attr("src", Some("/ok.js"))]),
            &mut ctx,
            &mut sink,
        );
        assert!(
            sink.0
                .iter()
                .any(|m| m.code == "html.csp.external_script.blocked")
        );
    }

    #[test]
    fn script_src_attr_allows_event_handlers_even_if_script_src_blocks() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink::default();
        let mut rule = ContentSecurityPolicyWarnings::default();

        rule.on_event(
            &start_tag(
                "meta",
                vec![
                    attr("http-equiv", Some("Content-Security-Policy")),
                    attr(
                        "content",
                        Some("script-src 'self'; script-src-attr 'unsafe-inline'"),
                    ),
                ],
            ),
            &mut ctx,
            &mut sink,
        );
        rule.on_event(
            &start_tag("div", vec![attr("onclick", Some("x"))]),
            &mut ctx,
            &mut sink,
        );
        assert!(
            !sink
                .0
                .iter()
                .any(|m| m.code == "html.csp.event_handler.blocked")
        );
    }

    #[test]
    fn style_src_attr_allows_style_attribute_even_if_style_src_blocks() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink::default();
        let mut rule = ContentSecurityPolicyWarnings::default();

        rule.on_event(
            &start_tag(
                "meta",
                vec![
                    attr("http-equiv", Some("Content-Security-Policy")),
                    attr(
                        "content",
                        Some("style-src 'self'; style-src-attr 'unsafe-inline'"),
                    ),
                ],
            ),
            &mut ctx,
            &mut sink,
        );
        rule.on_event(
            &start_tag("div", vec![attr("style", Some("color: red"))]),
            &mut ctx,
            &mut sink,
        );
        assert!(
            !sink
                .0
                .iter()
                .any(|m| m.code == "html.csp.style_attribute.blocked")
        );
    }

    #[test]
    fn csp_header_is_enforced_and_takes_precedence_in_message_source() {
        let mut ctx = ValidationContext::new(
            Config {
                csp_header: Some("script-src 'none'".to_string()),
                ..Config::default()
            },
            InputFormat::Html,
        );
        let mut sink = Sink::default();
        let mut rule = ContentSecurityPolicyWarnings::default();
        rule.init(&ctx);

        rule.on_event(&start_tag("script", vec![]), &mut ctx, &mut sink);
        rule.on_event(&text("alert(1)"), &mut ctx, &mut sink);
        rule.on_event(&end_tag("script"), &mut ctx, &mut sink);
        let msg = sink
            .0
            .iter()
            .find(|m| m.code == "html.csp.inline_script.blocked")
            .expect("expected inline script warning");
        assert!(msg.message.contains("HTTP header"));
    }

    #[test]
    fn csp_header_and_meta_both_apply() {
        let mut ctx = ValidationContext::new(
            Config {
                csp_header: Some("script-src 'self'".to_string()),
                ..Config::default()
            },
            InputFormat::Html,
        );
        let mut sink = Sink::default();
        let mut rule = ContentSecurityPolicyWarnings::default();
        rule.init(&ctx);

        // Meta policy blocks scripts, header allows: should still be blocked.
        rule.on_event(
            &start_tag(
                "meta",
                vec![
                    attr("http-equiv", Some("Content-Security-Policy")),
                    attr("content", Some("script-src 'none'")),
                ],
            ),
            &mut ctx,
            &mut sink,
        );
        rule.on_event(
            &start_tag("script", vec![attr("src", Some("/ok.js"))]),
            &mut ctx,
            &mut sink,
        );
        assert!(
            sink.0
                .iter()
                .any(|m| m.code == "html.csp.external_script.blocked")
        );
    }
}
