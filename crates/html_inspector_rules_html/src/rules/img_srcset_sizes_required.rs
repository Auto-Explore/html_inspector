use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct ImgSrcsetSizesRequired;

impl Rule for ImgSrcsetSizesRequired {
    fn id(&self) -> &'static str {
        "html.img.srcset_sizes_required"
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

        let srcset = attr_value(ctx, attrs, "srcset");
        let Some(srcset) = srcset else { return };
        if has_attr(ctx, attrs, "sizes") {
            return;
        }

        // vnu treats missing `sizes` as OK for lazy-loaded images with explicit dimensions (suite coverage).
        let loading = attr_value(ctx, attrs, "loading").unwrap_or("");
        let loading_is_lazy = match ctx.format {
            html_inspector_core::InputFormat::Html => loading.eq_ignore_ascii_case("lazy"),
            html_inspector_core::InputFormat::Xhtml => loading == "lazy",
        };
        if loading_is_lazy && has_attr(ctx, attrs, "width") && has_attr(ctx, attrs, "height") {
            return;
        }

        if srcset_has_width_descriptor(srcset) {
            out.push(Message::new(
                "html.img.srcset.width_descriptor_requires_sizes",
                Severity::Error,
                Category::Html,
                "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
                *span,
            ));
        }
    }
}

fn srcset_has_width_descriptor(srcset: &str) -> bool {
    // Very small subset: detect any descriptor token ending with 'w' where preceding chars are digits.
    for candidate in srcset.split(',') {
        let candidate = candidate.trim();
        if candidate.is_empty() {
            continue;
        }
        // tokens: url [descriptor...]
        for token in candidate.split_ascii_whitespace().skip(1) {
            let token = token.trim();
            if let Some(num) = token.strip_suffix('w')
                && !num.is_empty() && num.chars().all(|c| c.is_ascii_digit()) {
                    return true;
                }
        }
    }
    false
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
