use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct DlDivGroupConstraints {
    stack: Vec<DivGroupState>,
}

#[derive(Clone, Copy, Debug)]
struct DivGroupState {
    has_dt: bool,
    has_dd: bool,
    seen_dd: bool,
    span: Option<html_inspector::Span>,
}

impl Rule for DlDivGroupConstraints {
    fn id(&self) -> &'static str {
        "html.dl.div_group_constraints"
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
                // Match vnu.jar behavior: schema/Assertions checks are skipped inside <template> contents.
                if ctx.has_ancestor("template") {
                    return;
                }
                if ctx.name_is(name, "div")
                    && ctx.current_parent().is_some_and(|p| ctx.name_is(p, "dl"))
                {
                    if let Some(role) = ctx.attr_value(attrs, "role") {
                        let role = role.trim();
                        if !role.is_empty() && role != "presentation" && role != "none" {
                            out.push(Message::new(
                                "html.dl.div.role.disallowed",
                                Severity::Error,
                                Category::Html,
                                "A “div” child of a “dl” element must not have any “role” value other than “presentation” or “none”.",
                                *span,
                            ));
                        }
                    }
                    if !*self_closing {
                        self.stack.push(DivGroupState {
                            has_dt: false,
                            has_dd: false,
                            seen_dd: false,
                            span: *span,
                        });
                    }
                    return;
                }

                let Some(group) = self.stack.last_mut() else {
                    return;
                };
                if ctx.has_ancestor("template") || dl_ancestor_count(ctx) != 1 {
                    return;
                }

                if ctx.name_is(name, "div") && current_div_parent_is_dl(ctx) {
                    out.push(Message::new(
                        "html.dl.div.nested_div_disallowed",
                        Severity::Error,
                        Category::Html,
                        "Element “div” not allowed as child of “div” in this context.",
                        *span,
                    ));
                    return;
                }

                if ctx.name_is(name, "dt") {
                    if group.seen_dd {
                        out.push(Message::new(
                            "html.dl.div.dt.after_dd",
                            Severity::Error,
                            Category::Html,
                            "Element “dt” not allowed as child of “div” in this context.",
                            *span,
                        ));
                    }
                    group.has_dt = true;
                    return;
                }

                if ctx.name_is(name, "dd") {
                    group.has_dd = true;
                    group.seen_dd = true;
                }
            }
            ParseEvent::EndTag { name, span } => {
                // Match vnu.jar behavior: schema/Assertions checks are skipped inside <template> contents.
                if ctx.has_ancestor("template") {
                    return;
                }
                if ctx.name_is(name, "div") {
                    if !current_div_parent_is_dl(ctx) {
                        return;
                    }
                    if let Some(group) = self.stack.pop() {
                        // If we didn't see a dt or dd in this group, report a required-child error.
                        if !group.has_dd {
                            out.push(Message::new(
                                "html.dl.div.missing_dd",
                                Severity::Error,
                                Category::Html,
                                "Element “div” is missing a required instance of child element “dd”.",
                                span.or(group.span),
                            ));
                        } else if !group.has_dt {
                            out.push(Message::new(
                                "html.dl.div.missing_dt",
                                Severity::Error,
                                Category::Html,
                                "Element “div” is missing a required instance of child element “dt”.",
                                span.or(group.span),
                            ));
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, _out: &mut dyn MessageSink) {
        self.stack.clear();
    }
}

fn current_div_parent_is_dl(ctx: &ValidationContext) -> bool {
    let open = ctx.open_elements();
    let Some(pos) = open.iter().rposition(|n| ctx.name_is(n.as_str(), "div")) else {
        return false;
    };
    if pos == 0 {
        return false;
    }
    ctx.name_is(open[pos - 1].as_str(), "dl")
}

fn dl_ancestor_count(ctx: &ValidationContext) -> usize {
    ctx.open_elements()
        .iter()
        .filter(|n| ctx.name_is(n.as_str(), "dl"))
        .count()
}
