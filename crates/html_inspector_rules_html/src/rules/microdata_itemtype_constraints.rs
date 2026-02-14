use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

use super::a_href_constraints::{emit_forbidden_url_code_point, href_issue_severity};

#[derive(Default)]
pub struct MicrodataItemtypeConstraints;

impl Rule for MicrodataItemtypeConstraints {
    fn id(&self) -> &'static str {
        "html.microdata.itemtype.datatype"
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

        if !ctx.has_attr(attrs, "itemscope") {
            return;
        }

        let itemtype = ctx.attr_value(attrs, "itemtype");
        let Some(itemtype) = itemtype else { return };

        if itemtype.is_empty() || itemtype.trim().is_empty() {
            out.push(Message::new(
                "html.microdata.itemtype.empty",
                Severity::Error,
                Category::Html,
                format!(
                    "Bad value “” for attribute “itemtype” on element “{}”.",
                    normalize_name(ctx, name)
                ),
                *span,
            ));
            return;
        }

        if emit_forbidden_url_code_point(itemtype, *span, out) {
            return;
        }

        for token in itemtype.split_ascii_whitespace() {
            if !looks_like_absolute_url(token) {
                out.push(Message::new(
                    "html.microdata.itemtype.not_absolute",
                    Severity::Error,
                    Category::Html,
                    format!(
                        "Bad value “{itemtype}” for attribute “itemtype” on element “{}”.",
                        normalize_name(ctx, name)
                    ),
                    *span,
                ));
                return;
            }
            if let Some(sev) = href_issue_severity(token) {
                out.push(Message::new(
                    "html.microdata.itemtype.invalid",
                    sev,
                    Category::Html,
                    format!(
                        "Bad value “{itemtype}” for attribute “itemtype” on element “{}”.",
                        normalize_name(ctx, name)
                    ),
                    *span,
                ));
                return;
            }
        }
    }
}

fn looks_like_absolute_url(token: &str) -> bool {
    let bytes = token.as_bytes();
    if bytes.is_empty() {
        return false;
    }
    if !bytes[0].is_ascii_alphabetic() {
        return false;
    }
    let mut i = 1usize;
    while i < bytes.len() {
        let b = bytes[i];
        if b == b':' {
            return true;
        }
        if !(b.is_ascii_alphanumeric() || b == b'+' || b == b'-' || b == b'.') {
            return false;
        }
        i += 1;
    }
    false
}

fn normalize_name(ctx: &ValidationContext, name: &str) -> String {
    match ctx.format {
        html_inspector::InputFormat::Html => name.to_ascii_lowercase(),
        html_inspector::InputFormat::Xhtml => name.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use html_inspector::{Attribute, Config, InputFormat};

    #[test]
    fn looks_like_absolute_url_rejects_empty_and_invalid_scheme_chars() {
        assert!(!looks_like_absolute_url(""));
        assert!(!looks_like_absolute_url("a!"));
    }

    #[test]
    fn xhtml_name_normalization_and_attr_matching_are_case_sensitive() {
        let ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        assert_eq!(normalize_name(&ctx, "DIV"), "DIV".to_string());

        let attrs = vec![Attribute {
            name: "itemtype".to_string(),
            value: Some("https://example.com/".to_string()),
            span: None,
        }];
        assert_eq!(
            ctx.attr_value(&attrs, "itemtype"),
            Some("https://example.com/")
        );
        assert_eq!(ctx.attr_value(&attrs, "ITEMTYPE"), None);
    }
}
