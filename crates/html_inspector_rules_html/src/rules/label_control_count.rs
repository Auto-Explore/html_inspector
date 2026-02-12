use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct LabelControlCount {
    stack: Vec<u32>,
}

impl Rule for LabelControlCount {
    fn id(&self) -> &'static str {
        "html.label.one_control"
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
                name, attrs, span, ..
            } => {
                if is(ctx, name, "label") {
                    self.stack.push(0);
                    return;
                }

                let Some(count) = self.stack.last_mut() else {
                    return;
                };
                if is_labelable(ctx, name, attrs) {
                    *count += 1;
                    if *count > 1 {
                        out.push(Message::new(
                            "html.label.multiple_controls",
                            Severity::Error,
                            Category::Html,
                            "The “label” element may contain at most one “button”, “input”, “meter”, “output”, “progress”, “select”, or “textarea” descendant.",
                            *span,
                        ));
                    }
                }
            }
            ParseEvent::EndTag { name, .. } => {
                if is(ctx, name, "label") && !self.stack.is_empty() {
                    self.stack.pop();
                }
            }
            _ => {}
        }
    }
}

fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    match ctx.format {
        html_inspector_core::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector_core::InputFormat::Xhtml => actual == expected,
    }
}

fn is_labelable(
    ctx: &ValidationContext,
    name: &str,
    attrs: &[html_inspector_core::Attribute],
) -> bool {
    if is(ctx, name, "input") {
        let t = attrs
            .iter()
            .find(|a| match ctx.format {
                html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case("type"),
                html_inspector_core::InputFormat::Xhtml => a.name == "type",
            })
            .and_then(|a| a.value.as_deref())
            .unwrap_or("");
        return !t.eq_ignore_ascii_case("hidden");
    }
    is(ctx, name, "textarea")
        || is(ctx, name, "select")
        || is(ctx, name, "button")
        || is(ctx, name, "meter")
        || is(ctx, name, "output")
        || is(ctx, name, "progress")
}
