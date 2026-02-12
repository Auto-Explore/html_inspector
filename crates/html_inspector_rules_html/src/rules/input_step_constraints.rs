use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct InputStepConstraints;

impl Rule for InputStepConstraints {
    fn id(&self) -> &'static str {
        "html.input.step.datatype"
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
        if !is(ctx, name, "input") {
            return;
        }

        let step = attrs
            .iter()
            .find(|a| match ctx.format {
                html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case("step"),
                html_inspector_core::InputFormat::Xhtml => a.name == "step",
            })
            .and_then(|a| a.value.as_deref());
        let Some(step) = step else { return };
        let v = step.trim();

        if v.is_empty() || (!v.eq_ignore_ascii_case("any") && v.parse::<f64>().is_err()) {
            out.push(Message::new(
                "html.input.step.invalid",
                Severity::Error,
                Category::Html,
                format!("Bad value “{step}” for attribute “step” on element “input”."),
                *span,
            ));
        }
    }
}

fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    match ctx.format {
        html_inspector_core::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector_core::InputFormat::Xhtml => actual == expected,
    }
}
