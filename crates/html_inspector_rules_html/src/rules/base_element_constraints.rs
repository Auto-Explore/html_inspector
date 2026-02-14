use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct BaseElementConstraints {
    seen_link_or_script: bool,
}

impl Rule for BaseElementConstraints {
    fn id(&self) -> &'static str {
        "html.base.element_constraints"
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

        if is(ctx, name, "link") || is(ctx, name, "script") {
            self.seen_link_or_script = true;
            return;
        }

        if !is(ctx, name, "base") {
            return;
        }

        let has_href = has_attr(ctx, attrs, "href");
        let has_target = has_attr(ctx, attrs, "target");

        if !has_href && !has_target {
            out.push(Message::new(
                "html.base.missing_href_or_target",
                Severity::Error,
                Category::Html,
                "Element “base” is missing one or more of the following attributes: “href”, “target”.",
                *span,
            ));
        }

        if self.seen_link_or_script {
            out.push(Message::new(
                "html.base.must_come_before_link_or_script",
                Severity::Error,
                Category::Html,
                "The “base” element must come before any “link” or “script” elements in the document.",
                *span,
            ));
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, _out: &mut dyn MessageSink) {
        self.seen_link_or_script = false;
    }
}

fn has_attr(ctx: &ValidationContext, attrs: &[html_inspector::Attribute], needle: &str) -> bool {
    attrs.iter().any(|a| match ctx.format {
        html_inspector::InputFormat::Html => a.name.eq_ignore_ascii_case(needle),
        html_inspector::InputFormat::Xhtml => a.name == needle,
    })
}

fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    match ctx.format {
        html_inspector::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector::InputFormat::Xhtml => actual == expected,
    }
}
