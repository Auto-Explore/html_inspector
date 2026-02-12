use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct ADownloadConstraints;

impl Rule for ADownloadConstraints {
    fn id(&self) -> &'static str {
        "html.a.download"
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
        let element = if is(ctx, name, "a") {
            "a"
        } else if is(ctx, name, "area") {
            "area"
        } else {
            return;
        };
        let has_download = attrs.iter().any(|a| match ctx.format {
            html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case("download"),
            html_inspector_core::InputFormat::Xhtml => a.name == "download",
        });
        if !has_download {
            return;
        }
        let has_href = attrs.iter().any(|a| match ctx.format {
            html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case("href"),
            html_inspector_core::InputFormat::Xhtml => a.name == "href",
        });
        if !has_href {
            if element == "area"
                && !(has_attr(ctx, attrs, "coords") || has_attr(ctx, attrs, "shape"))
            {
                // vnu suite: `area[download]` without href is only an error for typical image-map areas (coords/shape present).
                return;
            }
            out.push(Message::new(
                if element == "a" {
                    "html.a.href.required_for_download"
                } else {
                    "html.area.href.required_for_download"
                },
                Severity::Error,
                Category::Html,
                format!("Element “{element}” is missing required attribute “href”."),
                *span,
            ));
        }
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

fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    match ctx.format {
        html_inspector_core::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector_core::InputFormat::Xhtml => actual == expected,
    }
}
