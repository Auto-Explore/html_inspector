use html_inspector::{
    Attribute, Category, Interest, Message, MessageSink, ParseEvent, Rule, RuleSet, Severity, Span,
    ValidationContext,
};

fn attr_value_ci<'a>(attrs: &'a [Attribute], name: &str) -> Option<&'a str> {
    attrs.iter().find_map(|a| {
        let value = a.value.as_deref()?;
        a.name.eq_ignore_ascii_case(name).then_some(value)
    })
}

fn css_config_from_ctx(ctx: &ValidationContext) -> css_inspector::Config {
    css_inspector::Config {
        // Match vnu.jar defaults (Assertions.java): css3svg, medium=all, warningLevel=-1.
        profile: ctx
            .config
            .css_profile
            .clone()
            .or_else(|| Some("css3svg".to_string())),
        medium: ctx
            .config
            .css_medium
            .clone()
            .or_else(|| Some("all".to_string())),
        warning: ctx
            .config
            .css_warning
            .clone()
            .or_else(|| Some("-1".to_string())),
    }
}

fn line_col_at_byte(s: &str, byte: usize) -> (u32, u32) {
    let mut line: u32 = 1;
    let mut col: u32 = 1;
    for &b in s.as_bytes().iter().take(byte.min(s.len())) {
        if b == b'\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }
    (line, col)
}

fn emit_css_errors(report: &css_inspector::Report, span: Option<Span>, out: &mut dyn MessageSink) {
    for m in &report.messages {
        if m.severity != css_inspector::Severity::Error {
            continue;
        }
        out.push(Message::new(
            "html.css.error",
            Severity::Error,
            Category::Html,
            format!("CSS: {}", m.message),
            span,
        ));
    }
}

fn emit_css_errors_in_style_text(
    report: &css_inspector::Report,
    style_span: Option<Span>,
    text_span: Option<Span>,
    css: &str,
    out: &mut dyn MessageSink,
) {
    for m in &report.messages {
        if m.severity != css_inspector::Severity::Error {
            continue;
        }

        let span = text_span
            .and_then(|base| locate_css_error_span(css, base, &m.message))
            .or(style_span);

        out.push(Message::new(
            "html.css.error",
            Severity::Error,
            Category::Html,
            format!("CSS: {}", m.message),
            span,
        ));
    }
}

fn locate_css_error_span(css: &str, base: Span, message: &str) -> Option<Span> {
    let byte = locate_css_error_byte(css, message)?;
    let (rel_line, rel_col) = line_col_at_byte(css, byte);
    let abs_line = base.line + rel_line - 1;
    let abs_col = if rel_line == 1 {
        base.col + rel_col - 1
    } else {
        rel_col
    };
    Some(Span::new(
        base.byte_start + byte,
        base.byte_start + byte.saturating_add(1),
        abs_line,
        abs_col,
    ))
}

fn locate_css_error_byte(css: &str, message: &str) -> Option<usize> {
    if let Some(token) = first_curly_quoted_token(message) {
        if let Some(pos) = css.find(token) {
            return Some(pos);
        }
        if token.is_ascii() {
            return find_substring_ascii_ci(css, token);
        }
    }
    if message.trim() == "Unclosed comment." {
        return css.rfind("/*");
    }
    None
}

fn first_curly_quoted_token(message: &str) -> Option<&str> {
    let start = message.find('“')? + '“'.len_utf8();
    let end = message[start..].find('”')? + start;
    Some(&message[start..end])
}

fn find_substring_ascii_ci(haystack: &str, needle: &str) -> Option<usize> {
    if needle.is_empty() {
        return Some(0);
    }
    let needle_lower = needle.to_ascii_lowercase();
    haystack
        .as_bytes()
        .windows(needle.len())
        .position(|window| {
            window
                .iter()
                .zip(needle_lower.as_bytes())
                .all(|(&h, &n)| h.to_ascii_lowercase() == n)
        })
}

fn report_from_err(e: css_inspector::ValidatorError) -> css_inspector::Report {
    css_inspector::Report {
        errors: 1,
        messages: vec![css_inspector::Message {
            severity: css_inspector::Severity::Error,
            message: e.to_string(),
        }],
        ..css_inspector::Report::default()
    }
}

pub fn pack_css_checks() -> RuleSet {
    RuleSet::new()
        .push(StyleAttributeCssRule::default())
        .push(StyleElementCssRule::default())
}

#[derive(Default)]
struct StyleAttributeCssRule {
    css_config: Option<css_inspector::Config>,
}

impl Rule for StyleAttributeCssRule {
    fn id(&self) -> &'static str {
        "css.style_attribute"
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
        if !ctx.config.also_check_css {
            return;
        }
        // Match vnu.jar behavior: CSS checks are skipped inside <template> contents.
        if ctx.has_ancestor("template") {
            return;
        }
        let ParseEvent::StartTag { attrs, span, .. } = event else {
            return;
        };
        let Some(style_contents) = attr_value_ci(attrs, "style") else {
            return;
        };
        if style_contents.trim().is_empty() {
            return;
        }

        let css_config = self
            .css_config
            .get_or_insert_with(|| css_config_from_ctx(ctx));

        // A style attribute is a declaration list, not a full stylesheet.
        let report = css_inspector::validate_css_declarations_text(style_contents, css_config)
            .unwrap_or_else(report_from_err);
        emit_css_errors(&report, *span, out);
    }
}

