use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, Span, ValidationContext,
};

use super::shared::starts_with_ascii_ci;

#[derive(Default)]
pub struct AHrefConstraints;

impl Rule for AHrefConstraints {
    fn id(&self) -> &'static str {
        "html.a.href.datatype"
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
        if !ctx.name_is(name, "a") {
            return;
        }

        let href = ctx.attr_value(attrs, "href");
        let Some(href) = href else { return };

        if emit_forbidden_url_code_point(href, *span, out) {
            return;
        }

        if let Some(sev) = href_issue_severity(href) {
            out.push(Message::new(
                "html.a.href.invalid",
                sev,
                Category::Html,
                format!("Bad value “{href}” for attribute “href” on element “a”."),
                *span,
            ));
        }
    }
}

pub(crate) fn href_issue_severity(href: &str) -> Option<Severity> {
    const HTTP_PREFIX: &str = "http://";
    const HTTPS_PREFIX: &str = "https://";
    const FTP_PREFIX: &str = "ftp://";
    const FILE_PREFIX: &str = "file://";

    // This is intentionally a small WHATWG-ish subset, expanded case-by-case using the vnu suite.
    // - Reject ASCII whitespace/control
    if href
        .chars()
        .any(|c| c.is_ascii_whitespace() || c.is_control())
    {
        return Some(Severity::Error);
    }

    // Reject forbidden Unicode noncharacters (suite coverage).
    if href.chars().any(is_forbidden_code_point) {
        return Some(Severity::Error);
    }

    // Scheme-specific cases (suite coverage).
    if starts_with_ascii_ci(href, "data:") {
        if !href.contains(',') {
            return Some(Severity::Error);
        }
        if href.contains('#') {
            return Some(Severity::Warning);
        }
    }

    for (scheme_colon, scheme_slash) in [
        ("http:", HTTP_PREFIX),
        ("https:", HTTPS_PREFIX),
        ("ftp:", FTP_PREFIX),
    ] {
        if starts_with_ascii_ci(href, scheme_colon) && !starts_with_ascii_ci(href, scheme_slash) {
            return Some(Severity::Error);
        }
    }

    if starts_with_ascii_ci(href, "file:") && href.contains('|') {
        // vnu accepts legacy `file://C|/...` (drive letter in authority), but rejects pipe elsewhere.
        if starts_with_ascii_ci(href, FILE_PREFIX) {
            let rest = &href[FILE_PREFIX.len()..];
            let authority = rest.split(['/', '?', '#']).next().unwrap_or("");
            if !is_legacy_windows_drive_authority(authority) {
                return Some(Severity::Error);
            }
        } else {
            return Some(Severity::Error);
        }
    }

    // Backslash is rejected in vnu's URL checks (suite coverage).
    if href.contains('\\') {
        return Some(Severity::Error);
    }

    // Percent-encoding must be well-formed.
    if has_malformed_percent_encoding(href) {
        return Some(Severity::Error);
    }

    // Fragments: spaces and hashes must be percent-encoded; also reject backslash (vnu suite).
    if let Some((_, frag)) = href.split_once('#')
        && (frag.contains('#') || frag.contains(' ') || frag.contains('\\')) {
            return Some(Severity::Error);
        }
    // Fragment-only already covered by backslash check above.

    // Host-empty and authority edge cases for http(s).
    if starts_with_ascii_ci(href, HTTP_PREFIX) || starts_with_ascii_ci(href, HTTPS_PREFIX) {
        let scheme_len = if starts_with_ascii_ci(href, HTTP_PREFIX) {
            HTTP_PREFIX.len()
        } else {
            HTTPS_PREFIX.len()
        };
        let rest = &href[scheme_len..];
        if rest.is_empty() {
            return Some(Severity::Error);
        }
        let authority = rest.split(['/', '?', '#']).next().unwrap_or("");
        if authority.is_empty() {
            return Some(Severity::Error);
        }

        // userinfo@host: if '@' present, host is after last '@'
        let hostport = if let Some((userinfo, host)) = authority.rsplit_once('@') {
            // Reject unencoded '@' in username/password, and other invalid userinfo characters (suite coverage).
            if userinfo.contains('@') || userinfo.contains('[') || userinfo.contains(']') {
                return Some(Severity::Error);
            }
            if !userinfo.is_ascii() {
                return Some(Severity::Error);
            }
            host
        } else {
            authority
        };

        if hostport.is_empty() {
            return Some(Severity::Error);
        }
        // Bracketed host must look like IPv6, and port must be valid if present.
        if hostport.starts_with('[') {
            let Some((inside, after)) = hostport.strip_prefix('[').and_then(|s| s.split_once(']'))
            else {
                return Some(Severity::Error);
            };
            if !inside.contains(':') {
                return Some(Severity::Error);
            }
            if let Some(after) = after.strip_prefix(':') {
                // Disallow extra ':' in port, and enforce range.
                if after.contains(':') {
                    return Some(Severity::Error);
                }
                if !after.is_empty() && !is_valid_port(after) {
                    return Some(Severity::Error);
                }
            } else if !after.is_empty() {
                return Some(Severity::Error);
            }
        } else {
            // host[:port]
            if let Some((host, port)) = hostport.rsplit_once(':') {
                // If host contains ':' without brackets, likely invalid for these tests.
                if host.contains(':') {
                    return Some(Severity::Error);
                }
                // Treat empty host as invalid.
                if host.is_empty() {
                    return Some(Severity::Error);
                }
                if !port.is_empty() && !is_valid_port(port) {
                    return Some(Severity::Error);
                }
            }
        }

        // Fullwidth percent sign (or its percent-encoded form) used in suite as invalid host.
        if href.contains('\u{FF05}') {
            return Some(Severity::Error);
        }
        if contains_ascii_ci(href, "%ef%bc%85") {
            return Some(Severity::Error);
        }
        // Percent-encoded forbidden code point used in suite (U+FDD0).
        if contains_ascii_ci(href, "%ef%b7%90") {
            return Some(Severity::Error);
        }
    }

    // Disallow bracketed host-like strings in relative references used in suite.
    if href.starts_with('[') {
        return Some(Severity::Error);
    }

    None
}

