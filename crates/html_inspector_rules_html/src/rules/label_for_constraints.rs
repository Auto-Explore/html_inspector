use rustc_hash::FxHashSet;

use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, Span, ValidationContext,
};

#[derive(Default)]
pub struct LabelForConstraints {
    label_stack: Vec<LabelState>,
    role_button_stack: Vec<String>,
    labelable_ids: FxHashSet<String>,
    label_for_refs: Vec<(String, Option<Span>)>,
    label_assoc_checks: Vec<LabelAssocCheck>,
}

#[derive(Clone, Debug)]
struct LabelState {
    for_value: Option<String>,
    has_aria_hidden: bool,
    aria_hidden_reported: bool,
    has_role: bool,
    role_reported: bool,
}

#[derive(Clone, Debug)]
struct LabelAssocCheck {
    for_value: String,
    has_role: bool,
    has_aria_label: bool,
    span: Option<Span>,
}

impl Rule for LabelForConstraints {
    fn id(&self) -> &'static str {
        "html.label.for_constraints"
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
                // Track role=button ancestors (suite coverage).
                let pushes = match ctx.format {
                    html_inspector_core::InputFormat::Html => {
                        !html_inspector_core::is_void_html_element(name)
                    }
                    html_inspector_core::InputFormat::Xhtml => !*self_closing,
                };
                if has_role_button(ctx, attrs) && pushes {
                    self.role_button_stack.push(normalize_name(ctx, name));
                }

                if is(ctx, name, "label") {
                    let for_value = attr_value(ctx, attrs, "for").map(|s| s.to_string());
                    if let Some(v) = for_value.as_ref()
                        && !v.is_empty()
                    {
                        self.label_for_refs.push((v.clone(), *span));
                    }
                    let has_aria_hidden = has_attr(ctx, attrs, "aria-hidden");
                    let has_role = has_attr(ctx, attrs, "role");
                    let has_aria_label = has_attr(ctx, attrs, "aria-label");
                    if let Some(v) = for_value.as_ref()
                        && !v.is_empty()
                        && (has_role || has_aria_label)
                    {
                        self.label_assoc_checks.push(LabelAssocCheck {
                            for_value: v.clone(),
                            has_role,
                            has_aria_label,
                            span: *span,
                        });
                    }
                    self.label_stack.push(LabelState {
                        for_value,
                        has_aria_hidden,
                        aria_hidden_reported: false,
                        has_role,
                        role_reported: false,
                    });
                    return;
                }

                // Collect IDs of non-hidden form controls for later "for" checks.
                if is_non_hidden_form_control(ctx, name, attrs)
                    && let Some(id) = attr_value(ctx, attrs, "id")
                    && !id.is_empty()
                {
                    self.labelable_ids.insert(id.to_string());
                }

                // role=button ancestors must not have input descendants (suite coverage).
                if !self.role_button_stack.is_empty() && is(ctx, name, "input") {
                    out.push(Message::new(
                        "html.role_button.descendant_input",
                        Severity::Error,
                        Category::Html,
                        "The element “input” must not appear as a descendant of an element with the attribute “role=button”.",
                        *span,
                    ));
                }

