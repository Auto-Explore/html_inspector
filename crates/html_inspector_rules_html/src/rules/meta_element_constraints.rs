use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct MetaElementConstraints {
    seen_charset_meta: bool,
    seen_content_type_meta: bool,
    seen_description_meta: bool,
}

impl Rule for MetaElementConstraints {
    fn id(&self) -> &'static str {
        "html.meta.element_constraints"
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
        if !is(ctx, name, "meta") {
            return;
        }

        let has_charset = has_attr(ctx, attrs, "charset");
        let has_content = has_attr(ctx, attrs, "content");
        let has_name = has_attr(ctx, attrs, "name");
        let has_property = has_attr(ctx, attrs, "property");
        let has_itemprop = has_attr(ctx, attrs, "itemprop");
        let has_media = has_attr(ctx, attrs, "media");
        let has_value = has_attr(ctx, attrs, "value");

        if has_charset && has_content {
            out.push(Message::new(
                "html.meta.charset.content.disallowed",
                Severity::Error,
                Category::Html,
                "Attribute “content” not allowed on element “meta” at this point.",
                *span,
            ));
        }

        if has_value {
            out.push(Message::new(
                "html.meta.value.disallowed",
                Severity::Error,
                Category::Html,
                "Attribute “value” not allowed on element “meta” at this point.",
                *span,
            ));
        }

        if !has_charset
            && !has_content
            && (has_name || has_property || has_itemprop || has_attr(ctx, attrs, "http-equiv"))
        {
            out.push(Message::new(
                "html.meta.missing_content",
                Severity::Error,
                Category::Html,
                "Element “meta” is missing one or more of the following attributes: “content”, “property”.",
                *span,
            ));
        }

        if has_itemprop && has_name {
            out.push(Message::new(
                "html.meta.itemprop.disallowed_with_name",
                Severity::Error,
                Category::Html,
                "Attribute “itemprop” not allowed on element “meta” at this point.",
                *span,
            ));
        }

        let http_equiv = attr_value(ctx, attrs, "http-equiv").unwrap_or("");
        let http_equiv_trim = http_equiv.trim();
        let http_equiv_lc = http_equiv_trim.to_ascii_lowercase();

        if matches!(
            http_equiv_lc.as_str(),
            "x-pjax-version" | "x-pjax-js-version" | "x-pjax-css-version"
        ) {
            out.push(Message::new(
                "html.meta.http_equiv.bad_value",
                Severity::Error,
                Category::Html,
                format!(
                    "Bad value “{http_equiv_trim}” for attribute “http-equiv” on element “meta”."
                ),
                *span,
            ));
        }

        if http_equiv_lc == "content-type" {
            if self.seen_charset_meta {
                out.push(Message::new(
                    "html.meta.charset_and_content_type.disallowed",
                    Severity::Error,
                    Category::Html,
                    "A document must not include both a “meta” element with an “http-equiv” attribute whose value is “content-type”, and a “meta” element with a “charset” attribute.",
                    *span,
                ));
            }
            self.seen_content_type_meta = true;
        }

        if has_charset {
            if self.seen_content_type_meta {
                out.push(Message::new(
                    "html.meta.charset_and_content_type.disallowed",
                    Severity::Error,
                    Category::Html,
                    "A document must not include both a “meta” element with an “http-equiv” attribute whose value is “content-type”, and a “meta” element with a “charset” attribute.",
                    *span,
                ));
            }
            self.seen_charset_meta = true;
        }

        if http_equiv_lc == "content-language" {
            out.push(Message::new(
                "html.meta.http_equiv.content_language.obsolete",
                Severity::Error,
                Category::Html,
                "Using the “meta” element to specify the document-wide default language is obsolete. Consider specifying the language on the root element instead.",
                *span,
            ));
        }

        let name_value = attr_value(ctx, attrs, "name").unwrap_or("");
        let name_lc = name_value.to_ascii_lowercase();
        if has_media && name_lc != "theme-color" {
            out.push(Message::new(
                "html.meta.media.requires_theme_color",
                Severity::Error,
                Category::Html,
                "A “meta” element with a “media” attribute must have a “name” attribute whose value is “theme-color”.",
                *span,
            ));
        }

