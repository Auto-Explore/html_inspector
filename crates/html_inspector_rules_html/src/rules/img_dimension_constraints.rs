use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct ImgDimensionConstraints;

impl Rule for ImgDimensionConstraints {
    fn id(&self) -> &'static str {
        "html.img.dimension_constraints"
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
        if !is(ctx, name, "img") {
            return;
        }

        if let Some(width) = attr_value(ctx, attrs, "width") {
            let w = width.trim();
            if !is_non_negative_integer(w) {
                out.push(Message::new(
                    "html.img.width.bad_value",
                    Severity::Error,
                    Category::Html,
                    format!("Bad value “{w}” for attribute “width” on element “img”."),
                    *span,
                ));
            }
        }

        if let Some(height) = attr_value(ctx, attrs, "height") {
            let h = height.trim();
            if !is_non_negative_integer(h) {
                out.push(Message::new(
                    "html.img.height.bad_value",
                    Severity::Error,
                    Category::Html,
                    format!("Bad value “{h}” for attribute “height” on element “img”."),
                    *span,
                ));
            }
        }
    }
}

fn is_non_negative_integer(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_digit())
}

fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    match ctx.format {
        html_inspector_core::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector_core::InputFormat::Xhtml => actual == expected,
    }
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
