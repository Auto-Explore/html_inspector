use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct SourceAttributeConstraints;

impl Rule for SourceAttributeConstraints {
    fn id(&self) -> &'static str {
        "html.source.attribute_constraints"
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
        if !is(ctx, name, "source") {
            return;
        }

        let Some(parent) = ctx.current_parent() else {
            return;
        };
        if is(ctx, parent, "picture") {
            for a in attrs {
                let attr = a.name.as_str();
                if is_disallowed(ctx, attr, &DISALLOWED_ATTRS_IN_PICTURE) {
                    out.push(Message::new(
                        format!("html.source.attr.{attr}.disallowed"),
                        Severity::Error,
                        Category::Html,
                        format!(
                            "Attribute “{attr}” not allowed on element “source” at this point."
                        ),
                        *span,
                    ));
                }
            }
            return;
        }

        // The vnu suite also asserts that <source> in <video>/<audio> must not have picture-only attributes.
        if is(ctx, parent, "video") || is(ctx, parent, "audio") {
            for a in attrs {
                let attr = a.name.as_str();
                if is_disallowed(ctx, attr, &DISALLOWED_ATTRS_IN_MEDIA) {
                    out.push(Message::new(
                        format!("html.source.attr.{attr}.disallowed"),
                        Severity::Error,
                        Category::Html,
                        format!(
                            "Attribute “{attr}” not allowed on element “source” at this point."
                        ),
                        *span,
                    ));
                }
            }
        }
    }
}

const DISALLOWED_ATTRS_IN_PICTURE: [&str; 12] = [
    "align",
    "alt",
    "border",
    "crossorigin",
    "hspace",
    "ismap",
    "longdesc",
    "name",
    "role",
    "src",
    "usemap",
    "vspace",
];

const DISALLOWED_ATTRS_IN_MEDIA: [&str; 2] = ["sizes", "srcset"];

fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    match ctx.format {
        html_inspector::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector::InputFormat::Xhtml => actual == expected,
    }
}

fn is_disallowed(ctx: &ValidationContext, actual: &str, disallowed: &[&str]) -> bool {
    disallowed.iter().any(|needle| match ctx.format {
        html_inspector::InputFormat::Html => actual.eq_ignore_ascii_case(needle),
        html_inspector::InputFormat::Xhtml => actual == *needle,
    })
}
