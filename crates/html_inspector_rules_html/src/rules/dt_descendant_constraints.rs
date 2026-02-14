use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

use super::shared::normalize_name;

#[derive(Default)]
pub struct DtDescendantConstraints {
    dt_open: bool,
}

impl Rule for DtDescendantConstraints {
    fn id(&self) -> &'static str {
        "html.dt.descendant_constraints"
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
            ParseEvent::StartTag {
                name,
                self_closing,
                span,
                ..
            } => {
                // Our tokenizer doesn't implement implicit closing for <dt>/<dd>. Track a simplified
                // "dt scope" so we don't flag descendants after a <dd> starts (suite coverage).
                if is(ctx, name, "dt") && !*self_closing {
                    self.dt_open = true;
                    return;
                }
                if is(ctx, name, "dd") {
                    self.dt_open = false;
                    return;
                }

                if !self.dt_open {
                    return;
                }

                if !is_disallowed_in_dt(ctx, name) {
                    return;
                }

                out.push(Message::new(
                    "html.dt.disallowed_descendant",
                    Severity::Error,
                    Category::Html,
                    format!(
                        "The element “{name}” must not appear as a descendant of the “dt” element."
                    ),
                    *span,
                ));
            }
            ParseEvent::EndTag { name, .. } => {
                if is(ctx, name, "dt") || is(ctx, name, "dl") {
                    self.dt_open = false;
                }
            }
            _ => {}
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, _out: &mut dyn MessageSink) {
        self.dt_open = false;
    }
}

fn is_disallowed_in_dt(ctx: &ValidationContext, name: &str) -> bool {
    let n = normalize_name(ctx, name);
    matches!(
        n.as_ref(),
        "article"
            | "h1"
            | "h2"
            | "h3"
            | "h4"
            | "h5"
            | "h6"
            | "header"
            | "footer"
            | "hgroup"
            | "nav"
            | "section"
    )
}

fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    match ctx.format {
        html_inspector::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector::InputFormat::Xhtml => actual == expected,
    }
}

#[cfg(test)]
mod tests {
    use super::is_disallowed_in_dt;
    use html_inspector::{Config, InputFormat, ValidationContext};

    #[test]
    fn disallowed_dt_descendants_match_case_insensitively_in_html() {
        let ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        assert!(is_disallowed_in_dt(&ctx, "SECTION"));
        assert!(!is_disallowed_in_dt(&ctx, "span"));
    }

    #[test]
    fn disallowed_dt_descendants_match_case_sensitively_in_xhtml() {
        let ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        assert!(is_disallowed_in_dt(&ctx, "section"));
        assert!(!is_disallowed_in_dt(&ctx, "SECTION"));
    }
}
