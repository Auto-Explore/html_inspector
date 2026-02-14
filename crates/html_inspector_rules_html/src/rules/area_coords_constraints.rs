use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct AreaCoordsConstraints;

impl Rule for AreaCoordsConstraints {
    fn id(&self) -> &'static str {
        "html.area.coords.constraints"
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
        if !ctx.name_is(name, "area") {
            return;
        }

        let shape = ctx.attr_value(attrs, "shape").unwrap_or("rect");
        if shape.eq_ignore_ascii_case("default") && ctx.has_attr(attrs, "coords") {
            out.push(Message::new(
                "html.area.coords.disallowed_for_default",
                Severity::Error,
                Category::Html,
                "Attribute “coords” not allowed on element “area” at this point.",
                *span,
            ));
        }
    }
}
