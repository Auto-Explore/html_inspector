use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct DlDivChildContent {
    stack: Vec<DivFrame>,
}

#[derive(Clone, Copy, Debug)]
struct DivFrame {
    span: Option<html_inspector_core::Span>,
}

impl Rule for DlDivChildContent {
    fn id(&self) -> &'static str {
        "html.dl.div.child_content"
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
            ParseEvent::StartTag {
                name,
                self_closing,
                span,
                ..
            } => {
                if is_dl_div_group_div(ctx, name) {
                    if !*self_closing {
                        self.stack.push(DivFrame { span: *span });
                    }
                    return;
                }

                let Some(frame) = self.stack.last().copied() else {
                    return;
                };
                if is(ctx, name, "dt")
                    || is(ctx, name, "dd")
                    || is(ctx, name, "script")
                    || is(ctx, name, "template")
                {
                    return;
                }
                out.push(Message::new(
                    "html.dl.div.child.invalid",
                    Severity::Error,
                    Category::Html,
                    format!("Element “{name}” not allowed as child of “div” in this context."),
                    span.or(frame.span),
                ));
            }
            ParseEvent::Text { text, .. } => {
                let Some(frame) = self.stack.last().copied() else {
                    return;
                };
                if !ctx.current_parent().is_some_and(|p| is(ctx, p, "div")) {
                    return;
                }
                if text.chars().any(|c| !c.is_whitespace()) {
                    out.push(Message::new(
                        "html.dl.div.text.disallowed",
                        Severity::Error,
                        Category::Html,
                        "Text not allowed in “div” in this context.",
                        frame.span,
                    ));
                }
            }
            ParseEvent::EndTag { name, .. } => {
                if is(ctx, name, "div") && !self.stack.is_empty() {
                    self.stack.pop();
                }
            }
            _ => {}
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, _out: &mut dyn MessageSink) {
        self.stack.clear();
    }
}

fn is_dl_div_group_div(ctx: &ValidationContext, name: &str) -> bool {
    if !is(ctx, name, "div") {
        return false;
    }
    ctx.current_parent().is_some_and(|p| is(ctx, p, "dl")) && !ctx.has_ancestor("template")
}

fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    match ctx.format {
        html_inspector_core::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector_core::InputFormat::Xhtml => actual == expected,
    }
}
