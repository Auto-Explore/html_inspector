use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

use super::mimetype_constraints::is_mime_type;
use super::url_attr;

#[derive(Default)]
pub struct EmbedConstraints;

impl Rule for EmbedConstraints {
    fn id(&self) -> &'static str {
        "html.embed.constraints"
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
        if !ctx.name_is(name, "embed") {
            return;
        }

        if let Some(h) = ctx.attr_value(attrs, "height") {
            let v = h.trim();
            if v.is_empty() || !v.chars().all(|c| c.is_ascii_digit()) {
                out.push(Message::new(
                    "html.embed.height.invalid",
                    Severity::Error,
                    Category::Html,
                    format!("Bad value “{v}” for attribute “height” on element “embed”."),
                    *span,
                ));
            }
        }

        if let Some(w) = ctx.attr_value(attrs, "width") {
            let v = w.trim();
            if v.is_empty() || !v.chars().all(|c| c.is_ascii_digit()) {
                out.push(Message::new(
                    "html.embed.width.invalid",
                    Severity::Error,
                    Category::Html,
                    format!("Bad value “{v}” for attribute “width” on element “embed”."),
                    *span,
                ));
            }
        }

        if let Some(t) = ctx.attr_value(attrs, "type") {
            let v = t.trim();
            if !is_mime_type(v) {
                out.push(Message::new(
                    "html.embed.type.invalid",
                    Severity::Error,
                    Category::Html,
                    format!("Bad value “{t}” for attribute “type” on element “embed”."),
                    *span,
                ));
            }
        }

        if let Some(src) = ctx.attr_value(attrs, "src") {
            if src.is_empty() {
                out.push(Message::new(
                    "html.embed.src.empty",
                    Severity::Error,
                    Category::Html,
                    "Bad value “” for attribute “src” on element “embed”.",
                    *span,
                ));
            } else {
                let _ = url_attr::validate_url_attr_value(
                    src,
                    "src",
                    "embed",
                    "html.embed.src.invalid",
                    *span,
                    out,
                );
            }
        }
    }
}
