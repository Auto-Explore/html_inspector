use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

use super::shared::ascii_lowercase_cow_if_needed;

#[derive(Default)]
pub struct ImgRoleConstraints;

impl Rule for ImgRoleConstraints {
    fn id(&self) -> &'static str {
        "html.img.role_constraints"
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

        let role = attr_value(ctx, attrs, "role");
        let Some(role) = role else { return };
        let role_trim = role.trim();

        if role_trim.is_empty() {
            out.push(Message::new(
                "html.img.role.empty",
                Severity::Error,
                Category::Html,
                "Bad value “” for attribute “role” on element “img”.",
                *span,
            ));
            return;
        }

        // Validate role tokens (fallback roles) against known ARIA roles.
        if role_trim.split_ascii_whitespace().any(|t| {
            let t_lc = ascii_lowercase_cow_if_needed(t);
            !is_known_role(t_lc.as_ref())
        }) {
            out.push(Message::new(
                "html.img.role.invalid",
                Severity::Error,
                Category::Html,
                format!("Bad value “{role_trim}” for attribute “role” on element “img”."),
                *span,
            ));
            return;
        }

        // If a role attribute is present, alt must not be the empty string.
        if let Some(alt) = attr_value(ctx, attrs, "alt") {
            if alt.is_empty() {
                out.push(Message::new(
                    "html.img.role.alt_empty",
                    Severity::Error,
                    Category::Html,
                    "An “img” element with a “role” attribute must not have an “alt” attribute whose value is the empty string.",
                    *span,
                ));
                return;
            }
        }

        // A role attribute implies the image participates in accessibility tree and needs a name.
        // For VNU parity, accept aria-label/labelledby as naming mechanisms too.
        let has_accessible_name = attr_value(ctx, attrs, "alt").is_some_and(|v| !v.is_empty())
            || attr_value(ctx, attrs, "aria-label").is_some_and(|v| !v.trim().is_empty())
            || attr_value(ctx, attrs, "aria-labelledby").is_some_and(|v| !v.trim().is_empty());
        if !has_accessible_name {
            out.push(Message::new(
                "html.img.role.accessible_name.missing",
                Severity::Error,
                Category::Html,
                "An “img” element with a “role” attribute must also have an accessible name (e.g., an “alt” attribute).",
                *span,
            ));
            return;
        }

        // role=none/presentation is invalid when the image is not purely decorative (suite coverage).
        let role_lc = ascii_lowercase_cow_if_needed(role_trim);
        if role_lc.as_ref() == "none" || role_lc.as_ref() == "presentation" {
            let has_non_empty_alt = attr_value(ctx, attrs, "alt").is_some_and(|v| !v.is_empty());
            let non_decorative = has_non_empty_alt
                || attr_value(ctx, attrs, "aria-label").is_some_and(|v| !v.trim().is_empty())
                || attr_value(ctx, attrs, "aria-labelledby").is_some_and(|v| !v.trim().is_empty());
            if non_decorative {
                // Note: vnu’s message for this case omits the trailing period.
                out.push(Message::new(
                    if has_non_empty_alt {
                        "html.img.role.invalid_for_non_empty_alt"
                    } else {
                        "html.img.role.invalid_for_non_decorative"
                    },
                    Severity::Error,
                    Category::Html,
                    format!("Bad value “{role_trim}” for attribute “role” on element “img”"),
                    *span,
                ));
            }
        }
    }
}

