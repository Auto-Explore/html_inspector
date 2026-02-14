use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct FigureFigcaption {
    figure_stack: Vec<FigureState>,
}

#[derive(Clone, Debug)]
struct FigureState {
    seen_figcaption: bool,
    saw_flow_before_figcaption: bool,
    order_error_emitted: bool,
    role_error_emitted: bool,
    role_token: Option<String>,
}

impl Rule for FigureFigcaption {
    fn id(&self) -> &'static str {
        "html.figure.figcaption"
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
                name, attrs, span, ..
            } => {
                if is(ctx, name, "figure") {
                    let role_token = attrs
                        .iter()
                        .find(|a| match ctx.format {
                            html_inspector_core::InputFormat::Html => {
                                a.name.eq_ignore_ascii_case("role")
                            }
                            html_inspector_core::InputFormat::Xhtml => a.name == "role",
                        })
                        .and_then(|a| a.value.as_deref())
                        .and_then(|v| v.split_ascii_whitespace().next())
                        .map(str::to_owned);
                    self.figure_stack.push(FigureState {
                        seen_figcaption: false,
                        saw_flow_before_figcaption: false,
                        order_error_emitted: false,
                        role_error_emitted: false,
                        role_token,
                    });
                    return;
                }

                if is(ctx, name, "figcaption") {
                    if !is_direct_child_of_figure(ctx) {
                        return;
                    }
                    if let Some(state) = self.figure_stack.last_mut() {
                        if state.seen_figcaption {
                            out.push(Message::new(
                                "html.figure.figcaption.multiple",
                                Severity::Error,
                                Category::Html,
                                "Element “figcaption” not allowed as child of “figure” in this context.",
                                *span,
                            ));
                        } else {
                            state.seen_figcaption = true;
                        }

                        if !state.role_error_emitted
                            && let Some(role) = state.role_token.as_deref()
                        {
                            let role_allows_figcaption = role.eq_ignore_ascii_case("figure")
                                || html_inspector_core::starts_with_ascii_ci(role, "doc-");
                            if !role_allows_figcaption {
                                state.role_error_emitted = true;
                                out.push(Message::new(
                                        "html.figure.role.with_figcaption",
                                        Severity::Error,
                                        Category::Html,
                                        "A “figure” element with a “figcaption” descendant must not have a “role” attribute.",
                                        *span,
                                    ));
                            }
                        }
                    }
                    return;
                }

                if let Some(state) = self.figure_stack.last_mut() {
                    if !is_direct_child_of_figure(ctx) {
                        return;
                    }

                    // Content-model constraint: <figcaption> can only be first or last child.
                    // If we've seen flow content before <figcaption>, then any subsequent
                    // flow content after <figcaption> is invalid.
                    if state.seen_figcaption
                        && state.saw_flow_before_figcaption
                        && !state.order_error_emitted
                    {
                        state.order_error_emitted = true;
                        out.push(Message::new(
                            "html.figure.figcaption.order",
                            Severity::Error,
                            Category::Html,
                            format!(
                                "Element “{name}” not allowed as child of “figure” in this context."
                            ),
                            *span,
                        ));
                        return;
                    }

                    // Any non-<figcaption> element directly under <figure> counts as flow content.
                    state.saw_flow_before_figcaption = true;
                }
            }
            ParseEvent::EndTag { name, .. } => {
                if is(ctx, name, "figure") && !self.figure_stack.is_empty() {
                    self.figure_stack.pop();
                }
            }
            ParseEvent::Text { text, span } => {
                let Some(state) = self.figure_stack.last_mut() else {
                    return;
                };
                if !is_direct_child_of_figure(ctx) {
                    return;
                }
                if text.trim().is_empty() {
                    return;
                }

                if state.seen_figcaption
                    && state.saw_flow_before_figcaption
                    && !state.order_error_emitted
                {
                    state.order_error_emitted = true;
                    out.push(Message::new(
                        "html.figure.figcaption.order_text",
                        Severity::Error,
                        Category::Html,
                        "Text not allowed in “figure” in this context.",
                        *span,
                    ));
                    return;
                }

                if !state.seen_figcaption {
                    state.saw_flow_before_figcaption = true;
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

fn is_direct_child_of_figure(ctx: &ValidationContext) -> bool {
    ctx.current_parent().is_some_and(|p| match ctx.format {
        html_inspector_core::InputFormat::Html => p.eq_ignore_ascii_case("figure"),
        html_inspector_core::InputFormat::Xhtml => p == "figure",
    })
}