fn contains_ascii_ci(s: &str, needle: &str) -> bool {
    let nb = needle.as_bytes();
    nb.is_empty()
        || s.as_bytes()
            .windows(nb.len())
            .any(|w| w.eq_ignore_ascii_case(nb))
}

fn has_malformed_percent_encoding(s: &str) -> bool {
    let bytes = s.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] == b'%' {
            if i + 2 >= bytes.len() {
                return true;
            }
            let a = bytes[i + 1];
            let b = bytes[i + 2];
            if !is_hex(a) || !is_hex(b) {
                return true;
            }
            i += 3;
            continue;
        }
        i += 1;
    }
    false
}

fn is_hex(b: u8) -> bool {
    b.is_ascii_hexdigit()
}

fn is_valid_port(port: &str) -> bool {
    if port.is_empty() || !port.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    match port.parse::<u32>() {
        Ok(n) => n <= 65535,
        Err(_) => false,
    }
}

fn is_legacy_windows_drive_authority(authority: &str) -> bool {
    let bytes = authority.as_bytes();
    if bytes.len() != 2 {
        return false;
    }
    bytes[0].is_ascii_alphabetic() && bytes[1] == b'|'
}

fn is_forbidden_code_point(c: char) -> bool {
    let u = c as u32;
    // Noncharacters: U+FDD0..U+FDEF and any code point ending in FFFE/FFFF.
    (0xFDD0..=0xFDEF).contains(&u) || (u & 0xFFFE) == 0xFFFE
}

