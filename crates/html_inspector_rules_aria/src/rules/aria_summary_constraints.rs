use std::borrow::Cow;

use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct AriaSummaryConstraints {
    details_stack: Vec<DetailsState>,
}

#[derive(Clone, Copy, Debug)]
struct DetailsState {
    saw_summary: bool,
}

impl Rule for AriaSummaryConstraints {
    fn id(&self) -> &'static str {
        "aria.summary.constraints"
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
        // Match vnu.jar behavior: schema/Assertions checks are skipped inside <template> contents.
        if ctx.has_ancestor("template") {
            return;
        }

        match event {
            ParseEvent::StartTag {
                name,
                attrs,
                span,
                self_closing,
            } => {
                if ctx.name_is(name, "details") {
                    let should_push = match ctx.format {
                        html_inspector_core::InputFormat::Html => true,
                        html_inspector_core::InputFormat::Xhtml => !*self_closing,
                    };
                    if should_push {
                        self.details_stack.push(DetailsState { saw_summary: false });
                    }
                    return;
                }

                if !ctx.name_is(name, "summary") {
                    return;
                }

                if !ctx
                    .current_parent()
                    .is_some_and(|p| ctx.name_is(p, "details"))
                {
                    return;
                }

                let Some(state) = self.details_stack.last_mut() else {
                    return;
                };
                if state.saw_summary {
                    return;
                }
                state.saw_summary = true;

                // Match vnu.jar ordering: schema-required-attribute style errors come before the
                // summary-specific "must not be used" diagnostics.
                let mut has_role = false;
                let mut has_aria_expanded = false;
                let mut has_aria_pressed = false;
                let mut has_aria_selected = false;
                for attr in attrs {
                    let name = normalize_attr_name(ctx, &attr.name);
                    match name.as_ref() {
                        "role" => has_role = true,
                        "aria-expanded" => has_aria_expanded = true,
                        "aria-pressed" => has_aria_pressed = true,
                        "aria-selected" => has_aria_selected = true,
                        _ => {}
                    }
                }
                if !has_role {
                    if has_aria_expanded {
                        out.push(Message::new(
                            "aria.summary.details_summary.missing_role_for_aria_expanded",
                            Severity::Error,
                            Category::Aria,
                            "Element “summary” is missing one or more of the following attributes: “aria-checked”, “aria-level”, “role”.",
                            *span,
                        ));
                    } else if has_aria_pressed {
                        out.push(Message::new(
                            "aria.summary.details_summary.missing_role_for_aria_pressed",
                            Severity::Error,
                            Category::Aria,
                            "Element “summary” is missing required attribute “role”.",
                            *span,
                        ));
                    } else if has_aria_selected {
                        out.push(Message::new(
                            "aria.summary.details_summary.missing_role_for_aria_selected",
                            Severity::Error,
                            Category::Aria,
                            "Element “summary” is missing one or more of the following attributes: “aria-checked”, “role”.",
                            *span,
                        ));
                    }
                }

                for attr in attrs {
                    let name = normalize_attr_name(ctx, &attr.name);
                    let name = name.as_ref();
                    if name == "role"
                        || (name.starts_with("aria-")
                            && !is_aria_global_attribute(name)
                            && name != "aria-haspopup"
                            && name != "aria-disabled")
                    {
                        out.push(Message::new(
                            format!(
                                "aria.summary.details_summary.disallowed_attribute.{}",
                                name.replace('-', "_")
                            ),
                            Severity::Error,
                            Category::Aria,
                            format!(
                                "The “{name}” attribute must not be used on any “summary” element that is a summary for its parent “details” element."
                            ),
                            *span,
                        ));
                    }
                }
            }
            ParseEvent::EndTag { name, .. } => {
                if ctx.name_is(name, "details") {
                    self.details_stack.pop();
                }
            }
            _ => {}
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, _out: &mut dyn MessageSink) {
        self.details_stack.clear();
    }
}

fn normalize_attr_name<'a>(ctx: &ValidationContext, name: &'a str) -> Cow<'a, str> {
    match ctx.format {
        html_inspector_core::InputFormat::Html => {
            html_inspector_core::ascii_lowercase_cow_if_needed(name)
        }
        html_inspector_core::InputFormat::Xhtml => Cow::Borrowed(name),
    }
}

fn is_aria_global_attribute(name_lc: &str) -> bool {
    const ARIA_GLOBAL_ATTRIBUTES: [&str; 22] = [
        "aria-atomic",
        "aria-braillelabel",
        "aria-brailleroledescription",
        "aria-busy",
        "aria-controls",
        "aria-current",
        "aria-describedby",
        "aria-details",
        "aria-disabled",
        "aria-errormessage",
        "aria-flowto",
        "aria-haspopup",
        "aria-hidden",
        "aria-invalid",
        "aria-keyshortcuts",
        "aria-label",
        "aria-labelledby",
        "aria-live",
        "aria-owns",
        "aria-relevant",
        "aria-roledescription",
        "aria-description",
    ];
    ARIA_GLOBAL_ATTRIBUTES.contains(&name_lc)
}
