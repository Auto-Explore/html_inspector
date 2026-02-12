use std::collections::HashMap;

use base64::Engine;
use html_inspector_core::ValidationContext;
use sha2::{Digest, Sha256, Sha384, Sha512};
use url::Url;

#[derive(Clone, Copy, Debug)]
pub(crate) enum CspKind {
    ScriptEventHandler,
    InlineScript,
    InlineStyle,
    StyleAttribute,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum CspExternalKind {
    Script,
    Style,
    Image,
    Frame,
    Object,
    Media,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct CspPolicy {
    pub(crate) directives: HashMap<String, Vec<String>>,
}

impl CspPolicy {
    pub(crate) fn blocking_directive<'a>(
        &'a self,
        kind: CspKind,
        nonce: Option<&'a str>,
        content: Option<&'a str>,
    ) -> Option<&'a str> {
        let (directive_name, sources) = self.effective_sources_prefer(match kind {
            CspKind::ScriptEventHandler => &["script-src-attr", "script-src", "default-src"],
            CspKind::InlineScript => &["script-src-elem", "script-src", "default-src"],
            CspKind::StyleAttribute => &["style-src-attr", "style-src", "default-src"],
            CspKind::InlineStyle => &["style-src-elem", "style-src", "default-src"],
        })?;

        // No relevant directive in the policy means it doesn't restrict the resource type.
        let allows_unsafe_inline = sources
            .iter()
            .any(|t| t.eq_ignore_ascii_case("'unsafe-inline'"));

        let allowed = match kind {
            CspKind::ScriptEventHandler | CspKind::StyleAttribute => {
                let allows_unsafe_hashes = sources
                    .iter()
                    .any(|t| t.eq_ignore_ascii_case("'unsafe-hashes'"));
                allows_unsafe_inline
                    || (allows_unsafe_hashes && content_hash_matches_sources(content, sources))
            }
            CspKind::InlineScript | CspKind::InlineStyle => {
                allows_unsafe_inline
                    || nonce_matches_sources(nonce, sources)
                    || content_hash_matches_sources(content, sources)
            }
        };
        (!allowed).then_some(directive_name)
    }

    pub(crate) fn effective_sources_prefer<'a>(
        &'a self,
        primaries: &[&'static str],
    ) -> Option<(&'static str, &'a [String])> {
        for &primary in primaries {
            if let Some(v) = self.directives.get(primary) {
                return Some((primary, v.as_slice()));
            }
        }
        None
    }

    pub(crate) fn blocking_external_directive(
        &self,
        kind: CspExternalKind,
        url: &str,
        base_uri: Option<&str>,
    ) -> Option<&str> {
        let (directive_name, sources) = match kind {
            CspExternalKind::Script => self
                .effective_sources_prefer(&["script-src-elem", "script-src", "default-src"])
                .unwrap_or(("default-src", &[])),
            CspExternalKind::Style => self
                .effective_sources_prefer(&["style-src-elem", "style-src", "default-src"])
                .unwrap_or(("default-src", &[])),
            CspExternalKind::Image => self
                .effective_sources_prefer(&["img-src", "default-src"])
                .unwrap_or(("default-src", &[])),
            CspExternalKind::Frame => self
                .effective_sources_prefer(&["frame-src", "default-src"])
                .unwrap_or(("default-src", &[])),
            CspExternalKind::Object => self
                .effective_sources_prefer(&["object-src", "default-src"])
                .unwrap_or(("default-src", &[])),
            CspExternalKind::Media => self
                .effective_sources_prefer(&["media-src", "default-src"])
                .unwrap_or(("default-src", &[])),
        };

        let allowed = external_url_allowed_by_sources(url, base_uri, sources);
        (!allowed).then_some(directive_name)
    }
}

pub(crate) fn parse_csp_policies(content: &str) -> Vec<CspPolicy> {
    let mut out = Vec::new();
    for policy in content.split(',').map(str::trim).filter(|p| !p.is_empty()) {
        let mut directives = HashMap::new();
        for directive in policy.split(';').map(str::trim).filter(|d| !d.is_empty()) {
            let mut parts = directive.split_ascii_whitespace();
            let Some(name) = parts.next() else { continue };
            directives.insert(
                name.to_ascii_lowercase(),
                parts.map(str::to_owned).collect(),
            );
        }
        out.push(CspPolicy { directives });
    }
    out
}