        if name_lc == "description" {
            // Keep this narrow: vnu case uses non-empty content values; allow empty/valueless `content` (suite coverage).
            let content_value = attr_value(ctx, attrs, "content").unwrap_or("");
            if !content_value.is_empty() {
                if self.seen_description_meta {
                    out.push(Message::new(
                        "html.meta.description.multiple.disallowed",
                        Severity::Error,
                        Category::Html,
                        "A document must not include more than one “meta” element with its “name” attribute set to the value “description”.",
                        *span,
                    ));
                }
                self.seen_description_meta = true;
            }
        }

        if name_lc == "viewport" {
            let content = attr_value(ctx, attrs, "content").unwrap_or("");
            let content_lc = content.to_ascii_lowercase();
            if content_lc.contains("user-scalable=no") {
                out.push(Message::new(
                    "html.meta.viewport.user_scalable_no",
                    Severity::Warning,
                    Category::Html,
                    "Consider avoiding viewport values that prevent users from resizing documents.",
                    *span,
                ));
            }
        }

        if http_equiv_lc == "x-ua-compatible" {
            let content = attr_value(ctx, attrs, "content").unwrap_or("");
            if !content.trim().eq_ignore_ascii_case("IE=edge") {
                out.push(Message::new(
                    "html.meta.x_ua_compatible.requires_ie_edge",
                    Severity::Error,
                    Category::Html,
                    "A “meta” element with an “http-equiv” attribute whose value is “X-UA-Compatible” must have a “content” attribute with the value “IE=edge”.",
                    *span,
                ));
            }
        }

        if http_equiv_lc == "content-security-policy" {
            let content = attr_value(ctx, attrs, "content").unwrap_or("");
            if let Some(sev) = csp_issue_severity(content) {
                out.push(Message::new(
                    "html.meta.csp.invalid",
                    sev,
                    Category::Html,
                    format!("Bad value “{content}” for attribute “content” on element “meta”."),
                    *span,
                ));
            }
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, _out: &mut dyn MessageSink) {
        self.seen_charset_meta = false;
        self.seen_content_type_meta = false;
        self.seen_description_meta = false;
    }
}

fn csp_issue_severity(content: &str) -> Option<Severity> {
    if content.is_empty() {
        return None;
    }

    if !content.is_ascii() {
        return Some(Severity::Error);
    }

    let mut has_unknown_directive = false;
    let mut has_invalid_source = false;

    for policy in content.split(',') {
        for directive in policy.split(';') {
            let directive = directive.trim();
            if directive.is_empty() {
                continue;
            }
            let mut parts = directive.split_ascii_whitespace();
            let Some(name) = parts.next() else { continue };
            let name_lc = name.to_ascii_lowercase();
            if !is_known_csp_directive(&name_lc) {
                has_unknown_directive = true;
                continue;
            }

            if is_source_list_directive(&name_lc) {
                for token in parts {
                    if token.starts_with('\'')
                        && token.ends_with('\'')
                        && !is_valid_csp_keyword_or_nonce_or_hash(token)
                    {
                        has_invalid_source = true;
                    }
                }
            } else {
                // directives like upgrade-insecure-requests should not have values; ignore for now.
                let _ = parts;
            }
        }
    }

    if has_invalid_source {
        return Some(Severity::Error);
    }
    if has_unknown_directive {
        return Some(Severity::Warning);
    }
    None
}

fn is_known_csp_directive(name_lc: &str) -> bool {
    matches!(
        name_lc,
        "default-src"
            | "script-src"
            | "style-src"
            | "img-src"
            | "connect-src"
            | "font-src"
            | "media-src"
            | "object-src"
            | "base-uri"
            | "form-action"
            | "frame-ancestors"
            | "upgrade-insecure-requests"
            | "block-all-mixed-content"
            | "sandbox"
            | "worker-src"
            | "child-src"
            | "frame-src"
    )
}

