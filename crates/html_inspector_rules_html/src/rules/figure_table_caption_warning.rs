use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct FigureTableCaptionWarning {
    stack: Vec<FigureState>,
}

#[derive(Clone, Copy, Debug, Default)]
struct FigureState {
    table_depth: usize,
    seen_table: bool,
    seen_figcaption: bool,
    table_has_caption: bool,
    seen_other_content: bool,
}

impl Rule for FigureTableCaptionWarning {
    fn id(&self) -> &'static str {
        "html.figure.table_caption_warning"
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
            ParseEvent::StartTag {
                name,
                span: _,
                self_closing,
                ..
            } => {
                if is(ctx, name, "figure") {
                    if !*self_closing {
                        self.stack.push(FigureState::default());
                    }
                    return;
                }

                let Some(state) = self.stack.last_mut() else {
                    return;
                };

                if is(ctx, name, "table") {
                    state.seen_table = true;
                    if !*self_closing {
                        state.table_depth += 1;
                    }
                    return;
                }

                if is(ctx, name, "figcaption") {
                    state.seen_figcaption = true;
                    return;
                }

                if is(ctx, name, "caption") && state.table_depth > 0 {
                    state.table_has_caption = true;
                    return;
                }

                // Any other element directly under <figure> besides <table>/<figcaption> counts as other content.
                if state.table_depth == 0 && is(ctx, ctx.current_parent().unwrap_or(""), "figure") {
                    state.seen_other_content = true;
                }
            }
            ParseEvent::EndTag { name, span } => {
                if is(ctx, name, "table") {
                    if let Some(state) = self.stack.last_mut() {
                        state.table_depth = state.table_depth.saturating_sub(1);
                    }
                    return;
                }

                if is(ctx, name, "figure")
                    && let Some(state) = self.stack.pop()
                        && state.seen_table
                            && state.seen_figcaption
                            && state.table_has_caption
                            && !state.seen_other_content
                        {
                            out.push(Message::new(
                                "html.figure.table_caption.prefers_figcaption",
                                Severity::Warning,
                                Category::Html,
                                "When a “table” element is the only content in a “figure” element other than the “figcaption”, the “caption” element should be omitted in favor of the “figcaption”.",
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

fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    match ctx.format {
        html_inspector_core::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector_core::InputFormat::Xhtml => actual == expected,
    }
}
