use html_inspector_core::{
    Category, DocumentSection, Interest, Message, MessageSink, ParseEvent, Rule, Severity,
    ValidationContext,
};

#[derive(Default)]
pub struct BaseInBody;

impl Rule for BaseInBody {
    fn id(&self) -> &'static str {
        "html.base.not_in_body"
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
        let is_base = match ctx.format {
            html_inspector_core::InputFormat::Html => name.eq_ignore_ascii_case("base"),
            html_inspector_core::InputFormat::Xhtml => name == "base",
        };
        if !is_base {
            return;
        }

        if ctx.document.section == DocumentSection::Body {
            out.push(Message::new(
                "html.base.not_in_body",
                Severity::Error,
                Category::Html,
                "Element “base” not allowed as child of “body” in this context.",
                *span,
            ));
        }
    }
}
