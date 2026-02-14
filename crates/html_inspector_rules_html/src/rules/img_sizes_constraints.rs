use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};
use std::borrow::Cow;

use super::shared::starts_with_ascii_ci;

#[derive(Default)]
pub struct ImgSizesConstraints;

impl Rule for ImgSizesConstraints {
    fn id(&self) -> &'static str {
        "html.img.sizes_constraints"
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
        if !is(ctx, name, "img") {
            return;
        }

        let sizes = attr_value(ctx, attrs, "sizes");
        let Some(sizes) = sizes else { return };

        if !has_attr(ctx, attrs, "srcset") {
            out.push(Message::new(
                "html.img.sizes.requires_srcset",
                Severity::Error,
                Category::Html,
                "The “sizes” attribute must only be specified if the “srcset” attribute is also specified.",
                *span,
            ));
            return;
        }

        if starts_with_ascii_ci(sizes.trim_start(), "auto") && !loading_is_lazy(ctx, attrs) {
            out.push(Message::new(
                "html.img.sizes.auto_requires_lazy",
                Severity::Error,
                Category::Html,
                "The “sizes” attribute value starting with “auto” is only valid for lazy-loaded images. Add “loading=”“lazy” to this element.",
                *span,
            ));
            return;
        }

        if !is_valid_source_size_list(sizes) {
            out.push(Message::new(
                "html.img.sizes.invalid",
                Severity::Error,
                Category::Html,
                format!("Bad value “{sizes}” for attribute “sizes” on element “img”."),
                *span,
            ));
        }
    }
}

fn is_valid_source_size_list(input: &str) -> bool {
    let s = input.trim();
    if s.is_empty() {
        return false;
    }

    let (s, ok) = strip_css_comments(s);
    if !ok {
        return false;
    }

    let mut items = split_top_level_commas(s.as_ref()).peekable();

    let validate_item = |raw_item: &str, has_more: bool| -> bool {
        let item = raw_item.trim();
        if item.is_empty() {
            return false;
        }

        if let Some((media, value)) = split_media_and_value(item) {
            if media.contains('{')
                || media.contains('}')
                || !media.chars().any(|c| c.is_ascii_alphabetic())
            {
                return false;
            }
            if has_empty_media_feature_value(media) {
                return false;
            }
            return is_valid_source_size_value(value);
        }

        // Default source-size value (must be last).
        if has_more {
            return false;
        }
        is_valid_source_size_value(item)
    };

    let Some(first) = items.next() else {
        return false;
    };
    let first_is_auto = first.trim().eq_ignore_ascii_case("auto");
    if first_is_auto && items.peek().is_none() {
        return true;
    }
    if !first_is_auto && !validate_item(first, items.peek().is_some()) {
        return false;
    }

    while let Some(raw_item) = items.next() {
        if !validate_item(raw_item, items.peek().is_some()) {
            return false;
        }
    }

    true
}

fn has_empty_media_feature_value(media: &str) -> bool {
    let lower = media.to_ascii_lowercase();
    for needle in ["min-width:", "max-width:", "width:"] {
        if let Some(pos) = lower.find(needle) {
            let after = &media[pos + needle.len()..];
            let after = after.trim().trim_end_matches(')').trim();
            if after.is_empty() {
                return true;
            }
        }
    }
    false
}

fn is_valid_source_size_value(value: &str) -> bool {
    let v = value.trim();
    if v.is_empty() {
        return false;
    }
    if v.eq_ignore_ascii_case("default")
        || v.eq_ignore_ascii_case("inherit")
        || v.eq_ignore_ascii_case("initial")
    {
        return false;
    }
    if v.eq_ignore_ascii_case("auto") {
        return true;
    }

    // Functions are treated opaquely; require balanced parentheses and a known function prefix.
    for prefix in ["calc(", "min(", "max(", "clamp("] {
        if starts_with_ascii_ci(v, prefix) {
            return is_balanced_parens(v);
        }
    }

    // Plain length token: must not contain whitespace.
    if v.chars().any(|c| c.is_whitespace()) {
        return false;
    }
    if v.contains('%') {
        return false;
    }

    let Some((num_str, unit)) = split_number_and_unit(v) else {
        return false;
    };

    // Unitless numbers are only allowed for zero.
    if unit.is_empty() {
        return num_str.parse::<f64>().is_ok_and(|n| n == 0.0);
    }

    if !is_allowed_length_unit(unit) {
        return false;
    }

    let Ok(n) = num_str.parse::<f64>() else {
        return false;
    };
    n >= 0.0
}

