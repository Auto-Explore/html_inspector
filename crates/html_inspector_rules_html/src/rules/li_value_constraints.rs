use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct LiValueConstraints;

impl Rule for LiValueConstraints {
    fn id(&self) -> &'static str {
        "html.li.value_constraints"
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
        if !is(ctx, name, "li") {
            return;
        }

        let value = attrs
            .iter()
            .find(|a| match ctx.format {
                html_inspector::InputFormat::Html => a.name.eq_ignore_ascii_case("value"),
                html_inspector::InputFormat::Xhtml => a.name == "value",
            })
            .and_then(|a| a.value.as_deref());
        let Some(value) = value else { return };

        let parent = ctx.current_parent().unwrap_or("");
        if !is(ctx, parent, "ol") {
            out.push(Message::new(
                "html.li.value.disallowed",
                Severity::Error,
                Category::Html,
                "Attribute “value” not allowed on element “li” at this point.",
                *span,
            ));
            return;
        }

        if value.is_empty() || value.parse::<i32>().is_err() {
            out.push(Message::new(
                "html.li.value.invalid",
                Severity::Error,
                Category::Html,
                format!("Bad value “{value}” for attribute “value” on element “li”."),
                *span,
            ));
        }
    }
}

fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    match ctx.format {
        html_inspector::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector::InputFormat::Xhtml => actual == expected,
    }
}
