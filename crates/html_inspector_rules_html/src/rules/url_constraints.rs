use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct UrlConstraints;

impl Rule for UrlConstraints {
    fn id(&self) -> &'static str {
        "html.url.constraints"
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

        // Minimal suite coverage: link[href] must not be empty.
        if is(ctx, name, "link") {
            let href = attrs
                .iter()
                .find(|a| match ctx.format {
                    html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case("href"),
                    html_inspector_core::InputFormat::Xhtml => a.name == "href",
                })
                .and_then(|a| a.value.as_deref());
            if let Some(href) = href
                && href.is_empty() {
                    out.push(Message::new(
                        "html.url.empty",
                        Severity::Error,
                        Category::Html,
                        "Bad value “” for attribute “href” on element “link”.",
                        *span,
                    ));
                }
        }
    }
}

fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    match ctx.format {
        html_inspector_core::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector_core::InputFormat::Xhtml => actual == expected,
    }
}