fn is_allowed_length_unit(unit: &str) -> bool {
    matches!(
        unit,
        "px" | "em"
            | "ex"
            | "ch"
            | "rem"
            | "vw"
            | "vh"
            | "vmin"
            | "vmax"
            | "cm"
            | "mm"
            | "q"
            | "in"
            | "pc"
            | "pt"
    )
}

fn split_number_and_unit(s: &str) -> Option<(&str, &str)> {
    let bytes = s.as_bytes();
    let mut i = 0usize;
    if bytes.get(i) == Some(&b'+') || bytes.get(i) == Some(&b'-') {
        i += 1;
    }
    let start_digits = i;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
    }
    if i < bytes.len() && bytes[i] == b'.' {
        i += 1;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
        }
    }
    if i == start_digits {
        // no digits before '.', allow ".2"
        if bytes.get(start_digits) != Some(&b'.') {
            return None;
        }
    }
    // exponent (only if `e`/`E` is followed by [sign] digit)
    if i < bytes.len() && (bytes[i] == b'e' || bytes[i] == b'E') {
        let next = bytes.get(i + 1).copied();
        let next2 = bytes.get(i + 2).copied();
        let looks_like_exponent = match (next, next2) {
            (Some(b'+') | Some(b'-'), Some(d)) if d.is_ascii_digit() => true,
            (Some(d), _) if d.is_ascii_digit() => true,
            _ => false,
        };
        if looks_like_exponent {
            i += 1;
            if bytes.get(i) == Some(&b'+') || bytes.get(i) == Some(&b'-') {
                i += 1;
            }
            let exp_start = i;
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                i += 1;
            }
            if i == exp_start {
                return None;
            }
        }
    }
    let num = &s[..i];
    let unit = &s[i..];
    Some((num, unit))
}

fn split_media_and_value(item: &str) -> Option<(&str, &str)> {
    let mut depth = 0i32;
    let mut last_split: Option<usize> = None;
    for (idx, ch) in item.char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => depth -= 1,
            c if depth == 0 && c.is_whitespace() => last_split = Some(idx),
            _ => {}
        }
    }
    let split = last_split?;
    let media = item[..split].trim();
    let value = item[split..].trim();
    if media.is_empty() || value.is_empty() {
        return None;
    }
    if !media.trim_start().starts_with('(') {
        return None;
    }
    Some((media, value))
}

fn strip_css_comments(input: &str) -> (Cow<'_, str>, bool) {
    let bytes = input.as_bytes();
    let mut i = 0usize;
    let mut last = 0usize;
    let mut out: Option<String> = None;
    while i + 1 < bytes.len() {
        if bytes[i] == b'/' && bytes[i + 1] == b'*' {
            let mut j = i + 2;
            while j + 1 < bytes.len() && !(bytes[j] == b'*' && bytes[j + 1] == b'/') {
                j += 1;
            }
            if j + 1 >= bytes.len() {
                let prefix = match out {
                    Some(mut out_buf) => {
                        out_buf.push_str(&input[last..i]);
                        Cow::Owned(out_buf)
                    }
                    None => Cow::Borrowed(&input[..i]),
                };
                return (prefix, false);
            }

            let out_buf = out.get_or_insert_with(|| String::with_capacity(input.len()));
            out_buf.push_str(&input[last..i]);
            i = j + 2;
            // Preserve token boundaries: comments behave like whitespace separators.
            out_buf.push(' ');
            last = i;
            continue;
        }
        i += 1;
    }

    match out {
        Some(mut out) => {
            out.push_str(&input[last..]);
            (Cow::Owned(out), true)
        }
        None => (Cow::Borrowed(input), true),
    }
}

fn split_top_level_commas<'a>(s: &'a str) -> impl Iterator<Item = &'a str> + 'a {
    let bytes = s.as_bytes();
    let mut start = 0usize;
    let mut i = 0usize;
    let mut depth = 0i32;
    let mut done = false;

    std::iter::from_fn(move || {
        if done {
            return None;
        }

        while i < bytes.len() {
            match bytes[i] {
                b'(' => depth += 1,
                b')' => depth -= 1,
                b',' if depth == 0 => {
                    let out = &s[start..i];
                    i += 1;
                    start = i;
                    return Some(out);
                }
                _ => {}
            }
            i += 1;
        }

        done = true;
        Some(&s[start..])
    })
}