                // Descendant checks within labels.
                if !self.label_stack.is_empty() {
                    if is(ctx, name, "input")
                        && let Some(state) = self.label_stack.last()
                        && let Some(for_value) = state.for_value.as_deref()
                    {
                        let id = attr_value(ctx, attrs, "id");
                        let matches = id.is_some_and(|v| v == for_value);
                        if !matches {
                            out.push(Message::new(
                                        "html.label.for.descendant_input_id_mismatch",
                                        Severity::Error,
                                        Category::Html,
                                        "Any “input” descendant of a “label” element with a “for” attribute must have an ID value that matches that “for” attribute.",
                                        *span,
                                    ));
                        }
                    }

                    if is_labelable_descendant(ctx, name, attrs) {
                        // aria-hidden must not be used on a label that is an ancestor of a labelable element.
                        if let Some(state) = self.label_stack.last_mut() {
                            if state.has_aria_hidden && !state.aria_hidden_reported {
                                state.aria_hidden_reported = true;
                                out.push(Message::new(
                                    "html.label.aria_hidden.with_labelable_descendant",
                                    Severity::Error,
                                    Category::Html,
                                    "The “aria-hidden” attribute must not be used on any “label” element that is an ancestor of a labelable element.",
                                    *span,
                                ));
                            }
                            if state.has_role && !state.role_reported {
                                state.role_reported = true;
                                out.push(Message::new(
                                    "html.label.role.with_labelable_descendant",
                                    Severity::Error,
                                    Category::Html,
                                    "The “role” attribute must not be used on any “label” element that is an ancestor of a labelable element.",
                                    *span,
                                ));
                            }
                        }
                    }
                }
            }
            ParseEvent::EndTag { name, .. } => {
                if is(ctx, name, "label") && !self.label_stack.is_empty() {
                    self.label_stack.pop();
                }

                if let Some(top) = self.role_button_stack.last()
                    && names_match(ctx, top, name)
                {
                    self.role_button_stack.pop();
                }
            }
            _ => {}
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, out: &mut dyn MessageSink) {
        for (for_value, span) in self.label_for_refs.drain(..) {
            if for_value.is_empty() {
                continue;
            }
            if !self.labelable_ids.contains(&for_value) {
                out.push(Message::new(
                    "html.label.for.must_reference_non_hidden_control",
                    Severity::Error,
                    Category::Html,
                    "The value of the “for” attribute of the “label” element must be the ID of a non-hidden form control.",
                    span,
                ));
            }
        }

        for check in self.label_assoc_checks.drain(..) {
            if check.for_value.is_empty() {
                continue;
            }
            if !self.labelable_ids.contains(&check.for_value) {
                continue;
            }
            if check.has_aria_label {
                out.push(Message::new(
                    "html.label.aria_label.associated_with_labelable",
                    Severity::Error,
                    Category::Html,
                    "The “aria-label” attribute must not be used on any “label” element that is associated with a labelable element.",
                    check.span,
                ));
            }
            if check.has_role {
                out.push(Message::new(
                    "html.label.role.associated_with_labelable",
                    Severity::Error,
                    Category::Html,
                    "The “role” attribute must not be used on any “label” element that is associated with a labelable element.",
                    check.span,
                ));
            }
        }
        self.labelable_ids.clear();
        self.label_stack.clear();
        self.role_button_stack.clear();
    }
}

fn is_labelable_descendant(
    ctx: &ValidationContext,
    name: &str,
    attrs: &[html_inspector_core::Attribute],
) -> bool {
    if is(ctx, name, "input") {
        let t = attr_value(ctx, attrs, "type").unwrap_or("");
        return !t.eq_ignore_ascii_case("hidden");
    }
    is(ctx, name, "textarea")
        || is(ctx, name, "select")
        || is(ctx, name, "button")
        || is(ctx, name, "meter")
        || is(ctx, name, "output")
        || is(ctx, name, "progress")
}

fn is_non_hidden_form_control(
    ctx: &ValidationContext,
    name: &str,
    attrs: &[html_inspector_core::Attribute],
) -> bool {
    if is(ctx, name, "input") {
        let t = attr_value(ctx, attrs, "type").unwrap_or("");
        return !t.eq_ignore_ascii_case("hidden");
    }
    is(ctx, name, "textarea") || is(ctx, name, "select") || is(ctx, name, "button")
}

fn has_role_button(ctx: &ValidationContext, attrs: &[html_inspector_core::Attribute]) -> bool {
    let role = attr_value(ctx, attrs, "role").unwrap_or("");
    match ctx.format {
        html_inspector_core::InputFormat::Html => role.eq_ignore_ascii_case("button"),
        html_inspector_core::InputFormat::Xhtml => role == "button",
    }
}

fn normalize_name(ctx: &ValidationContext, name: &str) -> String {
    match ctx.format {
        html_inspector_core::InputFormat::Html => name.to_ascii_lowercase(),
        html_inspector_core::InputFormat::Xhtml => name.to_string(),
    }
}

fn names_match(ctx: &ValidationContext, a: &str, b: &str) -> bool {
    match ctx.format {
        html_inspector_core::InputFormat::Html => a.eq_ignore_ascii_case(b),
        html_inspector_core::InputFormat::Xhtml => a == b,
    }
}

fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    match ctx.format {
        html_inspector_core::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector_core::InputFormat::Xhtml => actual == expected,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use html_inspector_core::{Config, InputFormat, RuleSet, ValidationContext};
    use html_inspector_html::HtmlEventSource;

    fn validate(html: &str) -> html_inspector_core::Report {
        let src = HtmlEventSource::from_str("t", InputFormat::Html, html).unwrap();
        html_inspector_core::validate_events(
            src,
            RuleSet::new().push(LabelForConstraints::default()),
            Config::default(),
        )
        .unwrap()
    }

    fn validate_fmt(format: InputFormat, html: &str) -> html_inspector_core::Report {
        let src = HtmlEventSource::from_str("t", format, html).unwrap();
        html_inspector_core::validate_events(
            src,
            RuleSet::new().push(LabelForConstraints::default()),
            Config::default(),
        )
        .unwrap()
    }

    #[test]
    fn label_for_must_reference_existing_non_hidden_control() {
        let report = validate(r#"<label for="x"></label>"#);
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.label.for.must_reference_non_hidden_control")
        );
    }

    #[test]
    fn role_button_must_not_have_input_descendant() {
        let report = validate(r#"<div role="button"><input></div>"#);
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.role_button.descendant_input")
        );
    }

    #[test]
    fn label_for_descendant_input_id_mismatch_emits_error() {
        let report = validate(r#"<label for="x"><input id="y"></label>"#);
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.label.for.descendant_input_id_mismatch")
        );
    }

    #[test]
    fn aria_hidden_and_role_on_label_with_labelable_descendant_emit_errors() {
        let report = validate(r#"<label aria-hidden="true" role="button"><input id="x"></label>"#);
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.label.aria_hidden.with_labelable_descendant")
        );
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.label.role.with_labelable_descendant")
        );
    }

    #[test]
    fn label_association_disallows_aria_label_and_role() {
        let report =
            validate(r#"<label for="x" aria-label="a" role="button"></label><input id="x">"#);
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.label.aria_label.associated_with_labelable")
        );
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.label.role.associated_with_labelable")
        );
    }

    #[test]
    fn xhtml_role_button_stack_push_and_pop_use_case_sensitive_matching() {
        let report = validate_fmt(
            InputFormat::Xhtml,
            r#"<div role="button"><span></span></div>"#,
        );
        assert!(report.messages.is_empty());
    }

    #[test]
    fn finish_skips_empty_and_unmatched_association_checks() {
        struct Sink(Vec<html_inspector_core::Message>);
        impl html_inspector_core::MessageSink for Sink {
            fn push(&mut self, msg: html_inspector_core::Message) {
                self.0.push(msg);
            }
        }

        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink(Vec::new());
        let mut rule = LabelForConstraints::default();

        rule.label_for_refs.push(("".to_string(), None));
        rule.label_assoc_checks.push(LabelAssocCheck {
            for_value: "".to_string(),
            has_role: true,
            has_aria_label: true,
            span: None,
        });
        rule.label_assoc_checks.push(LabelAssocCheck {
            for_value: "missing".to_string(),
            has_role: true,
            has_aria_label: true,
            span: None,
        });

        rule.on_finish(&mut ctx, &mut sink);
        assert!(sink.0.is_empty());

        html_inspector_core::MessageSink::push(
            &mut sink,
            html_inspector_core::Message::new(
                "test.dummy",
                html_inspector_core::Severity::Info,
                html_inspector_core::Category::Html,
                "x".to_string(),
                None,
            ),
        );
        assert_eq!(sink.0.len(), 1);
    }

    #[test]
    fn rule_ignores_unhandled_events() {
        struct NoopSink;
        impl html_inspector_core::MessageSink for NoopSink {
            fn push(&mut self, _msg: html_inspector_core::Message) {}
        }
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = NoopSink;
        let mut rule = LabelForConstraints::default();
        rule.on_event(
            &ParseEvent::Comment {
                text: "x".to_string(),
                span: None,
            },
            &mut ctx,
            &mut sink,
        );

        html_inspector_core::MessageSink::push(
            &mut sink,
            html_inspector_core::Message::new(
                "test.dummy",
                html_inspector_core::Severity::Info,
                html_inspector_core::Category::Html,
                "x".to_string(),
                None,
            ),
        );
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

fn attr_value<'a>(
    ctx: &ValidationContext,
    attrs: &'a [html_inspector_core::Attribute],
    needle: &str,
) -> Option<&'a str> {
    attrs
        .iter()
        .find(|a| match ctx.format {
            html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case(needle),
            html_inspector_core::InputFormat::Xhtml => a.name == needle,
        })
        .and_then(|a| a.value.as_deref())
}
