use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct AreaRequiresMapAncestor;

impl Rule for AreaRequiresMapAncestor {
    fn id(&self) -> &'static str {
        "html.area.map_ancestor"
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
        let is_area = match ctx.format {
            html_inspector::InputFormat::Html => name.eq_ignore_ascii_case("area"),
            html_inspector::InputFormat::Xhtml => name == "area",
        };
        if !is_area {
            return;
        }
        if !ctx.has_ancestor("map") {
            out.push(Message::new(
                "html.area.map_ancestor.missing",
                Severity::Error,
                Category::Html,
                "The “area” element must have a “map” ancestor.",
                *span,
            ));
        }
    }
}