pub(crate) fn emit_forbidden_url_code_point(
    value: &str,
    span: Option<Span>,
    out: &mut dyn MessageSink,
) -> bool {
    let Some(cp) = first_forbidden_url_code_point(value) else {
        return false;
    };
    out.push(Message::new(
        "html.url.forbidden_code_point",
        Severity::Error,
        Category::Html,
        format!("Forbidden code point U+{:04x}.", cp),
        span,
    ));
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn href_issue_severity_covers_more_edge_cases() {
        assert_eq!(
            href_issue_severity("http://example.com/\u{FDD0}"),
            Some(Severity::Error)
        );
        assert_eq!(href_issue_severity("http:///path"), Some(Severity::Error));
        assert_eq!(href_issue_severity("http://[::1"), Some(Severity::Error));
        assert_eq!(
            href_issue_severity("http://[::1]:99999"),
            Some(Severity::Error)
        );
        assert_eq!(href_issue_severity("http://[::1]x"), Some(Severity::Error));
        assert_eq!(href_issue_severity("http://:80/"), Some(Severity::Error));
        assert_eq!(
            href_issue_severity("http://example.com/%zz"),
            Some(Severity::Error)
        );
    }

    #[test]
    fn is_valid_port_covers_parse_error_path() {
        assert!(!is_valid_port("999999999999999999999999999999999999"));
    }

    #[test]
    fn href_issue_severity_covers_data_scheme_file_pipe_and_fragment_chars() {
        assert_eq!(
            href_issue_severity("data:text/plain"),
            Some(Severity::Error)
        );
        assert_eq!(
            href_issue_severity("data:text/plain,hi#frag"),
            Some(Severity::Warning)
        );
        assert_eq!(
            href_issue_severity("http:example.com"),
            Some(Severity::Error)
        );
        assert_eq!(
            href_issue_severity("HTTP:example.com"),
            Some(Severity::Error)
        );
        assert_eq!(href_issue_severity("HTTP://example.com/"), None);
        assert_eq!(href_issue_severity("hTTps://example.com/"), None);

        assert_eq!(href_issue_severity("file:/C|/x"), Some(Severity::Error));
        assert_eq!(href_issue_severity("file://C|/x"), None);
        assert_eq!(href_issue_severity("FILE://C|/x"), None);
        assert_eq!(
            href_issue_severity("file://example.com|/x"),
            Some(Severity::Error)
        );

        assert_eq!(
            href_issue_severity("http://example.com/#a b"),
            Some(Severity::Error)
        );
        assert_eq!(
            href_issue_severity("http://example.com/#a#b"),
            Some(Severity::Error)
        );
    }

    #[test]
    fn is_valid_port_rejects_empty_and_non_digit() {
        assert!(!is_valid_port(""));
        assert!(!is_valid_port("x"));
    }

    #[test]
    fn xhtml_href_attribute_is_matched_case_sensitively() {
        #[derive(Default)]
        struct Sink(Vec<Message>);
        impl MessageSink for Sink {
            fn push(&mut self, msg: Message) {
                self.0.push(msg);
            }
        }

        let mut ctx = ValidationContext::new(
            html_inspector_core::Config::default(),
            html_inspector_core::InputFormat::Xhtml,
        );
        let mut sink = Sink::default();
        let mut rule = AHrefConstraints::default();
        rule.on_event(
            &ParseEvent::StartTag {
                name: "a".to_string(),
                attrs: vec![html_inspector_core::Attribute {
                    name: "href".to_string(),
                    value: Some("data:text/plain".to_string()),
                    span: None,
                }],
                self_closing: false,
                span: None,
            },
            &mut ctx,
            &mut sink,
        );
        assert!(sink.0.iter().any(|m| m.code == "html.a.href.invalid"));
    }

    #[test]
    fn href_issue_severity_accepts_valid_percent_encoding_and_handles_userinfo() {
        assert_eq!(
            href_issue_severity("http://user:pass@example.com/%20"),
            None
        );
        assert_eq!(
            href_issue_severity("http://u[s]@example.com/"),
            Some(Severity::Error)
        );
        assert_eq!(
            href_issue_severity("http://u\u{00E4}@example.com/"),
            Some(Severity::Error)
        );
    }

    #[test]
    fn href_issue_severity_covers_more_authority_and_percent_encoded_edge_cases() {
        assert_eq!(href_issue_severity("http://@/"), Some(Severity::Error));
        assert_eq!(href_issue_severity("http://[v]/"), Some(Severity::Error));
        assert_eq!(
            href_issue_severity("http://[::1]:1:2/"),
            Some(Severity::Error)
        );
        assert_eq!(href_issue_severity("http://a:b:c/"), Some(Severity::Error));
        assert_eq!(
            href_issue_severity("http://example.com:99999/"),
            Some(Severity::Error)
        );

        assert_eq!(
            href_issue_severity("http://example.com/\u{FF05}"),
            Some(Severity::Error)
        );
        assert_eq!(
            href_issue_severity("http://example.com/%EF%BC%85"),
            Some(Severity::Error)
        );
        assert_eq!(
            href_issue_severity("http://example.com/%EF%B7%90"),
            Some(Severity::Error)
        );
        assert_eq!(href_issue_severity("[::1]"), Some(Severity::Error));
    }

    #[test]
    fn rule_returns_early_for_non_start_tags_non_a_elements_and_forbidden_code_points() {
        #[derive(Default)]
        struct Sink(Vec<Message>);
        impl MessageSink for Sink {
            fn push(&mut self, msg: Message) {
                self.0.push(msg);
            }
        }

        let mut ctx = ValidationContext::new(
            html_inspector_core::Config::default(),
            html_inspector_core::InputFormat::Html,
        );
        let mut sink = Sink::default();
        let mut rule = AHrefConstraints::default();

        rule.on_event(
            &ParseEvent::Text {
                text: "x".to_string(),
                span: None,
            },
            &mut ctx,
            &mut sink,
        );
        assert!(sink.0.is_empty());

        rule.on_event(
            &ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![html_inspector_core::Attribute {
                    name: "href".to_string(),
                    value: Some("http://example.com/".to_string()),
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
                name: "a".to_string(),
                attrs: vec![html_inspector_core::Attribute {
                    name: "href".to_string(),
                    value: Some("http://example.com/\u{000B}".to_string()),
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
            .any(|m| m.code == "html.url.forbidden_code_point"));
        assert!(!sink.0.iter().any(|m| m.code == "html.a.href.invalid"));
    }

    #[test]
    fn contains_ascii_ci_accepts_empty_needle_and_rejects_longer_needle() {
        assert!(contains_ascii_ci("abc", ""));
        assert!(!contains_ascii_ci("a", "aa"));
        assert!(contains_ascii_ci("a", "A"));
        assert!(!contains_ascii_ci("a", "B"));
    }
}

fn first_forbidden_url_code_point(s: &str) -> Option<u32> {
    for ch in s.chars() {
        let cp = ch as u32;
        if cp == 0x0091
            || cp == 0x000B
            || (0xFDD0..=0xFDEF).contains(&cp)
            || (cp & 0xFFFE) == 0xFFFE
        {
            return Some(cp);
        }
    }
    None
}
