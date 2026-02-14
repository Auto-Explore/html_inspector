use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct ArticleHeadingWarning {
    stack: Vec<ArticleState>,
}

#[derive(Clone, Copy, Debug)]
struct ArticleState {
    has_heading: bool,
}

impl Rule for ArticleHeadingWarning {
    fn id(&self) -> &'static str {
        "html.article.heading_warning"
    }

    fn max_severity(&self) -> Severity {
        Severity::Warning
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
                if is(ctx, name, "article") {
                    self.stack.push(ArticleState { has_heading: false });
                    return;
                }

                if let Some(top) = self.stack.last_mut()
                    && is_heading_element(ctx, name)
                {
                    top.has_heading = true;
                }
            }
            ParseEvent::EndTag { name, span } => {
                if is(ctx, name, "article")
                    && let Some(state) = self.stack.pop()
                    && !state.has_heading
                {
                    out.push(Message::new(
                                "html.article.lacks_heading",
                                Severity::Warning,
                                Category::Html,
                                "Article lacks heading. Consider using “h2”-“h6” elements to add identifying headings to all articles.",
                                *span,
                            ));
                }
            }
            _ => {}
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, _out: &mut dyn MessageSink) {
        self.stack.clear();
    }
}

fn is_heading_element(ctx: &ValidationContext, name: &str) -> bool {
    // vnu warning suggests h2-h6 specifically.
    is(ctx, name, "h2")
        || is(ctx, name, "h3")
        || is(ctx, name, "h4")
        || is(ctx, name, "h5")
        || is(ctx, name, "h6")
}

fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    match ctx.format {
        html_inspector::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector::InputFormat::Xhtml => actual == expected,
    }
}
