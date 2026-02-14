use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

use super::url_attr;

#[derive(Default)]
pub struct ScriptSrcConstraints;

impl Rule for ScriptSrcConstraints {
    fn id(&self) -> &'static str {
        "html.script.src.datatype"
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
        if !ctx.name_is(name, "script") {
            return;
        }

        let src = ctx.attr_value(attrs, "src");
        let Some(src) = src else { return };

        if src.is_empty() {
            out.push(Message::new(
                "html.script.src.empty",
                Severity::Error,
                Category::Html,
                "Bad value “” for attribute “src” on element “script”.",
                *span,
            ));
            return;
        }

        let _ = url_attr::validate_url_attr_value(
            src,
            "src",
            "script",
            "html.script.src.invalid",
            *span,
            out,
        );
    }
}
