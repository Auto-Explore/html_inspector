use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct PictureContentModelConstraints {
    stack: Vec<PictureState>,
}

#[derive(Clone, Debug, Default)]
struct PictureState {
    seen_img: bool,
}

impl Rule for PictureContentModelConstraints {
    fn id(&self) -> &'static str {
        "html.picture.content_model"
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
                if is(ctx, name, "picture") {
                    if let Some(parent) = ctx.current_parent()
                        && is(ctx, parent, "picture") {
                            out.push(Message::new(
                                "html.picture.child.picture.disallowed",
                                Severity::Error,
                                Category::Html,
                                "Element “picture” not allowed as child of “picture” in this context.",
                                *span,
                            ));
                        }
                    self.stack.push(PictureState::default());
                    return;
                }

                let Some(parent) = ctx.current_parent() else {
                    return;
                };
                if !is(ctx, parent, "picture") {
                    return;
                }

                let Some(state) = self.stack.last_mut() else {
                    return;
                };

                if is(ctx, name, "script") || is(ctx, name, "template") {
                    return;
                }

                if is(ctx, name, "source") {
                    if state.seen_img {
                        out.push(Message::new(
                            "html.picture.child.source.disallowed_after_img",
                            Severity::Error,
                            Category::Html,
                            "Element “source” not allowed as child of “picture” in this context.",
                            *span,
                        ));
                    }
                    return;
                }

                if is(ctx, name, "img") {
                    if state.seen_img {
                        out.push(Message::new(
                            "html.picture.child.img.disallowed_multiple",
                            Severity::Error,
                            Category::Html,
                            "Element “img” not allowed as child of “picture” in this context.",
                            *span,
                        ));
                    } else {
                        state.seen_img = true;
                    }
                    return;
                }

                let child = normalize_name(ctx, name);
                out.push(Message::new(
                    "html.picture.child.disallowed",
                    Severity::Error,
                    Category::Html,
                    format!("Element “{child}” not allowed as child of “picture” in this context."),
                    *span,
                ));
            }
            ParseEvent::Text { text, span } => {
                let Some(parent) = ctx.current_parent() else {
                    return;
                };
                if !is(ctx, parent, "picture") {
                    return;
                }
                if text.chars().any(|c| !c.is_whitespace()) {
                    out.push(Message::new(
                        "html.picture.text.disallowed",
                        Severity::Error,
                        Category::Html,
                        "Text not allowed in “picture” in this context.",
                        *span,
                    ));
                }
            }
            ParseEvent::EndTag { name, span } => {
                if !is(ctx, name, "picture") {
                    return;
                }
                let Some(state) = self.stack.pop() else {
                    return;
                };
                if !state.seen_img {
                    out.push(Message::new(
                        "html.picture.missing_img",
                        Severity::Error,
                        Category::Html,
                        "Element “picture” is missing a required instance of child element “img”.",
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

fn normalize_name(ctx: &ValidationContext, name: &str) -> String {
    match ctx.format {
        html_inspector_core::InputFormat::Html => name.to_ascii_lowercase(),
        html_inspector_core::InputFormat::Xhtml => name.to_string(),
    }
}

fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    match ctx.format {
        html_inspector_core::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector_core::InputFormat::Xhtml => actual == expected,
    }
}
