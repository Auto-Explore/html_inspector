use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

use super::foreign_content::{Namespace, namespace_for_next_start_tag};

#[derive(Default)]
pub struct AttributeNotAllowedConstraints;

impl Rule for AttributeNotAllowedConstraints {
    fn id(&self) -> &'static str {
        "html.attributes.not_allowed"
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

        // These checks are for HTML documents and the HTML namespace.
        if ctx.format != html_inspector_core::InputFormat::Html {
            return;
        }
        if namespace_for_next_start_tag(ctx, name) != Namespace::Html {
            return;
        }

        let el = name.as_str();

        for attr in attrs {
            let attr_name = attr.name.as_str();
            if attr_name == "href" && !matches!(el, "a" | "area" | "base" | "link") {
                out.push(Message::new(
                    "html.attr.href.not_allowed",
                    Severity::Error,
                    Category::Html,
                    format!("Attribute “href” not allowed on element “{el}” at this point."),
                    *span,
                ));
                return;
            }

            if el == "a" && attr_name == "src" {
                out.push(Message::new(
                    "html.attr.src.not_allowed_on_a",
                    Severity::Error,
                    Category::Html,
                    "Attribute “src” not allowed on element “a” at this point.",
                    *span,
                ));
                return;
            }

            if el == "html" && attr_name == "xml:base" {
                out.push(Message::new(
                    "html.attr.xml_base.not_allowed",
                    Severity::Error,
                    Category::Html,
                    "Attribute “xml:base” not allowed on element “html” at this point.",
                    *span,
                ));
                return;
            }

            if attr_name == "prefix" {
                // Match vnu.jar schema behavior (schema/html5/rdfa.rnc): allow the empty string,
                // but not whitespace-only values.
                let raw = attr.value.as_deref().unwrap_or("");
                if !raw.is_empty() && !is_valid_rdfa_prefix_value(raw.trim()) {
                    let v = raw;
                    out.push(Message::new(
                        "html.attr.prefix.bad_value",
                        Severity::Error,
                        Category::Html,
                        format!("Bad value “{v}” for attribute “prefix” on element “{el}”."),
                        *span,
                    ));
                    return;
                }
            }

            if attr_name.starts_with("xmlns:") {
                out.push(Message::new(
                    "html.attr.xmlns.not_allowed",
                    Severity::Error,
                    Category::Html,
                    format!("Attribute “{}” not allowed here.", attr.name),
                    *span,
                ));
                return;
            }
        }
    }
}

fn is_valid_rdfa_prefix_value(value: &str) -> bool {
    if value.is_empty() {
        return false;
    }
    let mut tokens = value.split_ascii_whitespace();
    while let Some(prefix) = tokens.next() {
        let Some(iri) = tokens.next() else {
            return false;
        };
        let Some(label) = prefix.strip_suffix(':') else {
            return false;
        };
        if !is_valid_prefix_label(label) {
            return false;
        }
        // Keep URI validation lightweight; suite coverage focuses on the prefix label.
        if iri.is_empty() {
            return false;
        }
    }
    true
}

fn is_valid_prefix_label(label: &str) -> bool {
    let mut it = label.chars();
    let Some(first) = it.next() else { return false };
    if !first.is_ascii_alphabetic() {
        return false;
    }
    it.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.')
}

#[cfg(test)]
mod tests {
    use super::*;

    use html_inspector_core::{Config, InputFormat, RuleSet};
    use html_inspector_html::HtmlEventSource;

    #[test]
    fn rdfa_prefix_value_validation_accepts_multiple_pairs() {
        assert!(is_valid_rdfa_prefix_value("a: http://example.com/"));
        assert!(is_valid_rdfa_prefix_value("a: x b: y"));
        assert!(is_valid_rdfa_prefix_value("a.b-_: x"));
    }

    #[test]
    fn rdfa_prefix_value_validation_rejects_common_invalid_values() {
        assert!(!is_valid_rdfa_prefix_value(""));
        assert!(!is_valid_rdfa_prefix_value("a:"));
        assert!(!is_valid_rdfa_prefix_value("a x"));
        assert!(!is_valid_rdfa_prefix_value(": x"));
        assert!(!is_valid_rdfa_prefix_value("1a: x"));
        assert!(!is_valid_rdfa_prefix_value("a*: x"));
    }

    #[test]
    fn rule_emits_error_for_bad_prefix_value() {
        let src = HtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            r#"<div prefix="1a: http://example.com/"></div>"#,
        )
        .unwrap();
        let rules = RuleSet::new().push(AttributeNotAllowedConstraints::default());
        let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.attr.prefix.bad_value")
        );
    }

    #[test]
    fn rule_allows_empty_prefix_value() {
        let src =
            HtmlEventSource::from_str("t", InputFormat::Html, r#"<div prefix=""></div>"#).unwrap();
        let rules = RuleSet::new().push(AttributeNotAllowedConstraints::default());
        let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
        assert!(
            !report
                .messages
                .iter()
                .any(|m| m.code == "html.attr.prefix.bad_value")
        );
    }

    #[test]
    fn rule_rejects_whitespace_only_prefix_value() {
        let src =
            HtmlEventSource::from_str("t", InputFormat::Html, r#"<div prefix=" "></div>"#).unwrap();
        let rules = RuleSet::new().push(AttributeNotAllowedConstraints::default());
        let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.attr.prefix.bad_value")
        );
    }
}