pub(crate) fn nonce_matches_sources(nonce: Option<&str>, sources: &[String]) -> bool {
    let Some(nonce) = nonce else { return false };
    if nonce.trim().is_empty() {
        return false;
    }
    sources.iter().any(|t| {
        let t = t.trim();
        if !(t.starts_with("'nonce-") && t.ends_with('\'')) {
            return false;
        }
        t.trim_matches('\'').strip_prefix("nonce-") == Some(nonce)
    })
}

pub(crate) fn content_hash_matches_sources(content: Option<&str>, sources: &[String]) -> bool {
    let Some(content) = content else { return false };
    if content.is_empty() {
        return false;
    }

    let sha256 = Sha256::digest(content.as_bytes());
    let sha384 = Sha384::digest(content.as_bytes());
    let sha512 = Sha512::digest(content.as_bytes());

    let b64 = base64::engine::general_purpose::STANDARD;
    let sha256_b64 = b64.encode(sha256);
    let sha384_b64 = b64.encode(sha384);
    let sha512_b64 = b64.encode(sha512);

    sources.iter().any(|t| {
        let token = t.trim();
        let token = token
            .strip_prefix('\'')
            .and_then(|s| s.strip_suffix('\''))
            .unwrap_or(token);
        if let Some(v) = token.strip_prefix("sha256-") {
            return v == sha256_b64;
        }
        if let Some(v) = token.strip_prefix("sha384-") {
            return v == sha384_b64;
        }
        if let Some(v) = token.strip_prefix("sha512-") {
            return v == sha512_b64;
        }
        false
    })
}

pub(crate) fn truncate(s: &str, max: usize) -> &str {
    if s.len() <= max {
        return s;
    }
    // Avoid splitting UTF-8 codepoints; walk backwards to a boundary.
    let mut end = max;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[..end]
}

pub(crate) fn external_url_allowed_by_sources(
    url: &str,
    base_uri: Option<&str>,
    sources: &[String],
) -> bool {
    if sources.is_empty() {
        return true;
    }

    let sources = sources
        .iter()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    if sources.iter().any(|t| t.eq_ignore_ascii_case("*")) {
        return true;
    }
    if sources.len() == 1 && sources[0].eq_ignore_ascii_case("'none'") {
        return false;
    }

    let base = base_uri.and_then(|b| Url::parse(b).ok());
    let resolved = resolve_url(url, base.as_ref());

    for token in sources {
        if token.eq_ignore_ascii_case("'self'") {
            // Treat relative URLs as same-origin, since they resolve against base.
            if is_relative_url(url) {
                return true;
            }
            if let (Some(base), Some(actual)) = (base.as_ref(), resolved.as_ref()) {
                if origin_key(base) == origin_key(actual) {
                    return true;
                }
            }
            continue;
        }

        // Scheme sources: `https:`, `data:`, etc.
        if token.ends_with(':') && !token.contains('/') && !token.contains('\'') {
            if let Some(actual) = resolved.as_ref() {
                let scheme = token.trim_end_matches(':');
                if actual.scheme().eq_ignore_ascii_case(scheme) {
                    return true;
                }
            }
            continue;
        }

        // Host sources: https://cdn.example.com or cdn.example.com or *.example.com.
        let token_host = token
            .strip_prefix("http://")
            .or_else(|| token.strip_prefix("https://"))
            .unwrap_or(token);
        let token_host = token_host.split_once('/').map_or(token_host, |(h, _)| h);
        if token_host.is_empty() || token_host.contains('\'') {
            continue;
        }

        if let Some(actual) = resolved.as_ref() {
            let Some(actual_host) = actual.host_str() else {
                continue;
            };
            if let Some(suffix) = token_host.strip_prefix("*.") {
                if actual_host == suffix || actual_host.ends_with(&format!(".{suffix}")) {
                    return true;
                }
            } else if actual_host.eq_ignore_ascii_case(token_host) {
                return true;
            }
        }
    }

    false
}

fn resolve_url(url: &str, base: Option<&Url>) -> Option<Url> {
    let u = url.trim();
    if u.is_empty() {
        return None;
    }
    if u.starts_with("//") {
        if let Some(base) = base {
            // Scheme-relative URL.
            return Url::parse(&format!("{}:{u}", base.scheme())).ok();
        }
        return Url::parse(&format!("https:{u}")).ok();
    }

    if let Ok(abs) = Url::parse(u) {
        return Some(abs);
    }
    base.and_then(|b| b.join(u).ok())
}

