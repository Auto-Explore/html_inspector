use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

use super::shared::normalize_name;

#[derive(Default)]
pub struct PhrasingParentChildConstraints;

impl Rule for PhrasingParentChildConstraints {
    fn id(&self) -> &'static str {
        "html.phrasing_parent_child_constraints"
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
        // Match vnu.jar behavior: schema/Assertions checks are skipped inside <template> contents.
        if ctx.has_ancestor("template") {
            return;
        }

        let ParseEvent::StartTag { name, span, .. } = event else {
            return;
        };

        let Some(parent) = ctx.current_parent() else {
            return;
        };

        let disallowed = if ctx.name_is(parent, "label") || ctx.name_is(parent, "button") {
            ctx.name_is(name, "div")
        } else if ctx.name_is(parent, "span") {
            ctx.name_is(name, "div") || ctx.name_is(name, "form") || ctx.name_is(name, "section")
        } else if is_heading(ctx, parent) {
            ctx.name_is(name, "div") || ctx.name_is(name, "section")
        } else {
            false
        };

        if !disallowed {
            return;
        }

        let child = normalize_name(ctx, name);
        let parent = normalize_name(ctx, parent);
        out.push(Message::new(
            "html.child.disallowed_in_phrasing_parent",
            Severity::Error,
            Category::Html,
            format!("Element “{child}” not allowed as child of “{parent}” in this context."),
            *span,
        ));
    }
}

fn is_heading(ctx: &ValidationContext, name: &str) -> bool {
    ctx.name_is(name, "h1")
        || ctx.name_is(name, "h2")
        || ctx.name_is(name, "h3")
        || ctx.name_is(name, "h4")
        || ctx.name_is(name, "h5")
        || ctx.name_is(name, "h6")
}
