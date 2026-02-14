use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct AriaMultipleMainWarning {
    visible_main_count: u32,
}

impl Rule for AriaMultipleMainWarning {
    fn id(&self) -> &'static str {
        "aria.main.multiple_visible"
    }

    fn max_severity(&self) -> Severity {
        Severity::Warning
    }

    fn interest(&self) -> Interest {
        Interest::START_TAG
    }

    fn init(&mut self, _ctx: &ValidationContext) {
        self.visible_main_count = 0;
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

        if ctx.has_attr(attrs, "hidden") {
            return;
        }

        let role_token_is_main = ctx
            .attr_value(attrs, "role")
            .and_then(|v| v.split_ascii_whitespace().next())
            .is_some_and(|t| t.eq_ignore_ascii_case("main"));

        let is_main_landmark = role_token_is_main || ctx.name_is(name, "main");
        if !is_main_landmark {
            return;
        }

        self.visible_main_count += 1;
        if self.visible_main_count > 1 {
            out.push(Message::new(
                "aria.main.multiple_visible",
                Severity::Warning,
                Category::Aria,
                "A document should not include more than one visible element with “role=main”.",
                *span,
            ));
        }
    }
}
