use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

use super::shared::normalize_name;

#[derive(Default)]
pub struct ImpliedPEndTag;

impl Rule for ImpliedPEndTag {
    fn id(&self) -> &'static str {
        "html.parse.implied_p_end_tag"
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
        let ParseEvent::StartTag { name, span, .. } = event else {
            return;
        };

        if !is_p_end_tag_implying_start_tag(ctx, name) {
            return;
        }

        if ctx.has_ancestor("p") {
            if let Some(parent) = ctx.current_parent() {
                if !is(ctx, parent, "p") {
                    out.push(Message::new(
                        "html.parse.p.end_tag_implied_open_elements",
                        Severity::Error,
                        Category::Html,
                        "End tag “p” implied, but there were open elements.",
                        *span,
                    ));
                }
            }
        }
    }
}

fn is_p_end_tag_implying_start_tag(ctx: &ValidationContext, name: &str) -> bool {
    // In the HTML tree builder, many "block" start tags close an open <p>.
    // We model only what we need for the vnu suite and extend case-by-case.
    let n = normalize_name(ctx, name);
    matches!(
        n.as_ref(),
        "address"
            | "article"
            | "aside"
            | "blockquote"
            | "details"
            | "div"
            | "dl"
            | "fieldset"
            | "figcaption"
            | "figure"
            | "footer"
            | "form"
            | "h1"
            | "h2"
            | "h3"
            | "h4"
            | "h5"
            | "h6"
            | "header"
            | "hgroup"
            | "hr"
            | "main"
            | "menu"
            | "nav"
            | "ol"
            | "p"
            | "pre"
            | "section"
            | "table"
            | "ul"
    )
}

fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    match ctx.format {
        html_inspector_core::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector_core::InputFormat::Xhtml => actual == expected,
    }
}

#[cfg(test)]
mod tests {
    use super::is_p_end_tag_implying_start_tag;
    use html_inspector_core::{Config, InputFormat, ValidationContext};

    #[test]
    fn p_implying_start_tags_match_case_insensitively_in_html() {
        let ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        assert!(is_p_end_tag_implying_start_tag(&ctx, "DIV"));
        assert!(is_p_end_tag_implying_start_tag(&ctx, "section"));
    }

    #[test]
    fn p_implying_start_tags_match_case_sensitively_in_xhtml() {
        let ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        assert!(is_p_end_tag_implying_start_tag(&ctx, "div"));
        assert!(!is_p_end_tag_implying_start_tag(&ctx, "DIV"));
    }
}
