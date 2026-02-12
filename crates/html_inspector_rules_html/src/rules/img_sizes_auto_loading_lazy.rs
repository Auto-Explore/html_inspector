use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct ImgSizesAutoRequiresLazyLoading;

impl Rule for ImgSizesAutoRequiresLazyLoading {
    fn id(&self) -> &'static str {
        "html.img.sizes_auto_requires_lazy_loading"
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

        let sizes = attr_value(ctx, attrs, "sizes");
        let Some(sizes) = sizes else { return };

        if !starts_with_auto(sizes) {
            return;
        }

        let loading = attr_value(ctx, attrs, "loading").unwrap_or("");
        let loading_is_lazy = match ctx.format {
            html_inspector_core::InputFormat::Html => loading.eq_ignore_ascii_case("lazy"),
            html_inspector_core::InputFormat::Xhtml => loading == "lazy",
        };

        if !loading_is_lazy {
            out.push(Message::new(
                "html.img.sizes_auto.requires_loading_lazy",
                Severity::Error,
                Category::Html,
                "The “sizes” attribute value starting with “auto” is only valid for lazy-loaded images. Add “loading=”“lazy” to this element.",
                *span,
            ));
        }
    }
}

fn starts_with_auto(s: &str) -> bool {
    let trimmed = s.trim_start();
    let lower = trimmed.to_ascii_lowercase();
    if !lower.starts_with("auto") {
        return false;
    }
    match trimmed.as_bytes().get(4) {
        None => true,
        Some(b) => b.is_ascii_whitespace() || *b == b',',
    }
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
