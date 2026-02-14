use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct EmptyHeadingWarning {
    stack: Vec<HeadingState>,
}

#[derive(Clone, Debug)]
struct HeadingState {
    level: u8,
    has_content: bool,
}

impl Rule for EmptyHeadingWarning {
    fn id(&self) -> &'static str {
        "html.heading.empty_warning"
    }

    fn max_severity(&self) -> Severity {
        Severity::Warning
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
            ParseEvent::StartTag { name, .. } => {
                // Any element child counts as content for the heading.
                if let Some(top) = self.stack.last_mut() {
                    top.has_content = true;
                }

                if let Some(level) = heading_level(ctx, name) {
                    self.stack.push(HeadingState {
                        level,
                        has_content: false,
                    });
                }
            }
            ParseEvent::Text { text, .. } => {
                if !text.trim().is_empty()
                    && let Some(top) = self.stack.last_mut()
                {
                    top.has_content = true;
                }
            }
            ParseEvent::EndTag { name, span } => {
                let Some(level) = heading_level(ctx, name) else {
                    return;
                };

                if let Some(pos) = self.stack.iter().rposition(|s| s.level == level) {
                    // Pop any unclosed nested headings too, warning for each empty one.
                    while self.stack.len() > pos {
                        let state = self
                            .stack
                            .pop()
                            .expect("stack length checked before popping");
                        if !state.has_content {
                            out.push(Message::new(
                                "html.heading.empty",
                                Severity::Warning,
                                Category::Html,
                                "Empty heading.",
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

fn heading_level(ctx: &ValidationContext, name: &str) -> Option<u8> {
    if ctx.name_is(name, "h1") {
        Some(1)
    } else if ctx.name_is(name, "h2") {
        Some(2)
    } else if ctx.name_is(name, "h3") {
        Some(3)
    } else if ctx.name_is(name, "h4") {
        Some(4)
    } else if ctx.name_is(name, "h5") {
        Some(5)
    } else if ctx.name_is(name, "h6") {
        Some(6)
    } else {
        None
    }
}
