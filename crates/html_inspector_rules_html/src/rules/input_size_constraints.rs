use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct InputSizeConstraints;

impl Rule for InputSizeConstraints {
    fn id(&self) -> &'static str {
        "html.input.size.datatype"
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

        let size = attrs
            .iter()
            .find(|a| match ctx.format {
                html_inspector::InputFormat::Html => a.name.eq_ignore_ascii_case("size"),
                html_inspector::InputFormat::Xhtml => a.name == "size",
            })
            .and_then(|a| a.value.as_deref());
        let Some(size) = size else { return };
        let v = size;

        let input_type = attrs
            .iter()
            .find(|a| match ctx.format {
                html_inspector::InputFormat::Html => a.name.eq_ignore_ascii_case("type"),
                html_inspector::InputFormat::Xhtml => a.name == "type",
            })
            .and_then(|a| a.value.as_deref())
            .unwrap_or("text");
        let size_allowed = input_type.eq_ignore_ascii_case("text")
            || input_type.eq_ignore_ascii_case("search")
            || input_type.eq_ignore_ascii_case("tel")
            || input_type.eq_ignore_ascii_case("url")
            || input_type.eq_ignore_ascii_case("email")
            || input_type.eq_ignore_ascii_case("password");
        if !size_allowed {
            out.push(Message::new(
                "html.input.size.disallowed_for_type",
                Severity::Error,
                Category::Html,
                "Attribute “size” not allowed on element “input” at this point.",
                *span,
            ));
            return;
        }

        let ok = !v.is_empty()
            && v.as_bytes().iter().all(|b| b.is_ascii_digit())
            && v.parse::<u32>().ok().is_some_and(|n| n > 0);
        if !ok {
            out.push(Message::new(
                "html.input.size.invalid",
                Severity::Error,
                Category::Html,
                format!("Bad value “{v}” for attribute “size” on element “input”."),
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
