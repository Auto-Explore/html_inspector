use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct ImgUsemapAnchorDescendant {
    open_anchors: usize,
}

impl Rule for ImgUsemapAnchorDescendant {
    fn id(&self) -> &'static str {
        "html.img.usemap.anchor_descendant"
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
                if is(ctx, name, "a") && !*self_closing {
                    self.open_anchors += 1;
                }

                if is(ctx, name, "img") && has_attr(ctx, attrs, "usemap") && self.open_anchors != 0
                {
                    out.push(Message::new(
                        "html.img.usemap.descendant_of_a",
                        Severity::Error,
                        Category::Html,
                        "The element “img” with the attribute “usemap” must not appear as a descendant of the “a” element.",
                        *span,
                    ));
                }
            }
            ParseEvent::EndTag { name, .. } => {
                if is(ctx, name, "a") && self.open_anchors != 0 {
                    self.open_anchors -= 1;
                }
            }
            _ => {}
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, _out: &mut dyn MessageSink) {
        self.open_anchors = 0;
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