fn is_balanced_parens(s: &str) -> bool {
    let mut depth = 0i32;
    for ch in s.chars() {
        match ch {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth < 0 {
                    return false;
                }
            }
            _ => {}
        }
    }
    depth == 0 && s.trim_end().ends_with(')')
}

fn loading_is_lazy(ctx: &ValidationContext, attrs: &[html_inspector::Attribute]) -> bool {
    let Some(loading) = attr_value(ctx, attrs, "loading") else {
        return false;
    };
    match ctx.format {
        html_inspector::InputFormat::Html => loading.eq_ignore_ascii_case("lazy"),
        html_inspector::InputFormat::Xhtml => loading == "lazy",
    }
}

fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    match ctx.format {
        html_inspector::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector::InputFormat::Xhtml => actual == expected,
    }
}

fn has_attr(ctx: &ValidationContext, attrs: &[html_inspector::Attribute], needle: &str) -> bool {
    match ctx.format {
        html_inspector::InputFormat::Html => {
            attrs.iter().any(|a| a.name.eq_ignore_ascii_case(needle))
        }
        html_inspector::InputFormat::Xhtml => attrs.iter().any(|a| a.name == needle),
    }
}

fn attr_value<'a>(
    ctx: &ValidationContext,
    attrs: &'a [html_inspector::Attribute],
    needle: &str,
) -> Option<&'a str> {
    let attr = match ctx.format {
        html_inspector::InputFormat::Html => {
            attrs.iter().find(|a| a.name.eq_ignore_ascii_case(needle))
        }
        html_inspector::InputFormat::Xhtml => attrs.iter().find(|a| a.name == needle),
    };
    attr.and_then(|a| a.value.as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;

    use html_inspector::{Attribute, Config, InputFormat, ValidationContext};

    fn attr(name: &str, value: Option<&str>) -> Attribute {
        Attribute {
            name: name.to_string(),
            value: value.map(|v| v.to_string()),
            span: None,
        }
    }

    #[test]
    fn source_size_value_validation_covers_empty_auto_functions_and_numbers() {
        assert!(!is_valid_source_size_value(""));
        assert!(is_valid_source_size_value("auto"));
        assert!(!is_valid_source_size_value("default"));
        assert!(is_valid_source_size_value("calc(1px)"));
        assert!(is_valid_source_size_value("CALC(1px)"));
        assert!(!is_valid_source_size_value("calc(1px"));
        assert!(!is_valid_source_size_value("10%"));
        assert!(!is_valid_source_size_value("1 e"));
        assert!(!is_valid_source_size_value("1e+"));
        assert!(is_valid_source_size_value("0"));
        assert!(!is_valid_source_size_value("1"));
        assert!(is_valid_source_size_value("1px"));
        assert!(!is_valid_source_size_value("-1px"));
    }

    #[test]
    fn split_media_and_value_rejects_empty_media_or_value() {
        assert!(split_media_and_value(" 100vw").is_none());
        assert!(split_media_and_value("(min-width: 1px)").is_none());
        assert!(split_media_and_value("(min-width: 1px) 100vw").is_some());
    }

    #[test]
    fn split_top_level_commas_ignores_commas_inside_parentheses() {
        let parts: Vec<&str> = split_top_level_commas("a,(b,c),d").collect();
        assert_eq!(parts, vec!["a", "(b,c)", "d"]);
    }

    #[test]
    fn split_top_level_commas_preserves_empty_segments() {
        let parts: Vec<&str> = split_top_level_commas("").collect();
        assert_eq!(parts, vec![""]);

        let parts: Vec<&str> = split_top_level_commas("a,").collect();
        assert_eq!(parts, vec!["a", ""]);
    }

    #[test]
    fn split_top_level_commas_preserves_trailing_empty_after_parenthesized_segment() {
        let parts: Vec<&str> = split_top_level_commas("a,(b,c),").collect();
        assert_eq!(parts, vec!["a", "(b,c)", ""]);
    }

    #[test]
    fn balanced_parens_rejects_early_closing() {
        assert!(!is_balanced_parens(")"));
        assert!(is_balanced_parens("calc(1px)"));
    }

    #[test]
    fn loading_is_lazy_matches_xhtml_case_sensitively() {
        let ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        assert!(loading_is_lazy(&ctx, &[attr("loading", Some("lazy"))]));
        assert!(!loading_is_lazy(&ctx, &[attr("loading", Some("Lazy"))]));
    }

    #[test]
    fn has_attr_and_attr_value_follow_format_case_rules() {
        let html_ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        assert!(has_attr(&html_ctx, &[attr("LoAdInG", None)], "loading"));
        assert_eq!(
            attr_value(&html_ctx, &[attr("LoAdInG", Some("lazy"))], "loading"),
            Some("lazy")
        );

        let xhtml_ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        assert!(!has_attr(&xhtml_ctx, &[attr("LoAdInG", None)], "loading"));
        assert_eq!(
            attr_value(&xhtml_ctx, &[attr("LoAdInG", Some("lazy"))], "loading"),
            None
        );
    }

    #[test]
    fn source_size_list_rejects_empty_media_feature_value_and_bad_default_position() {
        assert!(!is_valid_source_size_list("(min-width:) 100vw"));
        assert!(!is_valid_source_size_list("(MIN-WIDTH:) 100vw"));
        assert!(!is_valid_source_size_list("100vw, (min-width: 1px) 50vw"));
        assert!(is_valid_source_size_list("(min-width: 1px) 50vw, 100vw"));
    }

    #[test]
    fn source_size_list_auto_handling_matches_existing_semantics() {
        assert!(is_valid_source_size_list("auto"));
        assert!(is_valid_source_size_list("auto, 100vw"));
        assert!(is_valid_source_size_list(
            "auto, (min-width: 1px) 50vw, 100vw"
        ));
        assert!(!is_valid_source_size_list("auto,"));
    }

    #[test]
    fn split_number_and_unit_covers_decimals_and_exponents() {
        assert_eq!(split_number_and_unit("1.0px"), Some(("1.0", "px")));
        assert_eq!(split_number_and_unit(".2px"), Some((".2", "px")));
        assert_eq!(split_number_and_unit("1e2px"), Some(("1e2", "px")));
        assert_eq!(split_number_and_unit("1e+2px"), Some(("1e+2", "px")));
        assert_eq!(split_number_and_unit("1E-2px"), Some(("1E-2", "px")));
    }

    #[test]
    fn split_number_and_unit_rejects_inputs_with_no_number() {
        assert_eq!(split_number_and_unit("px"), None);
        assert_eq!(split_number_and_unit("+px"), None);
    }

    #[test]
    fn strip_css_comments_removes_closed_comments_and_reports_unclosed() {
        let (s, ok) = strip_css_comments("/*x*/100vw");
        assert!(ok);
        assert_eq!(s.as_ref(), " 100vw");

        let (s3, ok3) = strip_css_comments("a/*x*/bé");
        assert!(ok3);
        assert_eq!(s3.as_ref(), "a bé");

        let (s2, ok2) = strip_css_comments("/*");
        assert!(!ok2);
        assert_eq!(s2.as_ref(), "");

        let (s4, ok4) = strip_css_comments("abc/*");
        assert!(!ok4);
        assert_eq!(s4.as_ref(), "abc");

        let (s5, ok5) = strip_css_comments("bé/*");
        assert!(!ok5);
        assert_eq!(s5.as_ref(), "bé");
    }

    #[test]
    fn strip_css_comments_returns_prefix_on_unclosed_comment_after_a_previous_comment() {
        let (s, ok) = strip_css_comments("a/*x*/b/*y");
        assert!(!ok);
        assert_eq!(s.as_ref(), "a b");
    }

    #[test]
    fn source_size_list_rejects_media_without_alpha_and_handles_xhtml_attr_matching() {
        assert!(!is_valid_source_size_list("() 100vw"));

        let ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        assert!(has_attr(&ctx, &[attr("srcset", Some("x"))], "srcset"));
        assert!(!has_attr(&ctx, &[attr("SRCSET", Some("x"))], "srcset"));
        assert_eq!(
            attr_value(&ctx, &[attr("sizes", Some("x"))], "sizes"),
            Some("x")
        );
        assert!(is(&ctx, "img", "img"));
    }

    #[test]
    fn strip_css_comments_returns_borrowed_when_no_comments() {
        let (s, ok) = strip_css_comments("100vw");
        assert!(ok);
        assert!(matches!(s, std::borrow::Cow::Borrowed(_)));
        assert_eq!(s.as_ref(), "100vw");
    }
}
