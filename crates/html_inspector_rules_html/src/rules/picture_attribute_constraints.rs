use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct PictureAttributeConstraints;

impl Rule for PictureAttributeConstraints {
    fn id(&self) -> &'static str {
        "html.picture.attribute_constraints"
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
        if !is(ctx, name, "picture") {
            return;
        }

        for attr in DISALLOWED_ATTRS {
            if has_attr(ctx, attrs, attr) {
                out.push(Message::new(
                    format!("html.picture.attr.{attr}.disallowed"),
                    Severity::Error,
                    Category::Html,
                    format!("Attribute “{attr}” not allowed on element “picture” at this point."),
                    *span,
                ));
            }
        }
    }
}

const DISALLOWED_ATTRS: [&str; 18] = [
    "align",
    "alt",
    "border",
    "crossorigin",
    "height",
    "hspace",
    "ismap",
    "longdesc",
    "lowsrc",
    "media",
    "name",
    "role",
    "sizes",
    "src",
    "srcset",
    "usemap",
    "vspace",
    "width",
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
