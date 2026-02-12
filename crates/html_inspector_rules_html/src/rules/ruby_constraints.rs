use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, Span, ValidationContext,
};

#[derive(Default)]
pub struct RubyConstraints {
    stack: Vec<RubyState>,
}

#[derive(Default)]
struct RubyState {
    rp_count: usize,
    rt_has_content: bool,
    rt_depth: usize,
    rt_current_has_content: bool,
    span: Option<Span>,
}

impl Rule for RubyConstraints {
    fn id(&self) -> &'static str {
        "html.ruby.constraints"
    }

    fn interest(&self) -> Interest {
        Interest::START_TAG | Interest::END_TAG | Interest::TEXT
    }

    fn on_event(
        &mut self,
        event: &ParseEvent,
        ctx: &mut ValidationContext,
        out: &mut dyn MessageSink,
    ) {
        match event {
            ParseEvent::StartTag { name, span, .. } => {
                if is(ctx, name, "ruby") {
                    self.stack.push(RubyState {
                        span: *span,
                        ..RubyState::default()
                    });
                    return;
                }
                let Some(state) = self.stack.last_mut() else {
                    return;
                };
                if is(ctx, name, "rp") {
                    state.rp_count += 1;
                } else if is(ctx, name, "rt") {
                    state.rt_depth += 1;
                    if state.rt_depth == 1 {
                        state.rt_current_has_content = false;
                    }
                } else if state.rt_depth > 0 {
                    state.rt_current_has_content = true;
                }
            }
            ParseEvent::Text { text, .. } => {
                let Some(state) = self.stack.last_mut() else {
                    return;
                };
                if state.rt_depth == 0 {
                    return;
                }
                if text.chars().any(|c| !c.is_whitespace()) {
                    state.rt_current_has_content = true;
                }
            }
            ParseEvent::EndTag { name, .. } => {
                if is(ctx, name, "rt") {
                    let Some(state) = self.stack.last_mut() else {
                        return;
                    };
                    if state.rt_depth > 0 {
                        state.rt_depth -= 1;
                        if state.rt_depth == 0 && state.rt_current_has_content {
                            state.rt_has_content = true;
                        }
                    }
                    return;
                }
                if is(ctx, name, "ruby") {
                    let Some(state) = self.stack.pop() else {
                        return;
                    };
                    if state.rt_has_content {
                        return;
                    }

                    if state.rp_count == 0 {
                        out.push(Message::new(
                            "html.ruby.missing.rp_rt",
                            Severity::Error,
                            Category::Html,
                            "Element “ruby” is missing a required instance of one or more of the following child elements: “rp”, “rt”.",
                            state.span,
                        ));
                    } else {
                        out.push(Message::new(
                            "html.ruby.missing.rt",
                            Severity::Error,
                            Category::Html,
                            "Element “ruby” is missing a required instance of child element “rt”.",
                            state.span,
                        ));
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
