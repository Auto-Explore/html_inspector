use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

use super::url_attr;

#[derive(Default)]
pub struct ImgSrcConstraints;

impl Rule for ImgSrcConstraints {
    fn id(&self) -> &'static str {
        "html.img.src_constraints"
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
        if !ctx.name_is(name, "img") {
            return;
        }

        let srcset_present = ctx.has_attr(attrs, "srcset");

        let src = ctx.attr_value(attrs, "src");
        let Some(src) = src else {
            if !srcset_present {
                out.push(Message::new(
                    "html.img.src_or_srcset.missing",
                    Severity::Error,
                    Category::Html,
                    "Element “img” is missing one or more of the following attributes: “src”, “srcset”.",
                    *span,
                ));
            }
            return;
        };

        if src.is_empty() {
            out.push(Message::new(
                "html.img.src.empty",
                Severity::Error,
                Category::Html,
                "Bad value “” for attribute “src” on element “img”.",
                *span,
            ));
            return;
        }

        let _ = url_attr::validate_url_attr_value(
            src,
            "src",
            "img",
            "html.img.src.invalid",
            *span,
            out,
        );
    }
}
