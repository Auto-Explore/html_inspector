use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct AHrefButtonDescendant;

impl Rule for AHrefButtonDescendant {
    fn id(&self) -> &'static str {
        "html.a.href.button_descendant"
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
        let ParseEvent::StartTag {
            name, attrs, span, ..
        } = event
        else {
            return;
        };
        if !is(ctx, name, "a") {
            return;
        }
        if !has_attr(ctx, attrs, "href") {
            return;
        }
        // Match vnu.jar behavior: schema/Assertions checks are skipped inside <template> contents.
        if ctx.has_ancestor("template") {
            return;
        }
        if ctx.has_ancestor("button") {
            out.push(Message::new(
                "html.a.href.button_descendant",
                Severity::Error,
                Category::Html,
                "The element “a” with the attribute “href” must not appear as a descendant of the “button” element.",
                *span,
            ));
        }
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

fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    match ctx.format {
        html_inspector_core::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector_core::InputFormat::Xhtml => actual == expected,
    }
}
