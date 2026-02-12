use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

use super::foreign_content::{namespace_for_next_start_tag, Namespace};

#[derive(Default)]
pub struct PDisallowedParentConstraints;

impl Rule for PDisallowedParentConstraints {
    fn id(&self) -> &'static str {
        "html.p.disallowed_parent"
    }

    fn interest(&self) -> Interest {
        Interest::START_TAG
    }

    fn on_event(
        &mut self,
        event: &ParseEvent,
        ctx: &mut ValidationContext,
        out: &mut dyn MessageSink,
    ) {
        let ParseEvent::StartTag { name, span, .. } = event else {
            return;
        };

        if namespace_for_next_start_tag(ctx, name) != Namespace::Html {
            return;
        }
        if name != "p" {
            return;
        }

        let Some(parent) = ctx.current_parent() else {
            return;
        };

        if parent == "strong" {
            out.push(Message::new(
                "html.p.parent_strong",
                Severity::Error,
                Category::Html,
                "Element “p” not allowed as child of “strong” in this context.",
                *span,
            ));
            return;
        }

        if parent == "span" {
            out.push(Message::new(
                "html.p.parent_span",
                Severity::Error,
                Category::Html,
                "Element “p” not allowed as child of “span” in this context.",
                *span,
            ));
        }
    }
}
