use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct ElementSpecificAttributes;

impl Rule for ElementSpecificAttributes {
    fn id(&self) -> &'static str {
        "html.element_specific_attributes"
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

        if is(ctx, name, "div") {
            let has_name = has_attr(ctx, attrs, "name");
            if has_name {
                out.push(Message::new(
                    "html.div.name.disallowed",
                    Severity::Error,
                    Category::Html,
                    "Attribute “name” not allowed on element “div” at this point.",
                    *span,
                ));
            }
            return;
        }

        if is(ctx, name, "a") {
            if has_attr(ctx, attrs, "media") {
                out.push(Message::new(
                    "html.a.media.disallowed",
                    Severity::Error,
                    Category::Html,
                    "Attribute “media” not allowed on element “a” at this point.",
                    *span,
                ));
            }

            if has_attr(ctx, attrs, "name") {
                out.push(Message::new(
                    "html.a.name.obsolete",
                    Severity::Warning,
                    Category::Html,
                    "The “name” attribute on the “a” element is obsolete. Consider putting an “id” attribute on the nearest container instead.",
                    *span,
                ));
            }

            if has_attr(ctx, attrs, "ping") && !has_attr(ctx, attrs, "href") {
                out.push(Message::new(
                    "html.a.ping.requires_href",
                    Severity::Error,
                    Category::Html,
                    "Element “a” is missing required attribute “href”.",
                    *span,
                ));
            }
            return;
        }

        if is(ctx, name, "area") {
            if has_attr(ctx, attrs, "media") {
                out.push(Message::new(
                    "html.area.media.disallowed",
                    Severity::Error,
                    Category::Html,
                    "Attribute “media” not allowed on element “area” at this point.",
                    *span,
                ));
            }
            return;
        }

        if is(ctx, name, "img") {
            if has_attr(ctx, attrs, "type") {
                out.push(Message::new(
                    "html.img.type.disallowed",
                    Severity::Error,
                    Category::Html,
                    "Attribute “type” not allowed on element “img” at this point.",
                    *span,
                ));
            }
            if has_attr(ctx, attrs, "size") {
                out.push(Message::new(
                    "html.img.size.disallowed",
                    Severity::Error,
                    Category::Html,
                    "Attribute “size” not allowed on element “img” at this point.",
                    *span,
                ));
            }
            return;
        }

        if is(ctx, name, "link") {
            if has_attr(ctx, attrs, "srcset") {
                out.push(Message::new(
                    "html.link.srcset.disallowed",
                    Severity::Error,
                    Category::Html,
                    "Attribute “srcset” not allowed on element “link” at this point.",
                    *span,
                ));
            }
            return;
        }

        if is(ctx, name, "video") {
            if has_attr(ctx, attrs, "srcset") {
                out.push(Message::new(
                    "html.video.srcset.disallowed",
                    Severity::Error,
                    Category::Html,
                    "Attribute “srcset” not allowed on element “video” at this point.",
                    *span,
                ));
            }
            return;
        }

        if is(ctx, name, "object") {
            if has_attr(ctx, attrs, "srcset") {
                out.push(Message::new(
                    "html.object.srcset.disallowed",
                    Severity::Error,
                    Category::Html,
                    "Attribute “srcset” not allowed on element “object” at this point.",
                    *span,
                ));
            }
            return;
        }

        if is(ctx, name, "audio") {
            if has_attr(ctx, attrs, "srcset") {
                out.push(Message::new(
                    "html.audio.srcset.disallowed",
                    Severity::Error,
                    Category::Html,
                    "Attribute “srcset” not allowed on element “audio” at this point.",
                    *span,
                ));
            }
            return;
        }

        if is(ctx, name, "track") && has_attr(ctx, attrs, "srcset") {
            out.push(Message::new(
                "html.track.srcset.disallowed",
                Severity::Error,
                Category::Html,
                "Attribute “srcset” not allowed on element “track” at this point.",
                *span,
            ));
        }

        if is(ctx, name, "section") && has_attr(ctx, attrs, "width") {
            out.push(Message::new(
                "html.section.width.disallowed",
                Severity::Error,
                Category::Html,
                "Attribute “width” not allowed on element “section” at this point.",
                *span,
            ));
        }
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
