use std::borrow::Cow;

use html_inspector::{Attribute, ValidationContext};

pub(super) use html_inspector::ascii_lowercase_cow_if_needed;
pub(super) use html_inspector::starts_with_ascii_ci;

pub(super) fn eq_name(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    ctx.name_is(actual, expected)
}

pub(super) fn attr_value<'a>(
    ctx: &ValidationContext,
    attrs: &'a [Attribute],
    name: &str,
) -> Option<&'a str> {
    attrs
        .iter()
        .find(|a| eq_name(ctx, &a.name, name))
        .and_then(|a| a.value.as_deref())
}

pub(super) fn normalize_name<'a>(ctx: &ValidationContext, name: &'a str) -> Cow<'a, str> {
    match ctx.format {
        html_inspector::InputFormat::Html => ascii_lowercase_cow_if_needed(name),
        html_inspector::InputFormat::Xhtml => Cow::Borrowed(name),
    }
}

pub(super) fn is_phrasing_element(ctx: &ValidationContext, name: &str) -> bool {
    // Intentionally incomplete: extended case-by-case using the vnu suite.
    let n = normalize_name(ctx, name);
    matches!(
        n.as_ref(),
        "a" | "abbr"
            | "b"
            | "bdi"
            | "bdo"
            | "br"
            | "button"
            | "cite"
            | "code"
            | "data"
            | "del"
            | "dfn"
            | "em"
            | "i"
            | "img"
            | "input"
            | "ins"
            | "kbd"
            | "label"
            | "mark"
            | "meter"
            | "noscript"
            | "output"
            | "picture"
            | "progress"
            | "q"
            | "ruby"
            | "s"
            | "samp"
            | "script"
            | "select"
            | "small"
            | "span"
            | "strong"
            | "sub"
            | "sup"
            | "template"
            | "textarea"
            | "time"
            | "u"
            | "var"
            | "wbr"
    )
}

#[cfg(test)]
mod tests {
    use super::{attr_value, eq_name, is_phrasing_element, starts_with_ascii_ci};
    use html_inspector::{Attribute, Config, InputFormat, ValidationContext};

    #[test]
    fn phrasing_elements_match_case_insensitively_in_html() {
        let ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        assert!(is_phrasing_element(&ctx, "A"));
        assert!(is_phrasing_element(&ctx, "span"));
        assert!(!is_phrasing_element(&ctx, "div"));
    }

    #[test]
    fn phrasing_elements_match_case_sensitively_in_xhtml() {
        let ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        assert!(is_phrasing_element(&ctx, "a"));
        assert!(!is_phrasing_element(&ctx, "A"));
    }

    #[test]
    fn starts_with_ascii_ci_is_case_insensitive_and_safe_on_short_inputs() {
        assert!(starts_with_ascii_ci("AUTO 1fr", "auto"));
        assert!(starts_with_ascii_ci("auto", "AUTO"));
        assert!(!starts_with_ascii_ci("au", "auto"));
        assert!(starts_with_ascii_ci("", ""));
        assert!(!starts_with_ascii_ci("❤", "❤H"));
    }

    #[test]
    fn eq_name_matches_case_insensitively_in_html_and_sensitively_in_xhtml() {
        let html = ValidationContext::new(Config::default(), InputFormat::Html);
        assert!(eq_name(&html, "DIV", "div"));

        let xhtml = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        assert!(!eq_name(&xhtml, "DIV", "div"));
    }

    #[test]
    fn attr_value_follows_input_format_case_rules() {
        let attrs = vec![Attribute {
            name: "HREF".to_string(),
            value: Some("x".to_string()),
            span: None,
        }];

        let html = ValidationContext::new(Config::default(), InputFormat::Html);
        assert_eq!(attr_value(&html, &attrs, "href"), Some("x"));

        let xhtml = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        assert_eq!(attr_value(&xhtml, &attrs, "href"), None);
    }

    #[test]
    fn attr_value_uses_first_matching_attribute_even_if_valueless() {
        let attrs = vec![
            Attribute {
                name: "disabled".to_string(),
                value: None,
                span: None,
            },
            Attribute {
                name: "disabled".to_string(),
                value: Some("x".to_string()),
                span: None,
            },
        ];

        let html = ValidationContext::new(Config::default(), InputFormat::Html);
        assert_eq!(attr_value(&html, &attrs, "disabled"), None);
    }
}
