use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct InputAttributeAllowedTypes;

impl Rule for InputAttributeAllowedTypes {
    fn id(&self) -> &'static str {
        "html.input.attribute_allowed_types"
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

        let input_type = attr_value(ctx, attrs, "type").unwrap_or("text");
        let input_type_lc = input_type.to_ascii_lowercase();
        let is_hidden = input_type_lc == "hidden";

        if has_attr(ctx, attrs, "readonly") && !READONLY_ALLOWED.contains(&input_type_lc.as_str()) {
            out.push(Message::new(
                "html.input.readonly.disallowed_for_type",
                Severity::Error,
                Category::Html,
                if is_hidden {
                    "Attribute “readonly” not allowed on element “input” at this point."
                } else {
                    "Attribute “readonly” is only allowed when the input type is “date”, “datetime-local”, “email”, “month”, “number”, “password”, “search”, “tel”, “text”, “time”, “url”, or “week”."
                },
                *span,
            ));
        }

        if has_attr(ctx, attrs, "required") && !REQUIRED_ALLOWED.contains(&input_type_lc.as_str()) {
            out.push(Message::new(
                "html.input.required.disallowed_for_type",
                Severity::Error,
                Category::Html,
                if is_hidden {
                    "Attribute “required” not allowed on element “input” at this point."
                } else {
                    "Attribute “required” is only allowed when the input type is “checkbox”, “date”, “datetime-local”, “email”, “file”, “month”, “number”, “password”, “radio”, “search”, “tel”, “text”, “time”, “url”, or “week”."
                },
                *span,
            ));
        }

        if has_attr(ctx, attrs, "pattern") && !PATTERN_ALLOWED.contains(&input_type_lc.as_str()) {
            out.push(Message::new(
                "html.input.pattern.disallowed_for_type",
                Severity::Error,
                Category::Html,
                if is_hidden {
                    "Attribute “pattern” not allowed on element “input” at this point."
                } else {
                    "Attribute “pattern” is only allowed when the input type is “email”, “password”, “search”, “tel”, “text”, or “url”."
                },
                *span,
            ));
        }

        if has_attr(ctx, attrs, "maxlength") && !MAXLENGTH_ALLOWED.contains(&input_type_lc.as_str())
        {
            out.push(Message::new(
                "html.input.maxlength.disallowed_for_type",
                Severity::Error,
                Category::Html,
                if is_hidden {
                    "Attribute “maxlength” not allowed on element “input” at this point."
                } else {
                    "Attribute “maxlength” is only allowed when the input type is “email”, “password”, “search”, “tel”, “text”, or “url”."
                },
                *span,
            ));
        }
    }
}

const READONLY_ALLOWED: [&str; 12] = [
    "date",
    "datetime-local",
    "email",
    "month",
    "number",
    "password",
    "search",
    "tel",
    "text",
    "time",
    "url",
    "week",
];

const REQUIRED_ALLOWED: [&str; 15] = [
    "checkbox",
    "date",
    "datetime-local",
    "email",
    "file",
    "month",
    "number",
    "password",
    "radio",
    "search",
    "tel",
    "text",
    "time",
    "url",
    "week",
];

const PATTERN_ALLOWED: [&str; 6] = ["email", "password", "search", "tel", "text", "url"];
const MAXLENGTH_ALLOWED: [&str; 6] = ["email", "password", "search", "tel", "text", "url"];

fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    match ctx.format {
        html_inspector_core::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector_core::InputFormat::Xhtml => actual == expected,
    }
}

fn has_attr(
    ctx: &ValidationContext,
    attrs: &[html_inspector_core::Attribute],
    needle: &str,
) -> bool {
    attrs.iter().any(|a| match ctx.format {
        html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case(needle),
        html_inspector_core::InputFormat::Xhtml => a.name == needle,
    })
}

fn attr_value<'a>(
    ctx: &ValidationContext,
    attrs: &'a [html_inspector_core::Attribute],
    needle: &str,
) -> Option<&'a str> {
    attrs
        .iter()
        .find(|a| match ctx.format {
            html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case(needle),
            html_inspector_core::InputFormat::Xhtml => a.name == needle,
        })
        .and_then(|a| a.value.as_deref())
}
