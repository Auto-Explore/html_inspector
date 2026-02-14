use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, Span, ValidationContext,
};

#[derive(Default)]
pub struct PictureSourceMediaAllConstraints {
    picture_depth: usize,
    pending_media_all_sources: Vec<Option<Span>>,
}

impl Rule for PictureSourceMediaAllConstraints {
    fn id(&self) -> &'static str {
        "html.picture.source.media_all"
    }

    fn interest(&self) -> Interest {
        Interest::START_TAG | Interest::END_TAG
    }

    fn on_event(
        &mut self,
        event: &ParseEvent,
        ctx: &mut ValidationContext,
        out: &mut dyn MessageSink,
    ) {
        match event {
            ParseEvent::StartTag {
                name,
                attrs,
                self_closing,
                span,
            } => {
                if is(ctx, name, "picture") && !*self_closing {
                    self.picture_depth += 1;
                    self.pending_media_all_sources.clear();
                    return;
                }

                if self.picture_depth == 0 {
                    return;
                }

                if !is(ctx, name, "source") {
                    return;
                }

                let has_srcset = has_attr(ctx, attrs, "srcset");
                if !has_srcset {
                    return;
                }

                // Encountering any later <source srcset> means earlier media=all sources were pointless.
                if !self.pending_media_all_sources.is_empty() {
                    for pending_span in self.pending_media_all_sources.drain(..) {
                        out.push(Message::new(
                            "html.picture.source.media_all.disallowed",
                            Severity::Error,
                            Category::Html,
                            "Value of “media” attribute here must not be “all”.",
                            pending_span,
                        ));
                    }
                }

                let media = attr_value(ctx, attrs, "media").unwrap_or("");
                if media.trim().eq_ignore_ascii_case("all") {
                    self.pending_media_all_sources.push(*span);
                }
            }
            ParseEvent::EndTag { name, .. } => {
                if is(ctx, name, "picture") && self.picture_depth > 0 {
                    self.picture_depth -= 1;
                    if self.picture_depth == 0 {
                        self.pending_media_all_sources.clear();
                    }
                }
            }
            _ => {}
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, _out: &mut dyn MessageSink) {
        self.picture_depth = 0;
        self.pending_media_all_sources.clear();
    }
}

fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    match ctx.format {
        html_inspector::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector::InputFormat::Xhtml => actual == expected,
    }
}

fn has_attr(ctx: &ValidationContext, attrs: &[html_inspector::Attribute], needle: &str) -> bool {
    attrs.iter().any(|a| match ctx.format {
        html_inspector::InputFormat::Html => a.name.eq_ignore_ascii_case(needle),
        html_inspector::InputFormat::Xhtml => a.name == needle,
    })
}

fn attr_value<'a>(
    ctx: &ValidationContext,
    attrs: &'a [html_inspector::Attribute],
    needle: &str,
) -> Option<&'a str> {
    attrs
        .iter()
        .find(|a| match ctx.format {
            html_inspector::InputFormat::Html => a.name.eq_ignore_ascii_case(needle),
            html_inspector::InputFormat::Xhtml => a.name == needle,
        })
        .and_then(|a| a.value.as_deref())
}