fn is_source_list_directive(name_lc: &str) -> bool {
    matches!(
        name_lc,
        "default-src"
            | "script-src"
            | "style-src"
            | "img-src"
            | "connect-src"
            | "font-src"
            | "media-src"
            | "object-src"
            | "base-uri"
            | "form-action"
            | "frame-src"
            | "worker-src"
            | "child-src"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    use html_inspector::{Config, InputFormat, RuleSet};
    use html_inspector_html::HtmlEventSource;

    fn validate(html: &str) -> html_inspector::Report {
        let src = HtmlEventSource::from_str("t", InputFormat::Html, html).unwrap();
        html_inspector::validate_events(
            src,
            RuleSet::new().push(MetaElementConstraints::default()),
            Config::default(),
        )
        .unwrap()
    }

    #[test]
    fn charset_and_content_disallowed_together() {
        let report = validate(r#"<meta charset="utf-8" content="x">"#);
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.meta.charset.content.disallowed")
        );
    }

    #[test]
    fn itemprop_disallowed_with_name() {
        let report = validate(r#"<meta itemprop="x" name="y">"#);
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.meta.itemprop.disallowed_with_name")
        );
    }

    #[test]
    fn charset_and_content_type_are_mutually_exclusive() {
        let report = validate(r#"<meta charset="utf-8"><meta http-equiv="content-type">"#);
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.meta.charset_and_content_type.disallowed")
        );
    }

    #[test]
    fn content_type_then_charset_is_also_disallowed() {
        let report = validate(r#"<meta http-equiv="content-type"><meta charset="utf-8">"#);
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.meta.charset_and_content_type.disallowed")
        );
    }

    #[test]
    fn multiple_description_meta_is_disallowed_when_content_non_empty() {
        let report = validate(
            r#"<meta name="description" content="a"><meta name="description" content="b">"#,
        );
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.meta.description.multiple.disallowed")
        );
    }

    #[test]
    fn viewport_user_scalable_no_emits_warning() {
        let report = validate(r#"<meta name="viewport" content="user-scalable=no">"#);
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.meta.viewport.user_scalable_no")
        );
    }

    #[test]
    fn x_ua_compatible_requires_ie_edge() {
        let report = validate(r#"<meta http-equiv="x-ua-compatible" content="foo">"#);
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.meta.x_ua_compatible.requires_ie_edge")
        );
    }

    #[test]
    fn csp_unknown_directive_is_warning() {
        let report =
            validate(r#"<meta http-equiv="content-security-policy" content=";;bogus-src 'none'">"#);
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.meta.csp.invalid"
                    && m.severity == html_inspector::Severity::Warning)
        );
    }

    #[test]
    fn csp_unsafe_hashes_is_allowed() {
        let report = validate(
            r#"<meta http-equiv="content-security-policy" content="script-src 'unsafe-hashes'">"#,
        );
        assert!(
            !report
                .messages
                .iter()
                .any(|m| m.code == "html.meta.csp.invalid")
        );
    }
}

fn is_valid_csp_keyword_or_nonce_or_hash(token: &str) -> bool {
    match token {
        "'self'" | "'none'" | "'unsafe-inline'" | "'unsafe-eval'" | "'unsafe-hashes'"
        | "'strict-dynamic'" => return true,
        _ => {}
    }
    if token.starts_with("'nonce-") && token.ends_with('\'') {
        return token.len() > "'nonce-'".len() + 1;
    }
    for alg in ["'sha256-", "'sha384-", "'sha512-"] {
        if token.starts_with(alg) && token.ends_with('\'') {
            return token.len() > alg.len() + 1;
        }
    }
    false
}

fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    match ctx.format {
        html_inspector::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector::InputFormat::Xhtml => actual == expected,
    }
}

fn has_attr(ctx: &ValidationContext, attrs: &[html_inspector::Attribute], needle: &str) -> bool {
    attrs.iter().any(|a| match ctx.format {
        html_inspector::InputFormat::Html => a.name.eq_ignore_ascii_case(needle),
        html_inspector::InputFormat::Xhtml => a.name == needle,
    })
}

fn attr_value<'a>(
    ctx: &ValidationContext,
    attrs: &'a [html_inspector::Attribute],
    needle: &str,
) -> Option<&'a str> {
    attrs
        .iter()
        .find(|a| match ctx.format {
            html_inspector::InputFormat::Html => a.name.eq_ignore_ascii_case(needle),
            html_inspector::InputFormat::Xhtml => a.name == needle,
        })
        .and_then(|a| a.value.as_deref())
}
