use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct MenuConstraints {
    open_menu: usize,
}

impl Rule for MenuConstraints {
    fn id(&self) -> &'static str {
        "html.menu.constraints"
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
                attrs,
                self_closing,
                span,
            } => {
                if is(ctx, name, "menu") {
                    if is_parent_menu(ctx) {
                        out.push(Message::new(
                            "html.menu.child.menu.not_allowed",
                            Severity::Error,
                            Category::Html,
                            "Element “menu” not allowed as child of “menu” in this context.",
                            *span,
                        ));
                        return;
                    }
                    if ctx.format == html_inspector_core::InputFormat::Xhtml
                        && has_attr(ctx, attrs, "type")
                    {
                        out.push(Message::new(
                            "html.menu.type.obsolete",
                            Severity::Warning,
                            Category::Html,
                            "The “type” attribute on the “menu” element is obsolete. Use script to handle “contextmenu” event instead.",
                            *span,
                        ));
                    }
                    if !*self_closing {
                        self.open_menu += 1;
                    }
                    return;
                }

                if is_parent_menu(ctx) && !is(ctx, name, "li") && !is(ctx, name, "script") {
                    out.push(Message::new(
                        "html.menu.child.not_allowed",
                        Severity::Error,
                        Category::Html,
                        format!("Element “{name}” not allowed as child of “menu” in this context."),
                        *span,
                    ));
                }
            }
            ParseEvent::EndTag { name, .. } => {
                if is(ctx, name, "menu") && self.open_menu != 0 {
                    self.open_menu -= 1;
                }
            }
            ParseEvent::Text { text, span } => {
                if is_parent_menu(ctx) && !text.trim().is_empty() {
                    out.push(Message::new(
                        "html.menu.text.not_allowed",
                        Severity::Error,
                        Category::Html,
                        "Text not allowed in “menu” in this context.",
                        *span,
                    ));
                }
            }
            _ => {}
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, _out: &mut dyn MessageSink) {
        self.open_menu = 0;
    }
}

fn is_parent_menu(ctx: &ValidationContext) -> bool {
    ctx.current_parent().is_some_and(|p| match ctx.format {
        html_inspector_core::InputFormat::Html => p.eq_ignore_ascii_case("menu"),
        html_inspector_core::InputFormat::Xhtml => p == "menu",
    })
}

fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    match ctx.format {
        html_inspector_core::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector_core::InputFormat::Xhtml => actual == expected,
    }
}

fn has_attr(
    ctx: &ValidationContext,
    attrs: &[html_inspector_core::Attribute],
    needle: &str,
) -> bool {
    attrs.iter().any(|a| match ctx.format {
        html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case(needle),
        html_inspector_core::InputFormat::Xhtml => a.name == needle,
    })
}
