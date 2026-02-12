use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

use super::url_attr;

#[derive(Default)]
pub struct FormActionConstraints;

impl Rule for FormActionConstraints {
    fn id(&self) -> &'static str {
        "html.form.action.constraints"
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
        if !ctx.name_is(name, "form") {
            return;
        }

        if let Some(ac) = ctx.attr_value(attrs, "accept-charset") {
            let v = ac.trim().to_ascii_lowercase();
            if v != "utf-8" {
                out.push(Message::new(
                    "html.form.accept_charset.only_utf8",
                    Severity::Error,
                    Category::Html,
                    "The only allowed value for the “accept-charset” attribute for the “form” element is “utf-8”.",
                    *span,
                ));
            }
        }

        if let Some(action) = ctx.attr_value(attrs, "action") {
            if action.is_empty() {
                out.push(Message::new(
                    "html.form.action.empty",
                    Severity::Error,
                    Category::Html,
                    "Bad value “” for attribute “action” on element “form”.",
                    *span,
                ));
                return;
            }
            let _ = url_attr::validate_url_attr_value(
                action,
                "action",
                "form",
                "html.form.action.invalid",
                *span,
                out,
            );
        }
    }
}
