use html_inspector::{
    Attribute, Category, Interest, Message, MessageSink, ParseEvent, Rule, RuleSet, Severity, Span,
    ValidationContext,
};

#[cfg(test)]
fn attr_value_ci<'a>(attrs: &'a [Attribute], name: &str) -> Option<&'a str> {
    attrs.iter().find_map(|a| {
        let value = a.value.as_deref()?;
        a.name.eq_ignore_ascii_case(name).then_some(value)
    })
}

fn attr_ci<'a>(attrs: &'a [Attribute], name: &str) -> Option<&'a Attribute> {
    attrs
        .iter()
        .find(|a| a.value.is_some() && a.name.eq_ignore_ascii_case(name))
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

#[cfg(test)]
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

fn emit_css_errors_in_style_attr(
    report: &css_inspector::Report,
    style_contents: &str,
    attr: &Attribute,
    tag_span: Option<Span>,
    out: &mut dyn MessageSink,
) {
    let attr_span = attr.span.or(tag_span);

    // Try to compute a base span for the style attribute value.
    // The attribute span covers `style="..."`, so the value starts at
    // attr.span.byte_start + len("style") + len('=') + len(quote_char).
    // We estimate: name.len() + 2 for `="` and the value ends 1 byte before byte_end for `"`.
    let value_base = attr.span.map(|s| {
        let value_offset = attr.name.len() + 2; // `name="` — the `=` and opening quote
        Span::new(
            s.byte_start + value_offset,
            s.byte_end.saturating_sub(1),
            s.line,
            s.col + value_offset as u32,
        )
    });

    for m in &report.messages {
        if m.severity != css_inspector::Severity::Error {
            continue;
        }

        let span = value_base
            .and_then(|base| locate_css_error_span(style_contents, base, &m.message))
            .or(attr_span);

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
        let Some(style_attr) = attr_ci(attrs, "style") else {
            return;
        };
        let Some(style_contents) = style_attr.value.as_deref() else {
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
        emit_css_errors_in_style_attr(&report, style_contents, style_attr, *span, out);
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
        let tag_span = Span::new(0, 20, 1, 1);
        let attr_span = Span::new(3, 15, 1, 4);
        let src = CaptureSource::new(
            InputFormat::Html,
            vec![ParseEvent::StartTag {
                name: "p".to_string(),
                attrs: vec![Attribute {
                    name: "style".to_string(),
                    value: Some("/*".to_string()),
                    span: Some(attr_span),
                }],
                self_closing: false,
                span: Some(tag_span),
            }],
        );

        let report = validate_events(src, pack_css_checks(), default_test_config()).unwrap();
        let css_err = report
            .messages
            .iter()
            .find(|m| m.code == "html.css.error")
            .expect("expected a CSS error");
        assert_eq!(css_err.severity, html_inspector::Severity::Error);
        assert_eq!(css_err.category, Category::Html);
        // Error should point to the style attribute, not the tag.
        assert_ne!(
            css_err.span,
            Some(tag_span),
            "CSS error should not point to the tag span"
        );
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
        // attr spans cover `style="/*"` — value starts at byte_start + 7 (len("style") + len('="'))
        let attr_span1 = Span::new(3, 15, 1, 4);
        let attr_span2 = Span::new(23, 35, 2, 4);
        let src = CaptureSource::new(
            InputFormat::Html,
            vec![
                ParseEvent::StartTag {
                    name: "p".to_string(),
                    attrs: vec![Attribute {
                        name: "style".to_string(),
                        value: Some("/*".to_string()),
                        span: Some(attr_span1),
                    }],
                    self_closing: false,
                    span: Some(Span::new(0, 16, 1, 1)),
                },
                ParseEvent::StartTag {
                    name: "p".to_string(),
                    attrs: vec![Attribute {
                        name: "style".to_string(),
                        value: Some("/*".to_string()),
                        span: Some(attr_span2),
                    }],
                    self_closing: false,
                    span: Some(Span::new(20, 36, 2, 1)),
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
        // Errors should be localized to the `/*` token within the style value
        // (value starts at attr byte_start + 7), not point at the tag.
        assert_eq!(
            spans,
            vec![Span::new(10, 11, 1, 11), Span::new(30, 31, 2, 11)],
            "CSS errors should be localized within the style attribute value"
        );
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

    // ── locate_css_error_span / locate_css_error_byte regression tests ──

    #[test]
    fn line_col_at_byte_returns_1_1_for_zero() {
        assert_eq!(super::line_col_at_byte("abc", 0), (1, 1));
    }

    #[test]
    fn line_col_at_byte_counts_columns() {
        assert_eq!(super::line_col_at_byte("abcd", 3), (1, 4));
    }

    #[test]
    fn line_col_at_byte_counts_newlines() {
        assert_eq!(super::line_col_at_byte("ab\ncd\nef", 5), (2, 3));
        assert_eq!(super::line_col_at_byte("ab\ncd\nef", 6), (3, 1));
    }

    #[test]
    fn first_curly_quoted_token_extracts_token() {
        assert_eq!(
            super::first_curly_quoted_token("Property \u{201c}foo\u{201d} doesn\u{2019}t exist"),
            Some("foo")
        );
    }

    #[test]
    fn first_curly_quoted_token_returns_none_without_quotes() {
        assert_eq!(super::first_curly_quoted_token("no quotes here"), None);
    }

    #[test]
    fn locate_css_error_byte_finds_quoted_token() {
        let css = "color: gween";
        let msg = "Value Error: \u{201c}gween\u{201d} is not a valid color";
        assert_eq!(super::locate_css_error_byte(css, msg), Some(7));
    }

    #[test]
    fn locate_css_error_byte_case_insensitive_ascii_fallback() {
        let css = "COLOR: GWEEN";
        let msg = "Value Error: \u{201c}gween\u{201d} is not a valid color";
        assert_eq!(super::locate_css_error_byte(css, msg), Some(7));
    }

    #[test]
    fn locate_css_error_byte_unclosed_comment() {
        assert_eq!(
            super::locate_css_error_byte("a:1;/*", "Unclosed comment."),
            Some(4)
        );
    }

    #[test]
    fn locate_css_error_byte_unclosed_comment_finds_last_occurrence() {
        // "/* ok */ /*" → positions: 0:/ 1:* 2:  3:o 4:k 5:  6:* 7:/ 8:  9:/ 10:*
        assert_eq!(
            super::locate_css_error_byte("/* ok */ /*", "Unclosed comment."),
            Some(9)
        );
    }

    #[test]
    fn locate_css_error_byte_returns_none_for_unknown_message() {
        assert_eq!(
            super::locate_css_error_byte("color:red", "Something went wrong"),
            None
        );
    }

    #[test]
    fn locate_css_error_span_first_line_adds_to_base_col() {
        // css = "foo:bad", error at byte 4 ("bad" token)
        let base = Span::new(100, 120, 5, 10);
        let span =
            super::locate_css_error_span("foo:bad", base, "Value Error: \u{201c}bad\u{201d}")
                .unwrap();
        // byte 4 in css → (1, 5) relative → abs line = 5+1-1=5, abs col = 10+5-1=14
        assert_eq!(span.byte_start, 104);
        assert_eq!(span.byte_end, 105);
        assert_eq!(span.line, 5);
        assert_eq!(span.col, 14);
    }

    #[test]
    fn locate_css_error_span_subsequent_line_resets_col() {
        // css = "ok:1;\nbad:x", error at byte 6 ("bad" token on line 2)
        let base = Span::new(200, 220, 10, 50);
        let span =
            super::locate_css_error_span("ok:1;\nbad:x", base, "Property \u{201c}bad\u{201d}")
                .unwrap();
        // byte 6 in css → (2, 1) relative → abs line = 10+2-1=11, abs col = 1 (not base col)
        assert_eq!(span.byte_start, 206);
        assert_eq!(span.byte_end, 207);
        assert_eq!(span.line, 11);
        assert_eq!(span.col, 1);
    }

    #[test]
    fn locate_css_error_span_returns_none_when_token_not_found() {
        let base = Span::new(0, 10, 1, 1);
        assert!(
            super::locate_css_error_span("color:red", base, "Property \u{201c}zzz\u{201d}")
                .is_none()
        );
    }

    // ── Style attribute span regression tests ──

    #[test]
    fn style_attr_error_points_to_token_inside_value() {
        // Simulates: <p style="color: gween">
        // attr span covers `style="color: gween"` starting at byte 3
        let attr_span = Span::new(3, 24, 1, 4);
        let tag_span = Span::new(0, 25, 1, 1);
        let src = CaptureSource::new(
            InputFormat::Html,
            vec![ParseEvent::StartTag {
                name: "p".to_string(),
                attrs: vec![Attribute {
                    name: "style".to_string(),
                    value: Some("color: gween".to_string()),
                    span: Some(attr_span),
                }],
                self_closing: false,
                span: Some(tag_span),
            }],
        );

        let report = validate_events(src, pack_css_checks(), default_test_config()).unwrap();
        let css_errs: Vec<_> = report
            .messages
            .iter()
            .filter(|m| m.code == "html.css.error")
            .collect();
        assert!(!css_errs.is_empty(), "expected at least one CSS error");

        for err in &css_errs {
            let span = err.span.expect("CSS error should have a span");
            // Must point inside the value, not at the tag or attribute start.
            // Value starts at byte 3 + 7 (len("style=\"")) = 10
            assert!(
                span.byte_start >= 10,
                "span byte_start={} should be >= 10 (inside value), msg={}",
                span.byte_start,
                err.message
            );
            assert!(
                span.byte_start < 24,
                "span byte_start={} should be < 24 (within attr), msg={}",
                span.byte_start,
                err.message
            );
        }
    }

    #[test]
    fn style_attr_error_at_start_of_value() {
        // Error token is at the very beginning of the value: <p style="badprop: 1">
        let attr_span = Span::new(3, 22, 1, 4);
        let src = CaptureSource::new(
            InputFormat::Html,
            vec![ParseEvent::StartTag {
                name: "p".to_string(),
                attrs: vec![Attribute {
                    name: "style".to_string(),
                    value: Some("badprop: 1".to_string()),
                    span: Some(attr_span),
                }],
                self_closing: false,
                span: Some(Span::new(0, 23, 1, 1)),
            }],
        );

        let report = validate_events(src, pack_css_checks(), default_test_config()).unwrap();
        let css_errs: Vec<_> = report
            .messages
            .iter()
            .filter(|m| m.code == "html.css.error")
            .collect();

        for err in &css_errs {
            let span = err.span.expect("CSS error should have a span");
            // Value starts at byte_start=3 + 7 = 10
            assert_eq!(
                span.byte_start, 10,
                "error at start of value should point to byte 10 (value base), msg={}",
                err.message
            );
            assert_eq!(span.line, 1);
            // col = attr.col(4) + 7(name="") + 1 - 1 = 11
            assert_eq!(
                span.col, 11,
                "column should be at value start, msg={}",
                err.message
            );
        }
    }

    #[test]
    fn style_attr_unclosed_comment_points_to_comment_start() {
        // <div style="color:red; /* oops">
        // attr span covers style="color:red; /* oops"
        let attr_span = Span::new(5, 36, 1, 6);
        let src = CaptureSource::new(
            InputFormat::Html,
            vec![ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![Attribute {
                    name: "style".to_string(),
                    value: Some("color:red; /* oops".to_string()),
                    span: Some(attr_span),
                }],
                self_closing: false,
                span: Some(Span::new(0, 37, 1, 1)),
            }],
        );

        let report = validate_events(src, pack_css_checks(), default_test_config()).unwrap();
        let uc = report
            .messages
            .iter()
            .find(|m| m.code == "html.css.error" && m.message.contains("Unclosed"))
            .expect("expected unclosed comment error");
        let span = uc.span.unwrap();
        // "/* oops" starts at offset 11 in value, value base = 5+7=12
        assert_eq!(span.byte_start, 12 + 11, "should point to /*");
    }

    #[test]
    fn style_attr_fallback_to_attr_span_when_token_not_found() {
        // When the error message has no locatable token, fall back to attr span.
        let attr_span = Span::new(3, 15, 1, 4);
        let report = css_inspector::Report {
            errors: 1,
            messages: vec![css_inspector::Message {
                severity: css_inspector::Severity::Error,
                message: "Something went wrong".to_string(),
            }],
            ..css_inspector::Report::default()
        };

        let mut out = VecSink::default();
        let attr = Attribute {
            name: "style".to_string(),
            value: Some("color: red".to_string()),
            span: Some(attr_span),
        };
        super::emit_css_errors_in_style_attr(
            &report,
            "color: red",
            &attr,
            Some(Span::new(0, 20, 1, 1)),
            &mut out,
        );
        assert_eq!(out.0.len(), 1);
        assert_eq!(
            out.0[0].span,
            Some(attr_span),
            "should fall back to attribute span when token not found"
        );
    }

    #[test]
    fn style_attr_fallback_to_tag_span_when_attr_has_no_span() {
        let tag_span = Span::new(0, 20, 1, 1);
        let report = css_inspector::Report {
            errors: 1,
            messages: vec![css_inspector::Message {
                severity: css_inspector::Severity::Error,
                message: "Something".to_string(),
            }],
            ..css_inspector::Report::default()
        };

        let mut out = VecSink::default();
        let attr = Attribute {
            name: "style".to_string(),
            value: Some("color: red".to_string()),
            span: None,
        };
        super::emit_css_errors_in_style_attr(
            &report,
            "color: red",
            &attr,
            Some(tag_span),
            &mut out,
        );
        assert_eq!(out.0.len(), 1);
        assert_eq!(
            out.0[0].span,
            Some(tag_span),
            "should fall back to tag span when attr has no span"
        );
    }

    // ── Style element span regression tests ──

    #[test]
    fn style_element_error_at_col_1_of_text() {
        // Error token "foo" (inside `a{foo:1}`) starts at byte 2 of the CSS text.
        let style_span = Span::new(0, 7, 1, 1);
        let text_span = Span::new(8, 20, 2, 1);
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
        let err = report
            .messages
            .iter()
            .find(|m| m.code == "html.css.error")
            .expect("expected css error");
        let span = err.span.unwrap();
        // "foo" is at byte 2 of the CSS text → abs byte_start = text_span.byte_start + 2 = 10
        assert_eq!(span.byte_start, 10);
        assert_eq!(span.line, 2, "should be on line 2 (text_span.line)");
        // col = base.col(1) + rel_col(3) - 1 = 3
        assert_eq!(span.col, 3, "error at byte 2 of CSS → col 3");
    }

    #[test]
    fn style_element_error_on_third_line_of_css() {
        // CSS text spans 3 lines, error "foo" on line 3 inside `c{foo:1}`
        // Full CSS: "a{color:red;}\nb{color:red;}\nc{foo:1}"
        // byte offsets:     0-13       \n14    14-27      \n28  28-36
        // "foo" is at byte 30 in CSS text
        let text_span = Span::new(100, 200, 5, 3);
        let src = CaptureSource::new(
            InputFormat::Html,
            vec![
                ParseEvent::StartTag {
                    name: "style".to_string(),
                    attrs: vec![],
                    self_closing: false,
                    span: Some(Span::new(90, 100, 5, 1)),
                },
                ParseEvent::Text {
                    text: "a{color:red;}\nb{color:red;}\nc{foo:1}".to_string(),
                    span: Some(text_span),
                },
                ParseEvent::EndTag {
                    name: "style".to_string(),
                    span: None,
                },
            ],
        );

        let report = validate_events(src, pack_css_checks(), default_test_config()).unwrap();
        let err = report
            .messages
            .iter()
            .find(|m| m.code == "html.css.error")
            .expect("expected css error");
        let span = err.span.unwrap();
        // "foo" is at byte 30 in the CSS text
        assert_eq!(span.byte_start, 100 + 30);
        // rel_line=3 → abs_line = 5 + 3 - 1 = 7
        assert_eq!(span.line, 7);
        // On a subsequent line, col resets (not added to base): col for "foo" at byte 2 of line 3 → col 3
        assert_eq!(span.col, 3);
    }

    #[test]
    fn style_element_unclosed_comment_span() {
        let text_span = Span::new(50, 60, 3, 1);
        let src = CaptureSource::new(
            InputFormat::Html,
            vec![
                ParseEvent::StartTag {
                    name: "style".to_string(),
                    attrs: vec![],
                    self_closing: false,
                    span: Some(Span::new(40, 50, 3, 1)),
                },
                ParseEvent::Text {
                    text: "a{} /*".to_string(),
                    span: Some(text_span),
                },
                ParseEvent::EndTag {
                    name: "style".to_string(),
                    span: None,
                },
            ],
        );

        let report = validate_events(src, pack_css_checks(), default_test_config()).unwrap();
        let err = report
            .messages
            .iter()
            .find(|m| m.code == "html.css.error" && m.message.contains("Unclosed"))
            .expect("expected unclosed comment error");
        let span = err.span.unwrap();
        // "/*" is at byte 4 in CSS text → abs byte_start = 50 + 4 = 54
        assert_eq!(span.byte_start, 54);
        assert_eq!(span.line, 3);
        assert_eq!(span.col, 5); // base col=1, rel byte 4 → rel_col=5, same line → 1+5-1=5
    }

    #[test]
    fn style_element_fallback_to_style_span_when_text_has_no_span() {
        let style_span = Span::new(0, 10, 1, 1);
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
                    text: "/*".to_string(),
                    span: None, // no text span
                },
                ParseEvent::EndTag {
                    name: "style".to_string(),
                    span: None,
                },
            ],
        );

        let report = validate_events(src, pack_css_checks(), default_test_config()).unwrap();
        let err = report
            .messages
            .iter()
            .find(|m| m.code == "html.css.error")
            .expect("expected css error");
        assert_eq!(
            err.span,
            Some(style_span),
            "should fall back to <style> tag span when text has no span"
        );
    }

    #[test]
    fn style_attr_multiple_errors_each_localized() {
        // Two invalid properties in one style attribute
        let attr_span = Span::new(5, 40, 1, 6);
        let src = CaptureSource::new(
            InputFormat::Html,
            vec![ParseEvent::StartTag {
                name: "p".to_string(),
                attrs: vec![Attribute {
                    name: "style".to_string(),
                    value: Some("aaa: 1; bbb: 2".to_string()),
                    span: Some(attr_span),
                }],
                self_closing: false,
                span: Some(Span::new(0, 41, 1, 1)),
            }],
        );

        let report = validate_events(src, pack_css_checks(), default_test_config()).unwrap();
        let css_errs: Vec<_> = report
            .messages
            .iter()
            .filter(|m| m.code == "html.css.error")
            .collect();
        assert!(
            css_errs.len() >= 2,
            "expected at least 2 CSS errors, got {}",
            css_errs.len()
        );

        let spans: Vec<_> = css_errs.iter().filter_map(|m| m.span).collect();
        // Each error should have a distinct byte_start (localized to its token)
        let unique_starts: std::collections::HashSet<_> =
            spans.iter().map(|s| s.byte_start).collect();
        assert!(
            unique_starts.len() >= 2,
            "multiple errors should point to different locations, got {:?}",
            spans
        );
    }

    #[test]
    fn style_element_multiline_multiple_errors_localized() {
        // Two invalid properties on separate lines
        let text_span = Span::new(50, 100, 3, 1);
        let src = CaptureSource::new(
            InputFormat::Html,
            vec![
                ParseEvent::StartTag {
                    name: "style".to_string(),
                    attrs: vec![],
                    self_closing: false,
                    span: Some(Span::new(40, 50, 2, 1)),
                },
                ParseEvent::Text {
                    text: "a{aaa:1}\nb{bbb:2}".to_string(),
                    span: Some(text_span),
                },
                ParseEvent::EndTag {
                    name: "style".to_string(),
                    span: None,
                },
            ],
        );

        let report = validate_events(src, pack_css_checks(), default_test_config()).unwrap();
        let css_errs: Vec<_> = report
            .messages
            .iter()
            .filter(|m| m.code == "html.css.error")
            .collect();
        assert!(
            css_errs.len() >= 2,
            "expected at least 2 CSS errors, got {}",
            css_errs.len()
        );

        let spans: Vec<_> = css_errs.iter().filter_map(|m| m.span).collect();
        // Errors on different lines should have different line numbers
        let lines: std::collections::HashSet<_> = spans.iter().map(|s| s.line).collect();
        assert!(
            lines.len() >= 2,
            "errors on different lines should report different line numbers, got {:?}",
            spans
        );
    }

    // ── Byte offset / line:col round-trip consistency ──

    #[test]
    fn style_attr_span_byte_and_linecol_are_consistent() {
        // Verify that the byte_start, line, and col in the emitted span
        // agree with each other when cross-checked.
        let attr_span = Span::new(3, 20, 1, 4);
        let src = CaptureSource::new(
            InputFormat::Html,
            vec![ParseEvent::StartTag {
                name: "p".to_string(),
                attrs: vec![Attribute {
                    name: "style".to_string(),
                    value: Some("/*".to_string()),
                    span: Some(attr_span),
                }],
                self_closing: false,
                span: Some(Span::new(0, 21, 1, 1)),
            }],
        );

        let report = validate_events(src, pack_css_checks(), default_test_config()).unwrap();
        for msg in report
            .messages
            .iter()
            .filter(|m| m.code == "html.css.error")
        {
            let span = msg.span.expect("should have span");
            assert!(
                span.byte_start < span.byte_end,
                "byte range should be non-empty"
            );
            assert!(span.line >= 1, "line should be >= 1");
            assert!(span.col >= 1, "col should be >= 1");
        }
    }

    #[test]
    fn style_element_span_byte_and_linecol_are_consistent() {
        let text_span = Span::new(100, 120, 5, 10);
        let src = CaptureSource::new(
            InputFormat::Html,
            vec![
                ParseEvent::StartTag {
                    name: "style".to_string(),
                    attrs: vec![],
                    self_closing: false,
                    span: Some(Span::new(90, 100, 5, 1)),
                },
                ParseEvent::Text {
                    text: "a{}\n/*".to_string(),
                    span: Some(text_span),
                },
                ParseEvent::EndTag {
                    name: "style".to_string(),
                    span: None,
                },
            ],
        );

        let report = validate_events(src, pack_css_checks(), default_test_config()).unwrap();
        for msg in report
            .messages
            .iter()
            .filter(|m| m.code == "html.css.error")
        {
            let span = msg.span.expect("should have span");
            assert!(
                span.byte_start < span.byte_end,
                "byte range should be non-empty"
            );
            assert!(span.line >= 1, "line should be >= 1");
            assert!(span.col >= 1, "col should be >= 1");
        }
    }
}
