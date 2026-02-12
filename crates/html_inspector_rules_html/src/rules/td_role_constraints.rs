use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct TdRoleConstraints {
    table_disallows_td_role_stack: Vec<bool>,
}

impl Rule for TdRoleConstraints {
    fn id(&self) -> &'static str {
        "html.td.role_constraints"
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
                span,
                self_closing,
            } => {
                if ctx.name_is(name, "table") && !*self_closing {
                    let disallows_td_role = match ctx.attr_value(attrs, "role") {
                        None => true,
                        Some(role) => {
                            role.eq_ignore_ascii_case("table")
                                || role.eq_ignore_ascii_case("grid")
                                || role.eq_ignore_ascii_case("treegrid")
                        }
                    };
                    self.table_disallows_td_role_stack.push(disallows_td_role);
                    return;
                }

                if ctx.name_is(name, "td") && ctx.has_attr(attrs, "role") {
                    let Some(disallows_td_role) =
                        self.table_disallows_td_role_stack.last().copied()
                    else {
                        return;
                    };
                    if disallows_td_role {
                        out.push(Message::new(
                            "html.td.role.disallowed_in_role_table",
                            Severity::Error,
                            Category::Html,
                            "The “role” attribute must not be used on a “td” element which has a “table” ancestor with no “role” attribute, or with a “role” attribute whose value is “table”, “grid”, or “treegrid”.",
                            *span,
                        ));
                    }
                }
            }
            ParseEvent::EndTag { name, .. } => {
                if ctx.name_is(name, "table") {
                    let _ = self.table_disallows_td_role_stack.pop();
                }
            }
            _ => {}
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, _out: &mut dyn MessageSink) {
        self.table_disallows_td_role_stack.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::TdRoleConstraints;
    use html_inspector_core::{Config, InputFormat, RuleSet};
    use html_inspector_html::HtmlEventSource;

    fn run(format: InputFormat, html: &str) -> html_inspector_core::Report {
        let src = HtmlEventSource::from_str("t", format, html).unwrap();
        let rules = RuleSet::new().push(TdRoleConstraints::default());
        html_inspector_core::validate_events(src, rules, Config::default()).unwrap()
    }

    #[test]
    fn td_role_is_disallowed_under_table_with_no_role() {
        let report = run(
            InputFormat::Html,
            "<table><tr><td role=\"presentation\"></td></tr></table>",
        );
        assert!(report
            .messages
            .iter()
            .any(|m| m.code == "html.td.role.disallowed_in_role_table"));
    }

    #[test]
    fn td_role_is_allowed_under_table_with_non_table_role() {
        let report = run(
            InputFormat::Html,
            "<table role=\"presentation\"><tr><td role=\"presentation\"></td></tr></table>",
        );
        assert!(!report
            .messages
            .iter()
            .any(|m| m.code == "html.td.role.disallowed_in_role_table"));
    }

    #[test]
    fn table_role_value_is_compared_case_insensitively() {
        let report = run(
            InputFormat::Html,
            "<table role=\"GRID\"><tr><td role=\"presentation\"></td></tr></table>",
        );
        assert!(report
            .messages
            .iter()
            .any(|m| m.code == "html.td.role.disallowed_in_role_table"));
    }
}