fn origin_key(url: &Url) -> (String, String) {
    // Keep parity with the older best-effort origin parsing (scheme + host, ignoring ports).
    let scheme = url.scheme().to_ascii_lowercase();
    let host = url.host_str().unwrap_or("").to_ascii_lowercase();
    (scheme, host)
}

fn is_relative_url(url: &str) -> bool {
    let u = url.trim();
    if u.is_empty() {
        return true;
    }
    if u.starts_with('#') {
        return true;
    }
    if u.starts_with('/') && !u.starts_with("//") {
        return true;
    }
    !u.contains("://") && !looks_like_scheme(u)
}

fn looks_like_scheme(s: &str) -> bool {
    let bytes = s.as_bytes();
    if bytes.is_empty() || !bytes[0].is_ascii_alphabetic() {
        return false;
    }
    for (i, &b) in bytes.iter().enumerate().skip(1) {
        if b == b':' {
            return true;
        }
        if !(b.is_ascii_alphanumeric() || b == b'+' || b == b'-' || b == b'.') {
            return false;
        }
        if i > 32 {
            return false;
        }
    }
    false
}

const EVENT_HANDLERS: &[&str] = &[
    "onabort",
    "onauxclick",
    "onbeforeinput",
    "onbeforematch",
    "onbeforetoggle",
    "onblur",
    "oncancel",
    "oncanplay",
    "oncanplaythrough",
    "onchange",
    "onclick",
    "onclose",
    "oncontextlost",
    "oncontextmenu",
    "oncontextrestored",
    "oncopy",
    "oncuechange",
    "oncut",
    "ondblclick",
    "ondrag",
    "ondragend",
    "ondragenter",
    "ondragleave",
    "ondragover",
    "ondragstart",
    "ondrop",
    "ondurationchange",
    "onemptied",
    "onended",
    "onerror",
    "onfocus",
    "onformdata",
    "ongotpointercapture",
    "oninput",
    "oninvalid",
    "onkeydown",
    "onkeypress",
    "onkeyup",
    "onload",
    "onloadeddata",
    "onloadedmetadata",
    "onloadstart",
    "onlostpointercapture",
    "onmousedown",
    "onmouseenter",
    "onmouseleave",
    "onmousemove",
    "onmouseout",
    "onmouseover",
    "onmouseup",
    "onpaste",
    "onpause",
    "onplay",
    "onplaying",
    "onpointercancel",
    "onpointerdown",
    "onpointerenter",
    "onpointerleave",
    "onpointermove",
    "onpointerout",
    "onpointerover",
    "onpointerrawupdate",
    "onpointerup",
    "onprogress",
    "onratechange",
    "onreset",
    "onresize",
    "onscroll",
    "onscrollend",
    "onsecuritypolicyviolation",
    "onseeked",
    "onseeking",
    "onselect",
    "onslotchange",
    "onstalled",
    "onsubmit",
    "onsuspend",
    "ontimeupdate",
    "ontoggle",
    "onvolumechange",
    "onwaiting",
    "onwheel",
];

pub(crate) fn is_event_handler_attr(ctx: &ValidationContext, attr_name: &str) -> bool {
    match ctx.format {
        html_inspector_core::InputFormat::Html => EVENT_HANDLERS
            .iter()
            .any(|h| attr_name.eq_ignore_ascii_case(h)),
        html_inspector_core::InputFormat::Xhtml => EVENT_HANDLERS.contains(&attr_name),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn external_url_allows_self_for_relative_urls() {
        let sources = vec!["'self'".to_string()];
        assert!(external_url_allowed_by_sources(
            "/a.js",
            Some("https://example.com/"),
            &sources
        ));
        assert!(external_url_allowed_by_sources(
            "a.js",
            Some("https://example.com/"),
            &sources
        ));
    }

    #[test]
    fn external_url_self_compares_origin_using_base_uri() {
        let sources = vec!["'self'".to_string()];
        assert!(external_url_allowed_by_sources(
            "https://example.com/a.js",
            Some("https://example.com/index.html"),
            &sources
        ));
        assert!(!external_url_allowed_by_sources(
            "https://evil.com/a.js",
            Some("https://example.com/index.html"),
            &sources
        ));
    }

    #[test]
    fn external_url_scheme_relative_uses_base_scheme() {
        let sources = vec!["https:".to_string()];
        assert!(external_url_allowed_by_sources(
            "//example.com/a.js",
            Some("https://base.example/"),
            &sources
        ));
    }
}
