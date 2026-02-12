use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct AriaTabpanelRequiredForActiveTab {
    saw_active_tab: bool,
    saw_tabpanel: bool,
}

impl Rule for AriaTabpanelRequiredForActiveTab {
    fn id(&self) -> &'static str {
        "aria.tabpanel.required_for_active_tab"
    }

    fn interest(&self) -> Interest {
        Interest::START_TAG
    }

    fn init(&mut self, _ctx: &ValidationContext) {
        self.saw_active_tab = false;
        self.saw_tabpanel = false;
    }

    fn on_event(
        &mut self,
        event: &ParseEvent,
        ctx: &mut ValidationContext,
        _out: &mut dyn MessageSink,
    ) {
        let ParseEvent::StartTag { attrs, .. } = event else {
            return;
        };
        let role = ctx
            .attr_value(attrs, "role")
            .and_then(|v| v.split_ascii_whitespace().next());
        let Some(role) = role else { return };

        if role.eq_ignore_ascii_case("tabpanel") {
            self.saw_tabpanel = true;
        } else if role.eq_ignore_ascii_case("tab") {
            let selected = ctx.attr_value(attrs, "aria-selected").unwrap_or("");
            if selected.trim().eq_ignore_ascii_case("true") {
                self.saw_active_tab = true;
            }
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, out: &mut dyn MessageSink) {
        if self.saw_active_tab && !self.saw_tabpanel {
            out.push(Message::new(
                "aria.tabpanel.required_for_active_tab",
                Severity::Error,
                Category::Aria,
                "Every active “role=tab” element must have a corresponding “role=tabpanel” element.",
                None,
            ));
        }
    }
}
