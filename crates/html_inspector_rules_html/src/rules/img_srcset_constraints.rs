use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

use super::srcset_microsyntax;

#[derive(Default)]
pub struct ImgSrcsetConstraints;

impl Rule for ImgSrcsetConstraints {
    fn id(&self) -> &'static str {
        "html.img.srcset_constraints"
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

        let Some(srcset) = attr_value(ctx, attrs, "srcset") else {
            return;
        };
        let sizes_present = has_attr(ctx, attrs, "sizes");

        if srcset_microsyntax::is_invalid_srcset(srcset, sizes_present) {
            out.push(Message::new(
                "html.img.srcset.invalid",
                Severity::Error,
                Category::Html,
                format!("Bad value “{srcset}” for attribute “srcset” on element “img”."),
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