fn is_known_role(role_lc: &str) -> bool {
    // A pragmatic ARIA role list (WAI-ARIA 1.2 core roles + none/presentation).
    // Expanded case-by-case using the vnu suite.
    matches!(
        role_lc,
        "alert"
            | "alertdialog"
            | "application"
            | "article"
            | "banner"
            | "button"
            | "cell"
            | "checkbox"
            | "columnheader"
            | "combobox"
            | "complementary"
            | "contentinfo"
            | "definition"
            | "dialog"
            | "directory"
            | "document"
            | "feed"
            | "figure"
            | "form"
            | "grid"
            | "gridcell"
            | "group"
            | "heading"
            | "img"
            | "link"
            | "list"
            | "listbox"
            | "listitem"
            | "log"
            | "main"
            | "marquee"
            | "math"
            | "menu"
            | "menubar"
            | "menuitem"
            | "menuitemcheckbox"
            | "menuitemradio"
            | "navigation"
            | "none"
            | "note"
            | "option"
            | "presentation"
            | "progressbar"
            | "radio"
            | "radiogroup"
            | "region"
            | "row"
            | "rowgroup"
            | "rowheader"
            | "scrollbar"
            | "search"
            | "searchbox"
            | "separator"
            | "slider"
            | "spinbutton"
            | "status"
            | "switch"
            | "tab"
            | "table"
            | "tablist"
            | "tabpanel"
            | "term"
            | "textbox"
            | "timer"
            | "toolbar"
            | "tooltip"
            | "tree"
            | "treegrid"
            | "treeitem"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    use html_inspector_core::{Attribute, Config, InputFormat, Span};

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
            value: value.map(|v| v.to_string()),
            span: None,
        }
    }

    #[test]
    fn role_empty_is_rejected_and_presentation_requires_decorative_image() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink::default();
        let mut rule = ImgRoleConstraints::default();

        rule.on_event(
            &ParseEvent::StartTag {
                name: "img".to_string(),
                attrs: vec![attr("role", Some(""))],
                self_closing: true,
                span: Some(Span::new(0, 0, 1, 1)),
            },
            &mut ctx,
            &mut sink,
        );
        assert!(sink.0.iter().any(|m| m.code == "html.img.role.empty"));

        let mut sink = Sink::default();
        rule.on_event(
            &ParseEvent::StartTag {
                name: "img".to_string(),
                attrs: vec![
                    attr("role", Some("presentation")),
                    attr("aria-label", Some("x")),
                ],
                self_closing: true,
                span: Some(Span::new(0, 0, 1, 1)),
            },
            &mut ctx,
            &mut sink,
        );
        assert!(sink
            .0
            .iter()
            .any(|m| m.code == "html.img.role.invalid_for_non_decorative"));
    }

    #[test]
    fn aria_labelledby_makes_role_presentation_non_decorative() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink::default();
        let mut rule = ImgRoleConstraints::default();

        rule.on_event(
            &ParseEvent::StartTag {
                name: "img".to_string(),
                attrs: vec![
                    attr("role", Some("none")),
                    attr("aria-labelledby", Some("x")),
                ],
                self_closing: true,
                span: Some(Span::new(0, 0, 1, 1)),
            },
            &mut ctx,
            &mut sink,
        );
        assert!(sink
            .0
            .iter()
            .any(|m| m.code == "html.img.role.invalid_for_non_decorative"));
    }

    #[test]
    fn role_tokens_alt_empty_and_missing_accessible_name_emit_errors() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut rule = ImgRoleConstraints::default();

        let mut sink = Sink::default();
        rule.on_event(
            &ParseEvent::StartTag {
                name: "img".to_string(),
                attrs: vec![attr("role", Some("bogus"))],
                self_closing: true,
                span: Some(Span::new(0, 0, 1, 1)),
            },
            &mut ctx,
            &mut sink,
        );
        assert!(sink.0.iter().any(|m| m.code == "html.img.role.invalid"));

        let mut sink = Sink::default();
        rule.on_event(
            &ParseEvent::StartTag {
                name: "img".to_string(),
                attrs: vec![attr("role", Some("img")), attr("alt", Some(""))],
                self_closing: true,
                span: Some(Span::new(0, 0, 1, 1)),
            },
            &mut ctx,
            &mut sink,
        );
        assert!(sink.0.iter().any(|m| m.code == "html.img.role.alt_empty"));

        let mut sink = Sink::default();
        rule.on_event(
            &ParseEvent::StartTag {
                name: "img".to_string(),
                attrs: vec![attr("role", Some("img"))],
                self_closing: true,
                span: Some(Span::new(0, 0, 1, 1)),
            },
            &mut ctx,
            &mut sink,
        );
        assert!(sink
            .0
            .iter()
            .any(|m| m.code == "html.img.role.accessible_name.missing"));

        let mut sink = Sink::default();
        rule.on_event(
            &ParseEvent::StartTag {
                name: "img".to_string(),
                attrs: vec![attr("role", Some("presentation")), attr("alt", Some("x"))],
                self_closing: true,
                span: Some(Span::new(0, 0, 1, 1)),
            },
            &mut ctx,
            &mut sink,
        );
        assert!(sink
            .0
            .iter()
            .any(|m| m.code == "html.img.role.invalid_for_non_empty_alt"));
    }

    #[test]
    fn role_token_matching_is_case_insensitive_in_html() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink::default();
        let mut rule = ImgRoleConstraints::default();

        rule.on_event(
            &ParseEvent::StartTag {
                name: "img".to_string(),
                attrs: vec![attr("role", Some("IMG")), attr("alt", Some("x"))],
                self_closing: true,
                span: Some(Span::new(0, 0, 1, 1)),
            },
            &mut ctx,
            &mut sink,
        );
        assert!(sink.0.is_empty());
    }

    #[test]
    fn xhtml_attribute_matching_is_case_sensitive() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        let mut sink = Sink::default();
        let mut rule = ImgRoleConstraints::default();

        rule.on_event(
            &ParseEvent::StartTag {
                name: "img".to_string(),
                attrs: vec![attr("role", Some(""))],
                self_closing: true,
                span: Some(Span::new(0, 0, 1, 1)),
            },
            &mut ctx,
            &mut sink,
        );
        assert!(sink.0.iter().any(|m| m.code == "html.img.role.empty"));

        let mut sink = Sink::default();
        rule.on_event(
            &ParseEvent::StartTag {
                name: "img".to_string(),
                attrs: vec![
                    attr("ROLE", Some("presentation")),
                    attr("aria-label", Some("x")),
                ],
                self_closing: true,
                span: Some(Span::new(0, 0, 1, 1)),
            },
            &mut ctx,
            &mut sink,
        );
        assert!(sink.0.is_empty());
    }

    #[test]
    fn rule_ignores_non_start_tags() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink::default();
        let mut rule = ImgRoleConstraints::default();
        rule.on_event(
            &ParseEvent::Text {
                text: "x".to_string(),
                span: None,
            },
            &mut ctx,
            &mut sink,
        );
        assert!(sink.0.is_empty());
    }
}

fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    match ctx.format {
        html_inspector_core::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector_core::InputFormat::Xhtml => actual == expected,
    }
}

fn attr_value<'a>(
    ctx: &ValidationContext,
    attrs: &'a [html_inspector_core::Attribute],
    needle: &str,
) -> Option<&'a str> {
    attrs
        .iter()
        .find(|a| match ctx.format {
            html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case(needle),
            html_inspector_core::InputFormat::Xhtml => a.name == needle,
        })
        .and_then(|a| a.value.as_deref())
}
