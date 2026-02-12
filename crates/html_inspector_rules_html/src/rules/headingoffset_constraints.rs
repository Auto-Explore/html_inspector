use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct HeadingoffsetConstraints;

impl Rule for HeadingoffsetConstraints {
    fn id(&self) -> &'static str {
        "html.headingoffset.constraints"
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
        let ParseEvent::StartTag { attrs, span, .. } = event else {
            return;
        };

        let value = attrs
            .iter()
            .find(|a| match ctx.format {
                html_inspector_core::InputFormat::Html => {
                    a.name.eq_ignore_ascii_case("headingoffset")
                }
                html_inspector_core::InputFormat::Xhtml => a.name == "headingoffset",
            })
            .and_then(|a| a.value.as_deref());

        let Some(value) = value else { return };

        let n = value.trim().parse::<i32>().ok();
        let ok = n.is_some_and(|n| (0..=8).contains(&n));
        if !ok {
            out.push(Message::new(
                "html.headingoffset.range",
                Severity::Error,
                Category::Html,
                "The value of the “headingoffset” attribute must be a number between “0” and “8”.",
                *span,
            ));
        }
    }
}
