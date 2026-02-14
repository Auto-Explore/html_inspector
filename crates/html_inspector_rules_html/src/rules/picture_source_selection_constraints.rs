use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, Span, ValidationContext,
};

#[derive(Default)]
pub struct PictureSourceSelectionConstraints {
    picture_depth: usize,
    pending_always_matching_sources: Vec<Option<Span>>,
}

impl Rule for PictureSourceSelectionConstraints {
    fn id(&self) -> &'static str {
        "html.picture.source.selection"
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
                    self.pending_always_matching_sources.clear();
                    return;
                }

                if self.picture_depth == 0 {
                    return;
                }

                let is_source = is(ctx, name, "source");
                let is_img = is(ctx, name, "img");
                if !is_source && !is_img {
                    return;
                }

                let has_srcset = has_attr(ctx, attrs, "srcset");
                if !has_srcset {
                    return;
                }

                // Seeing a <source|img srcset> means earlier always-matching sources were pointless.
                if !self.pending_always_matching_sources.is_empty() {
                    for pending_span in self.pending_always_matching_sources.drain(..) {
                        out.push(Message::new(
                            "html.picture.source.always_matching.disallowed",
                            Severity::Error,
                            Category::Html,
                            "A “source” element that has a following sibling “source” element or “img” element with a “srcset” attribute must have a “media” attribute and/or “type” attribute.",
                            pending_span,
                        ));
                    }
                }

                // Only <source> itself can be "always matching"; <img srcset> is just the trigger.
                if !is_source {
                    return;
                }

                if has_attr(ctx, attrs, "media") {
                    let media = attr_value_or_empty(ctx, attrs, "media");
                    if media.trim().is_empty() {
                        out.push(Message::new(
                            "html.picture.source.media.empty",
                            Severity::Error,
                            Category::Html,
                            "Value of “media” attribute here must not be empty.",
                            *span,
                        ));
                        return;
                    }
                }

                let has_media = has_attr(ctx, attrs, "media");
                let has_type = has_attr(ctx, attrs, "type");
                if !has_media && !has_type {
                    self.pending_always_matching_sources.push(*span);
                }
            }
            ParseEvent::EndTag { name, .. } => {
                if is(ctx, name, "picture") && self.picture_depth > 0 {
                    self.picture_depth -= 1;
                    if self.picture_depth == 0 {
                        self.pending_always_matching_sources.clear();
                    }
                }
            }
            _ => {}
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, _out: &mut dyn MessageSink) {
        self.picture_depth = 0;
        self.pending_always_matching_sources.clear();
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

fn attr_value_or_empty<'a>(
    ctx: &ValidationContext,
    attrs: &'a [html_inspector::Attribute],
    needle: &str,
) -> &'a str {
    attrs
        .iter()
        .find(|a| match ctx.format {
            html_inspector::InputFormat::Html => a.name.eq_ignore_ascii_case(needle),
            html_inspector::InputFormat::Xhtml => a.name == needle,
        })
        .and_then(|a| a.value.as_deref())
        .unwrap_or("")
}
