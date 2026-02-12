use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct DetailsSummaryConstraints {
    stack: Vec<DetailsState>,
}

#[derive(Clone, Copy, Debug, Default)]
struct DetailsState {
    saw_element_child: bool,
    saw_summary: bool,
    first_child_was_summary: bool,
}

impl Rule for DetailsSummaryConstraints {
    fn id(&self) -> &'static str {
        "html.details.summary_constraints"
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
                span,
                self_closing,
                ..
            } => {
                if is(ctx, name, "details") {
                    if !*self_closing {
                        self.stack.push(DetailsState::default());
                    }
                    return;
                }

                if let Some(state) = self.stack.last_mut() {
                    if is(ctx, ctx.current_parent().unwrap_or(""), "details") {
                        if !state.saw_element_child {
                            state.saw_element_child = true;
                            if is(ctx, name, "summary") {
                                state.saw_summary = true;
                                state.first_child_was_summary = true;
                            } else {
                                state.first_child_was_summary = false;
                            }
                        } else if is(ctx, name, "summary") {
                            if state.saw_summary {
                                out.push(Message::new(
                                    "html.details.multiple_summary",
                                    Severity::Error,
                                    Category::Html,
                                    "Element “summary” not allowed as child of “details” in this context.",
                                    *span,
                                ));
                            } else {
                                state.saw_summary = true;
                            }
                        }
                    }
                }
            }
            ParseEvent::EndTag { name, span } => {
                if is(ctx, name, "details") {
                    if let Some(state) = self.stack.pop() {
                        if !state.saw_summary || !state.first_child_was_summary {
                            out.push(Message::new(
                                "html.details.missing_summary",
                                Severity::Error,
                                Category::Html,
                                "Element “details” is missing a required instance of child element “summary”.",
                                *span,
                            ));
                        }
                    }
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
