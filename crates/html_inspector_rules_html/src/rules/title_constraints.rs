use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, Span, ValidationContext,
};

use super::foreign_content::{self, Namespace};

#[derive(Default)]
pub struct TitleConstraints {
    in_title: bool,
    saw_title: bool,
    title_has_non_whitespace: bool,
    title_span: Option<Span>,
}

impl Rule for TitleConstraints {
    fn id(&self) -> &'static str {
        "html.title.constraints"
    }

    fn interest(&self) -> Interest {
        Interest::START_TAG | Interest::END_TAG | Interest::TEXT | Interest::FINISH
    }

    fn init(&mut self, _ctx: &ValidationContext) {
        self.in_title = false;
        self.saw_title = false;
        self.title_has_non_whitespace = false;
        self.title_span = None;
    }

    fn on_event(
        &mut self,
        event: &ParseEvent,
        ctx: &mut ValidationContext,
        out: &mut dyn MessageSink,
    ) {
        match event {
            ParseEvent::StartTag { name, span, .. } => {
                if ctx.name_is(name, "title")
                    && foreign_content::namespace_for_next_start_tag(ctx, name) == Namespace::Html
                {
                    self.saw_title = true;
                    self.in_title = true;
                    self.title_has_non_whitespace = false;
                    self.title_span = *span;
                }
            }
            ParseEvent::Text { text, .. } => {
                if self.in_title && text.chars().any(|c| !c.is_whitespace()) {
                    self.title_has_non_whitespace = true;
                }
            }
            ParseEvent::EndTag { name, .. } => {
                if ctx.name_is(name, "title") && self.in_title {
                    self.in_title = false;
                    if !self.title_has_non_whitespace {
                        out.push(Message::new(
                            "html.title.empty",
                            Severity::Error,
                            Category::Html,
                            "Element “title” must not be empty.",
                            self.title_span,
                        ));
                    }
                    self.title_span = None;
                }
            }
            _ => {}
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, out: &mut dyn MessageSink) {
        if !self.saw_title {
            out.push(Message::new(
                "html.head.title.missing",
                Severity::Error,
                Category::Html,
                "Element “head” is missing a required instance of child element “title”.",
                None,
            ));
        }
    }
}
