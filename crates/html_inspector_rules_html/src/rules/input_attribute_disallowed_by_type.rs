use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct InputAttributeDisallowedByType;

impl Rule for InputAttributeDisallowedByType {
    fn id(&self) -> &'static str {
        "html.input.attribute_disallowed_by_type"
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

        if has_attr(ctx, attrs, "srcset") {
            out.push(Message::new(
                "html.input.srcset.disallowed",
                Severity::Error,
                Category::Html,
                "Attribute “srcset” not allowed on element “input” at this point.",
                *span,
            ));
        }

        let input_type = attr_value(ctx, attrs, "type").unwrap_or("text");
        let input_type_lc = input_type.to_ascii_lowercase();

        // min/max/step allowed only for a subset of input types.
        if has_attr(ctx, attrs, "min") && !MIN_MAX_ALLOWED.contains(&input_type_lc.as_str()) {
            out.push(Message::new(
                "html.input.min.disallowed_for_type",
                Severity::Error,
                Category::Html,
                "Attribute “min” not allowed on element “input” at this point.",
                *span,
            ));
        }
        if has_attr(ctx, attrs, "max") && !MIN_MAX_ALLOWED.contains(&input_type_lc.as_str()) {
            out.push(Message::new(
                "html.input.max.disallowed_for_type",
                Severity::Error,
                Category::Html,
                "Attribute “max” not allowed on element “input” at this point.",
                *span,
            ));
        }
        if has_attr(ctx, attrs, "step") && !STEP_ALLOWED.contains(&input_type_lc.as_str()) {
            out.push(Message::new(
                "html.input.step.disallowed_for_type",
                Severity::Error,
                Category::Html,
                "Attribute “step” not allowed on element “input” at this point.",
                *span,
            ));
        }

        if has_attr(ctx, attrs, "accept") && input_type_lc != "file" {
            out.push(Message::new(
                "html.input.accept.disallowed_for_type",
                Severity::Error,
                Category::Html,
                "Attribute “accept” not allowed on element “input” at this point.",
                *span,
            ));
        }

        if has_attr(ctx, attrs, "checked")
            && input_type_lc != "checkbox"
            && input_type_lc != "radio"
        {
            out.push(Message::new(
                "html.input.checked.disallowed_for_type",
                Severity::Error,
                Category::Html,
                "Attribute “checked” not allowed on element “input” at this point.",
                *span,
            ));
        }

        if (has_attr(ctx, attrs, "src")
            || has_attr(ctx, attrs, "width")
            || has_attr(ctx, attrs, "height"))
            && input_type_lc != "image"
        {
            if has_attr(ctx, attrs, "src") {
                out.push(Message::new(
                    "html.input.src.disallowed_for_type",
                    Severity::Error,
                    Category::Html,
                    "Attribute “src” not allowed on element “input” at this point.",
                    *span,
                ));
            }
            if has_attr(ctx, attrs, "width") {
                out.push(Message::new(
                    "html.input.width.disallowed_for_type",
                    Severity::Error,
                    Category::Html,
                    "Attribute “width” not allowed on element “input” at this point.",
                    *span,
                ));
            }
            if has_attr(ctx, attrs, "height") {
                out.push(Message::new(
                    "html.input.height.disallowed_for_type",
                    Severity::Error,
                    Category::Html,
                    "Attribute “height” not allowed on element “input” at this point.",
                    *span,
                ));
            }
        }

        if input_type_lc == "hidden" {
            if has_attr(ctx, attrs, "placeholder") {
                out.push(Message::new(
                    "html.input.hidden.placeholder.disallowed",
                    Severity::Error,
                    Category::Html,
                    "Attribute “placeholder” not allowed on element “input” at this point.",
                    *span,
                ));
            }

            let has_aria_hidden = has_attr(ctx, attrs, "aria-hidden");
            if has_aria_hidden {
                out.push(Message::new(
                    "html.input.hidden.aria_hidden.disallowed",
                    Severity::Error,
                    Category::Html,
                    "The “aria-hidden” attribute must not be specified on an “input” element whose “type” attribute has the value “hidden”.",
                    *span,
                ));
            }

            let has_any_aria_except_hidden = attrs
                .iter()
                .any(|a| a.name.starts_with("aria-") && a.name != "aria-hidden");
            if has_any_aria_except_hidden {
                out.push(Message::new(
                    "html.input.hidden.aria.disallowed",
                    Severity::Error,
                    Category::Html,
                    "An “input” element with a “type” attribute whose value is “hidden” must not have any “aria-*” attributes.",
                    *span,
                ));
            }
        }
    }
}

const MIN_MAX_ALLOWED: [&str; 7] = [
    "date",
    "datetime-local",
    "month",
    "number",
    "range",
    "time",
    "week",
];
const STEP_ALLOWED: [&str; 7] = [
    "date",
    "datetime-local",
    "month",
    "number",
    "range",
    "time",
    "week",
];

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
