use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

use super::shared::normalize_name;

#[derive(Default)]
pub struct PEndTagScope {
    p_implicitly_closed: bool,
}

impl Rule for PEndTagScope {
    fn id(&self) -> &'static str {
        "html.parse.p_end_tag_scope"
    }

    fn interest(&self) -> Interest {
        Interest::START_TAG | Interest::END_TAG
    }

    fn on_event(
        &mut self,
        event: &ParseEvent,
        ctx: &mut ValidationContext,
        out: &mut dyn MessageSink,
    ) {
        match event {
            ParseEvent::StartTag { name, .. } => {
                // Many block elements implicitly close <p>.
                if closes_p(ctx, name) && is(ctx, ctx.current_parent().unwrap_or(""), "p") {
                    self.p_implicitly_closed = true;
                }
            }
            ParseEvent::EndTag { name, span } => {
                if is(ctx, name, "p") && self.p_implicitly_closed {
                    out.push(Message::new(
                        "html.parse.p.end_tag_without_scope",
                        Severity::Error,
                        Category::Html,
                        "No “p” element in scope but a “p” end tag seen.",
                        *span,
                    ));
                    self.p_implicitly_closed = false;
                }
            }
            _ => {}
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, _out: &mut dyn MessageSink) {
        self.p_implicitly_closed = false;
    }
}

fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    match ctx.format {
        html_inspector::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector::InputFormat::Xhtml => actual == expected,
    }
}

fn closes_p(ctx: &ValidationContext, name: &str) -> bool {
    let name_lc = normalize_name(ctx, name);
    matches!(
        name_lc.as_ref(),
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

#[cfg(test)]
mod tests {
    use super::closes_p;
    use html_inspector::{Config, InputFormat, ValidationContext};

    #[test]
    fn closes_p_matches_case_insensitively_in_html() {
        let ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        assert!(closes_p(&ctx, "DIV"));
        assert!(!closes_p(&ctx, "span"));
    }

    #[test]
    fn closes_p_matches_case_sensitively_in_xhtml() {
        let ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        assert!(closes_p(&ctx, "div"));
        assert!(!closes_p(&ctx, "DIV"));
    }
}
