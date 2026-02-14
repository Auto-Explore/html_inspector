use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct SelectConstraints {
    stack: Vec<SelectState>,
}

#[derive(Clone, Copy, Debug)]
struct SelectState {
    multiple: bool,
    required: bool,
    size_gt_one: bool,
    selected_count: u32,
    option_count: u32,
    first_option_placeholder_ok: bool,
    in_first_option: bool,
    first_option_value_empty: bool,
    first_option_has_text: bool,
}

impl Rule for SelectConstraints {
    fn id(&self) -> &'static str {
        "html.select.constraints"
    }

    fn interest(&self) -> Interest {
        Interest::START_TAG | Interest::END_TAG | Interest::TEXT
    }

    fn on_event(
        &mut self,
        event: &ParseEvent,
        ctx: &mut ValidationContext,
        out: &mut dyn MessageSink,
    ) {
        match event {
            ParseEvent::StartTag {
                name, attrs, span, ..
            } => {
                if is(ctx, name, "select") {
                    let multiple = attrs.iter().any(|a| match ctx.format {
                        html_inspector::InputFormat::Html => {
                            a.name.eq_ignore_ascii_case("multiple")
                        }
                        html_inspector::InputFormat::Xhtml => a.name == "multiple",
                    });
                    let required = attrs.iter().any(|a| match ctx.format {
                        html_inspector::InputFormat::Html => {
                            a.name.eq_ignore_ascii_case("required")
                        }
                        html_inspector::InputFormat::Xhtml => a.name == "required",
                    });

                    let size_value = attrs
                        .iter()
                        .find(|a| match ctx.format {
                            html_inspector::InputFormat::Html => {
                                a.name.eq_ignore_ascii_case("size")
                            }
                            html_inspector::InputFormat::Xhtml => a.name == "size",
                        })
                        .and_then(|a| a.value.as_deref());
                    let mut size_gt_one = false;
                    if let Some(raw) = size_value {
                        let parsed = raw.trim().parse::<i32>().ok();
                        if parsed.is_some_and(|n| n == 0) {
                            out.push(Message::new(
                                "html.select.size.nonzero",
                                Severity::Error,
                                Category::Html,
                                format!(
                                    "Bad value “{raw}” for attribute “size” on element “select”."
                                ),
                                *span,
                            ));
                        } else if parsed.is_some_and(|n| n > 1) {
                            size_gt_one = true;
                        }
                    }

                    let autocomplete = attrs
                        .iter()
                        .find(|a| match ctx.format {
                            html_inspector::InputFormat::Html => {
                                a.name.eq_ignore_ascii_case("autocomplete")
                            }
                            html_inspector::InputFormat::Xhtml => a.name == "autocomplete",
                        })
                        .and_then(|a| a.value.as_deref());
                    if let Some(v) = autocomplete
                        && v.split_ascii_whitespace()
                            .any(|t| t.eq_ignore_ascii_case("webauthn"))
                    {
                        out.push(Message::new(
                                "html.select.autocomplete.webauthn.disallowed",
                                Severity::Error,
                                Category::Html,
                                "The value of the “autocomplete” attribute for the “select” element must not contain “webauthn”.",
                                *span,
                            ));
                    }

                    self.stack.push(SelectState {
                        multiple,
                        required,
                        size_gt_one,
                        selected_count: 0,
                        option_count: 0,
                        first_option_placeholder_ok: true,
                        in_first_option: false,
                        first_option_value_empty: false,
                        first_option_has_text: false,
                    });
                    return;
                }

                if is(ctx, name, "option") {
                    let Some(state) = self.stack.last_mut() else {
                        return;
                    };
                    state.option_count += 1;
                    if state.option_count == 1 {
                        state.in_first_option = true;
                        state.first_option_has_text = false;
                        state.first_option_value_empty = attrs
                            .iter()
                            .find(|a| match ctx.format {
                                html_inspector::InputFormat::Html => {
                                    a.name.eq_ignore_ascii_case("value")
                                }
                                html_inspector::InputFormat::Xhtml => a.name == "value",
                            })
                            .and_then(|a| a.value.as_deref())
                            .is_some_and(|v| v.is_empty());
                    }

                    let selected = attrs.iter().any(|a| match ctx.format {
                        html_inspector::InputFormat::Html => {
                            a.name.eq_ignore_ascii_case("selected")
                        }
                        html_inspector::InputFormat::Xhtml => a.name == "selected",
                    });
                    if selected {
                        state.selected_count += 1;
                        if !state.multiple && state.selected_count > 1 {
                            out.push(Message::new(
                                "html.select.selected.multiple_without_multiple",
                                Severity::Error,
                                Category::Html,
                                "The “select” element cannot have more than one selected “option” descendant unless the “multiple” attribute is specified.",
                                *span,
                            ));
                        }
                    }
                }
            }
            ParseEvent::Text { text, .. } => {
                let Some(state) = self.stack.last_mut() else {
                    return;
                };
                if state.in_first_option && text.chars().any(|c| !c.is_whitespace()) {
                    state.first_option_has_text = true;
                }
            }
            ParseEvent::EndTag { name, span } => {
                if is(ctx, name, "option") {
                    if let Some(state) = self.stack.last_mut()
                        && state.in_first_option
                    {
                        state.first_option_placeholder_ok =
                            state.first_option_value_empty || !state.first_option_has_text;
                        state.in_first_option = false;
                    }
                    return;
                }

                if is(ctx, name, "select")
                    && !self.stack.is_empty()
                    && let Some(state) = self.stack.pop()
                    && state.required
                    && !state.multiple
                    && !state.size_gt_one
                {
                    if state.option_count == 0 {
                        out.push(Message::new(
                                    "html.select.required.must_have_option",
                                    Severity::Error,
                                    Category::Html,
                                    "A “select” element with a “required” attribute, and without a “multiple” attribute, and without a “size” attribute whose value is greater than “1”, must have a child “option” element.",
                                    *span,
                                ));
                    } else if !state.first_option_placeholder_ok {
                        out.push(Message::new(
                                    "html.select.required.first_option.placeholder",
                                    Severity::Error,
                                    Category::Html,
                                    "The first child “option” element of a “select” element with a “required” attribute, and without a “multiple” attribute, and without a “size” attribute whose value is greater than “1”, must have either an empty “value” attribute, or must have no text content. Consider either adding a placeholder option label, or adding a “size” attribute with a value equal to the number of “option” elements.",
                                    *span,
                                ));
                    }
                }
            }
            _ => {}
        }
    }
}

fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    match ctx.format {
        html_inspector::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector::InputFormat::Xhtml => actual == expected,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use html_inspector::{Config, InputFormat, RuleSet, ValidationContext};
    use html_inspector_html::HtmlEventSource;

    fn validate(html: &str) -> html_inspector::Report {
        let src = HtmlEventSource::from_str("t", InputFormat::Html, html).unwrap();
        html_inspector::validate_events(
            src,
            RuleSet::new().push(SelectConstraints::default()),
            Config::default(),
        )
        .unwrap()
    }

    #[test]
    fn size_zero_emits_error() {
        let report = validate(r#"<select size="0"></select>"#);
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.select.size.nonzero")
        );
    }

    #[test]
    fn autocomplete_webauthn_emits_error() {
        let report = validate(r#"<select autocomplete="section-x webauthn"></select>"#);
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.select.autocomplete.webauthn.disallowed")
        );
    }

    #[test]
    fn multiple_selected_without_multiple_emits_error() {
        let report =
            validate("<select><option selected>1</option><option selected>2</option></select>");
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.select.selected.multiple_without_multiple")
        );
    }

    #[test]
    fn required_select_without_options_emits_error() {
        let report = validate("<select required></select>");
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.select.required.must_have_option")
        );
    }

    #[test]
    fn required_select_first_option_placeholder_rules_apply() {
        let report = validate(r#"<select required><option value="x">x</option></select>"#);
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.select.required.first_option.placeholder")
        );
    }

    #[test]
    fn xhtml_attribute_matching_is_case_sensitive_and_exercises_xhtml_paths() {
        let src = HtmlEventSource::from_str(
            "t",
            InputFormat::Xhtml,
            r#"<select required><option value="">x</option><option selected="selected"/><option selected="selected"/></select>"#,
        )
        .unwrap();
        let report = html_inspector::validate_events(
            src,
            RuleSet::new().push(SelectConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.select.selected.multiple_without_multiple")
        );
    }

    #[test]
    fn rule_ignores_unhandled_events() {
        struct Sink(Vec<html_inspector::Message>);
        impl html_inspector::MessageSink for Sink {
            fn push(&mut self, msg: html_inspector::Message) {
                self.0.push(msg);
            }
        }
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink(Vec::new());
        let mut rule = SelectConstraints::default();
        rule.on_event(
            &ParseEvent::Comment {
                text: "x".to_string(),
                span: None,
            },
            &mut ctx,
            &mut sink,
        );
        assert!(sink.0.is_empty());
        html_inspector::MessageSink::push(
            &mut sink,
            html_inspector::Message::new(
                "test.dummy",
                html_inspector::Severity::Info,
                html_inspector::Category::Html,
                "x".to_string(),
                None,
            ),
        );
        assert_eq!(sink.0.len(), 1);
    }
}