#[derive(Default)]
struct StyleElementCssRule {
    depth: usize,
    buf: String,
    style_span: Option<Span>,
    text_span: Option<Span>,
    css_config: Option<css_inspector::Config>,
}

impl Rule for StyleElementCssRule {
    fn id(&self) -> &'static str {
        "css.style_element"
    }

    fn interest(&self) -> Interest {
        Interest::START_TAG | Interest::END_TAG | Interest::TEXT
    }

    fn on_event(
        &mut self,
        event: &ParseEvent,
        ctx: &mut ValidationContext,
        out: &mut dyn MessageSink,
    ) {
        if !ctx.config.also_check_css {
            return;
        }
        // Match vnu.jar behavior: CSS checks are skipped inside <template> contents.
        if ctx.has_ancestor("template") {
            return;
        }
        match event {
            ParseEvent::StartTag { name, span, .. } if name.eq_ignore_ascii_case("style") => {
                self.depth += 1;
                if self.depth == 1 {
                    self.buf.clear();
                    self.style_span = *span;
                    self.text_span = None;
                }
            }
            ParseEvent::Text { text, span } if self.depth != 0 => {
                if self.text_span.is_none() {
                    self.text_span = *span;
                }
                self.buf.push_str(text);
            }
            ParseEvent::EndTag { name, .. }
                if self.depth != 0 && name.eq_ignore_ascii_case("style") =>
            {
                self.depth -= 1;
                if self.depth != 0 {
                    return;
                }
                let css = self.buf.strip_prefix('\u{feff}').unwrap_or(&self.buf);
                if css.trim().is_empty() {
                    return;
                }

                let css_config = self
                    .css_config
                    .get_or_insert_with(|| css_config_from_ctx(ctx));

                // Keep inline stylesheet validation self-contained (do not attempt network
                // fetching for @import), matching the service behavior and avoiding
                // spurious errors when network is disabled.
                let report = css_inspector::validate_css_text(css, css_config)
                    .unwrap_or_else(report_from_err);

                emit_css_errors_in_style_text(&report, self.style_span, self.text_span, css, out);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use html_inspector::{
        Attribute, Category, Config, EventSource, InputFormat, Message, MessageOrder, MessageSink,
        ParseEvent, RuleSet, Span, validate_events,
    };

    use super::{attr_value_ci, pack_css_checks};

    #[derive(Default)]
    struct VecSink(Vec<Message>);

    impl MessageSink for VecSink {
        fn push(&mut self, msg: Message) {
            self.0.push(msg);
        }
    }

    struct CaptureSource {
        format: InputFormat,
        events: std::vec::IntoIter<ParseEvent>,
    }

    impl CaptureSource {
        fn new(format: InputFormat, events: Vec<ParseEvent>) -> Self {
            Self {
                format,
                events: events.into_iter(),
            }
        }
    }

    impl EventSource for CaptureSource {
        fn source_name(&self) -> &str {
            "capture"
        }

        fn format(&self) -> InputFormat {
            self.format
        }

        fn next_event(&mut self) -> Result<Option<ParseEvent>, html_inspector::ValidatorError> {
            Ok(self.events.next())
        }
    }

    fn default_test_config() -> Config {
        Config {
            ignore_missing_lang: true,
            message_order: MessageOrder::Emit,
            also_check_css: true,
            severity_profile: html_inspector::SeverityProfile::Risk,
            min_severity: html_inspector::Severity::Info,
            base_uri: None,
            css_profile: None,
            css_medium: None,
            css_warning: None,
            csp_header: None,
        }
    }

    #[test]
    fn attr_value_ci_matches_name_case_insensitively() {
        let attrs = vec![Attribute {
            name: "STYLE".to_string(),
            value: Some("color: red".to_string()),
            span: None,
        }];
        assert_eq!(attr_value_ci(&attrs, "style"), Some("color: red"));
    }

    #[test]
    fn attr_value_ci_returns_none_when_value_is_missing() {
        let attrs = vec![Attribute {
            name: "style".to_string(),
            value: None,
            span: None,
        }];
        assert_eq!(attr_value_ci(&attrs, "style"), None);
    }

    #[test]
    fn attr_value_ci_skips_missing_values_for_duplicate_attributes() {
        let attrs = vec![
            Attribute {
                name: "style".to_string(),
                value: None,
                span: None,
            },
            Attribute {
                name: "STYLE".to_string(),
                value: Some("color: red".to_string()),
                span: None,
            },
        ];
        assert_eq!(attr_value_ci(&attrs, "style"), Some("color: red"));
    }

    #[test]
    fn attr_value_ci_returns_first_matching_value_when_multiple_present() {
        let attrs = vec![
            Attribute {
                name: "style".to_string(),
                value: Some("a".to_string()),
                span: None,
            },
            Attribute {
                name: "STYLE".to_string(),
                value: Some("b".to_string()),
                span: None,
            },
        ];
        assert_eq!(attr_value_ci(&attrs, "style"), Some("a"));
    }

    #[test]
    fn starts_with_ascii_ci_is_case_insensitive_and_safe_on_short_inputs() {
        assert!(css_inspector::starts_with_ascii_ci("FILE://x", "file://"));
        assert!(css_inspector::starts_with_ascii_ci("file://x", "FILE://"));
        assert!(!css_inspector::starts_with_ascii_ci("fi", "file://"));
        assert!(css_inspector::starts_with_ascii_ci("", ""));
        assert!(css_inspector::starts_with_ascii_ci("❤H", "❤h"));
    }

    #[test]
    fn emit_css_errors_emits_only_errors() {
        let report = css_inspector::Report {
            errors: 1,
            warnings: 1,
            messages: vec![
                css_inspector::Message {
                    severity: css_inspector::Severity::Warning,
                    message: "w".to_string(),
                },
                css_inspector::Message {
                    severity: css_inspector::Severity::Error,
                    message: "e".to_string(),
                },
            ],
        };

        let mut out = VecSink::default();
        super::emit_css_errors(&report, None, &mut out);

        assert_eq!(out.0.len(), 1);
        assert_eq!(out.0[0].code, "html.css.error");
        assert!(out.0[0].message.contains("CSS: e"));
    }

    #[test]
    fn style_attribute_css_rule_emits_errors() {
        let span = Span::new(0, 0, 1, 1);
        let src = CaptureSource::new(
            InputFormat::Html,
            vec![ParseEvent::StartTag {
                name: "p".to_string(),
                attrs: vec![Attribute {
                    name: "style".to_string(),
                    value: Some("/*".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: Some(span),
            }],
        );

        let report = validate_events(src, pack_css_checks(), default_test_config()).unwrap();
        assert!(report.messages.iter().any(|m| {
            m.code == "html.css.error"
                && m.severity == html_inspector::Severity::Error
                && m.category == Category::Html
        }));
    }

    #[test]
    fn css_defaults_to_css3svg_profile_for_style_attribute() {
        let src = CaptureSource::new(
            InputFormat::Html,
            vec![ParseEvent::StartTag {
                name: "p".to_string(),
                attrs: vec![Attribute {
                    name: "style".to_string(),
                    value: Some("fill-opacity: 1".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            }],
        );

        let report = validate_events(src, pack_css_checks(), default_test_config()).unwrap();
        assert!(
            !report.messages.iter().any(|m| m.code == "html.css.error"),
            "unexpected CSS errors: {:#?}",
            report.messages
        );
    }

    #[test]
    fn css_defaults_do_not_emit_vendor_extension_warnings_as_errors_for_style_attribute() {
        let src = CaptureSource::new(
            InputFormat::Html,
            vec![ParseEvent::StartTag {
                name: "p".to_string(),
                attrs: vec![Attribute {
                    name: "style".to_string(),
                    value: Some("-webkit-foo: 1".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            }],
        );

        let report = validate_events(src, pack_css_checks(), default_test_config()).unwrap();
        assert!(
            !report.messages.iter().any(|m| m.code == "html.css.error"),
            "unexpected CSS errors: {:#?}",
            report.messages
        );
    }

    #[test]
    fn css_checks_are_noop_when_disabled() {
        let span = Span::new(0, 0, 1, 1);
        let src = CaptureSource::new(
            InputFormat::Html,
            vec![ParseEvent::StartTag {
                name: "p".to_string(),
                attrs: vec![Attribute {
                    name: "style".to_string(),
                    value: Some("/*".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: Some(span),
            }],
        );

        let config = Config {
            also_check_css: false,
            ..default_test_config()
        };
        let report = validate_events(src, pack_css_checks(), config).unwrap();
        assert!(
            !report.messages.iter().any(|m| m.code == "html.css.error"),
            "{report:?}"
        );
    }

    #[test]
    fn css_checks_skip_template_contents_for_style_attribute() {
        let src = CaptureSource::new(
            InputFormat::Html,
            vec![
                ParseEvent::StartTag {
                    name: "template".to_string(),
                    attrs: vec![],
                    self_closing: false,
                    span: None,
                },
                ParseEvent::StartTag {
                    name: "p".to_string(),
                    attrs: vec![Attribute {
                        name: "style".to_string(),
                        value: Some("/*".to_string()),
                        span: None,
                    }],
                    self_closing: false,
                    span: None,
                },
                ParseEvent::EndTag {
                    name: "p".to_string(),
                    span: None,
                },
                ParseEvent::EndTag {
                    name: "template".to_string(),
                    span: None,
                },
            ],
        );

        let report = validate_events(src, pack_css_checks(), default_test_config()).unwrap();
        assert!(
            !report.messages.iter().any(|m| m.code == "html.css.error"),
            "unexpected CSS errors: {:#?}",
            report.messages
        );
    }

    #[test]
    fn css_checks_skip_template_contents_for_style_element() {
        let src = CaptureSource::new(
            InputFormat::Html,
            vec![
                ParseEvent::StartTag {
                    name: "template".to_string(),
                    attrs: vec![],
                    self_closing: false,
                    span: None,
                },
                ParseEvent::StartTag {
                    name: "style".to_string(),
                    attrs: vec![],
                    self_closing: false,
                    span: None,
                },
                ParseEvent::Text {
                    text: "/*".to_string(),
                    span: None,
                },
                ParseEvent::EndTag {
                    name: "style".to_string(),
                    span: None,
                },
                ParseEvent::EndTag {
                    name: "template".to_string(),
                    span: None,
                },
            ],
        );

        let rules = RuleSet::new().merge(pack_css_checks());
        let report = validate_events(src, rules, default_test_config()).unwrap();
        assert!(
            !report.messages.iter().any(|m| m.code == "html.css.error"),
            "unexpected CSS errors: {:#?}",
            report.messages
        );
    }

    #[test]
    fn style_attribute_css_rule_handles_multiple_events() {
        let span1 = Span::new(0, 0, 1, 1);
        let span2 = Span::new(0, 0, 2, 1);
        let src = CaptureSource::new(
            InputFormat::Html,
            vec![
                ParseEvent::StartTag {
                    name: "p".to_string(),
                    attrs: vec![Attribute {
                        name: "style".to_string(),
                        value: Some("/*".to_string()),
                        span: None,
                    }],
                    self_closing: false,
                    span: Some(span1),
                },
                ParseEvent::StartTag {
                    name: "p".to_string(),
                    attrs: vec![Attribute {
                        name: "style".to_string(),
                        value: Some("/*".to_string()),
                        span: None,
                    }],
                    self_closing: false,
                    span: Some(span2),
                },
            ],
        );

        let report = validate_events(src, pack_css_checks(), default_test_config()).unwrap();
        let spans: Vec<_> = report
            .messages
            .iter()
            .filter(|m| m.code == "html.css.error")
            .filter_map(|m| m.span)
            .collect();
        assert_eq!(spans, vec![span1, span2]);
    }

    #[test]
    fn style_element_css_errors_are_localized_within_style_text_on_first_line() {
        let style_span = Span::new(10, 20, 7, 1);
        let text_span = Span::new(1000, 1001, 7, 20);
        let src = CaptureSource::new(
            InputFormat::Html,
            vec![
                ParseEvent::StartTag {
                    name: "style".to_string(),
                    attrs: vec![],
                    self_closing: false,
                    span: Some(style_span),
                },
                ParseEvent::Text {
                    text: "a{foo:1}".to_string(),
                    span: Some(text_span),
                },
                ParseEvent::EndTag {
                    name: "style".to_string(),
                    span: None,
                },
            ],
        );

        let report = validate_events(src, pack_css_checks(), default_test_config()).unwrap();
        let msg = report
            .messages
            .iter()
            .find(|m| m.code == "html.css.error")
            .expect("expected css error");
        assert_eq!(
            msg.span,
            Some(Span::new(1002, 1003, 7, 22)),
            "unexpected span for css error: {msg:?}"
        );
    }

    #[test]
    fn style_element_css_errors_are_localized_within_style_text_on_subsequent_lines() {
        let style_span = Span::new(10, 20, 7, 1);
        let text_span = Span::new(1000, 1001, 7, 20);
        let src = CaptureSource::new(
            InputFormat::Html,
            vec![
                ParseEvent::StartTag {
                    name: "style".to_string(),
                    attrs: vec![],
                    self_closing: false,
                    span: Some(style_span),
                },
                ParseEvent::Text {
                    text: "a{\nfoo:1}".to_string(),
                    span: Some(text_span),
                },
                ParseEvent::EndTag {
                    name: "style".to_string(),
                    span: None,
                },
            ],
        );

        let report = validate_events(src, pack_css_checks(), default_test_config()).unwrap();
        let msg = report
            .messages
            .iter()
            .find(|m| m.code == "html.css.error")
            .expect("expected css error");
        assert_eq!(
            msg.span,
            Some(Span::new(1003, 1004, 8, 1)),
            "unexpected span for css error: {msg:?}"
        );
    }

    #[test]
    fn style_element_css_rule_strips_leading_bom() {
        let src = CaptureSource::new(
            InputFormat::Html,
            vec![
                ParseEvent::StartTag {
                    name: "style".to_string(),
                    attrs: vec![],
                    self_closing: false,
                    span: None,
                },
                ParseEvent::Text {
                    text: "\u{feff}a{color:red;}".to_string(),
                    span: None,
                },
                ParseEvent::EndTag {
                    name: "style".to_string(),
                    span: None,
                },
            ],
        );

        let rules = RuleSet::new().merge(pack_css_checks());
        let report = validate_events(src, rules, default_test_config()).unwrap();
        assert!(
            !report.messages.iter().any(|m| m.code == "html.css.error"),
            "unexpected CSS errors: {:#?}",
            report.messages
        );
    }

    #[test]
    fn css_defaults_to_css3svg_profile_for_style_element() {
        let src = CaptureSource::new(
            InputFormat::Html,
            vec![
                ParseEvent::StartTag {
                    name: "style".to_string(),
                    attrs: vec![],
                    self_closing: false,
                    span: None,
                },
                ParseEvent::Text {
                    text: "a{fill-opacity:1;}".to_string(),
                    span: None,
                },
                ParseEvent::EndTag {
                    name: "style".to_string(),
                    span: None,
                },
            ],
        );

        let rules = RuleSet::new().merge(pack_css_checks());
        let report = validate_events(src, rules, default_test_config()).unwrap();
        assert!(
            !report.messages.iter().any(|m| m.code == "html.css.error"),
            "unexpected CSS errors: {:#?}",
            report.messages
        );
    }

    #[test]
    fn style_element_css_rule_does_not_emit_warnings_as_errors() {
        let src = CaptureSource::new(
            InputFormat::Html,
            vec![
                ParseEvent::StartTag {
                    name: "style".to_string(),
                    attrs: vec![],
                    self_closing: false,
                    span: None,
                },
                ParseEvent::Text {
                    text: "@import url(a);".to_string(),
                    span: None,
                },
                ParseEvent::EndTag {
                    name: "style".to_string(),
                    span: None,
                },
            ],
        );

        let rules = RuleSet::new().merge(pack_css_checks());
        let report = validate_events(src, rules, default_test_config()).unwrap();
        assert!(
            !report.messages.iter().any(|m| m.code == "html.css.error"),
            "unexpected CSS errors: {:#?}",
            report.messages
        );
    }

    #[test]
    fn style_element_css_rule_emits_errors_once_with_outer_span_when_nested() {
        let outer = Span::new(0, 0, 1, 1);
        let inner = Span::new(0, 0, 2, 1);
        let src = CaptureSource::new(
            InputFormat::Html,
            vec![
                ParseEvent::StartTag {
                    name: "style".to_string(),
                    attrs: vec![],
                    self_closing: false,
                    span: Some(outer),
                },
                ParseEvent::Text {
                    text: "/*".to_string(),
                    span: None,
                },
                ParseEvent::StartTag {
                    name: "style".to_string(),
                    attrs: vec![],
                    self_closing: false,
                    span: Some(inner),
                },
                ParseEvent::Text {
                    text: "x".to_string(),
                    span: None,
                },
                ParseEvent::EndTag {
                    name: "style".to_string(),
                    span: Some(inner),
                },
                ParseEvent::EndTag {
                    name: "style".to_string(),
                    span: Some(outer),
                },
            ],
        );

        let rules = RuleSet::new().merge(pack_css_checks());
        let report = validate_events(src, rules, default_test_config()).unwrap();
        let css_errors: Vec<_> = report
            .messages
            .iter()
            .filter(|m| m.code == "html.css.error")
            .collect();
        assert_eq!(css_errors.len(), 1);
        assert_eq!(css_errors[0].span, Some(outer));
    }

    #[test]
    fn attr_value_ci_finds_value_case_insensitively() {
        let attrs = vec![
            Attribute {
                name: "REL".to_string(),
                value: Some("stylesheet".to_string()),
                span: None,
            },
            Attribute {
                name: "href".to_string(),
                value: None,
                span: None,
            },
        ];

        assert_eq!(super::attr_value_ci(&attrs, "rel"), Some("stylesheet"));
        assert_eq!(super::attr_value_ci(&attrs, "href"), None);
        assert_eq!(super::attr_value_ci(&attrs, "missing"), None);
    }

    #[test]
    fn attr_value_ci_skips_missing_values_for_matching_attributes() {
        let attrs = vec![
            Attribute {
                name: "href".to_string(),
                value: None,
                span: None,
            },
            Attribute {
                name: "HREF".to_string(),
                value: Some("a.css".to_string()),
                span: None,
            },
        ];

        assert_eq!(super::attr_value_ci(&attrs, "href"), Some("a.css"));
    }

    #[test]
    fn attr_value_ci_returns_first_value_when_duplicates_have_values() {
        let attrs = vec![
            Attribute {
                name: "href".to_string(),
                value: Some("a.css".to_string()),
                span: None,
            },
            Attribute {
                name: "HREF".to_string(),
                value: Some("b.css".to_string()),
                span: None,
            },
        ];

        assert_eq!(super::attr_value_ci(&attrs, "href"), Some("a.css"));
    }

    #[test]
    fn starts_with_ascii_ci_compares_bytes_case_insensitively() {
        assert!(css_inspector::starts_with_ascii_ci(
            "FILE:///tmp/a.css",
            "file://"
        ));
        assert!(!css_inspector::starts_with_ascii_ci(
            "ftp://example.com/a.css",
            "file://"
        ));
        assert!(!css_inspector::starts_with_ascii_ci("", "file://"));
        assert!(!css_inspector::starts_with_ascii_ci("❤", "h"));
    }

    #[test]
    fn emit_css_errors_prefixes_message_and_filters_warnings() {
        struct Collect(Vec<html_inspector::Message>);
        impl html_inspector::MessageSink for Collect {
            fn push(&mut self, msg: html_inspector::Message) {
                self.0.push(msg);
            }
        }

        let report = css_inspector::Report {
            errors: 1,
            warnings: 1,
            messages: vec![
                css_inspector::Message {
                    severity: css_inspector::Severity::Warning,
                    message: "w".to_string(),
                },
                css_inspector::Message {
                    severity: css_inspector::Severity::Error,
                    message: "bad".to_string(),
                },
            ],
        };
        let span = Some(Span::new(1, 2, 3, 4));
        let mut out = Collect(Vec::new());
        super::emit_css_errors(&report, span, &mut out);

        assert_eq!(out.0.len(), 1);
        assert_eq!(out.0[0].code, "html.css.error");
        assert_eq!(out.0[0].message, "CSS: bad");
        assert_eq!(out.0[0].span, span);
    }
}
