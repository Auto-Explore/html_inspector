use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct ImgIsmapAnchorAncestor {
    anchor_stack: Vec<AnchorState>,
}

#[derive(Clone, Copy, Debug)]
struct AnchorState {
    has_href: bool,
}

impl Rule for ImgIsmapAnchorAncestor {
    fn id(&self) -> &'static str {
        "html.img.ismap.anchor_ancestor"
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
                    self.anchor_stack.push(AnchorState {
                        has_href: has_attr(ctx, attrs, "href"),
                    });
                }

                if is(ctx, name, "img") && has_attr(ctx, attrs, "ismap") {
                    let ok = self.anchor_stack.iter().any(|a| a.has_href);
                    if !ok {
                        out.push(Message::new(
                            "html.img.ismap.requires_anchor_href",
                            Severity::Error,
                            Category::Html,
                            "The “img” element with the “ismap” attribute set must have an “a” ancestor with the “href” attribute.",
                            *span,
                        ));
                    }
                }
            }
            ParseEvent::EndTag { name, .. } => {
                if is(ctx, name, "a") && !self.anchor_stack.is_empty() {
                    self.anchor_stack.pop();
                }
            }
            _ => {}
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, _out: &mut dyn MessageSink) {
        self.anchor_stack.clear();
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
