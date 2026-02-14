use html_inspector_core::{
    Attribute, Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity,
    ValidationContext,
};

#[derive(Default)]
pub struct OptionConstraints {
    stack: Vec<OptionState>,
}

#[derive(Clone, Debug)]
struct OptionState {
    has_label_attr: bool,
    has_value_attr: bool,
    in_datalist: bool,
    saw_non_whitespace_text: bool,
}

impl Rule for OptionConstraints {
    fn id(&self) -> &'static str {
        "html.option.constraints"
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
                name,
                attrs,
                self_closing,
                span,
            } => {
                if !is(ctx, name, "option") {
                    return;
                }

                if let Some(label) = attr_value(ctx, attrs, "label")
                    && label.is_empty() {
                        out.push(Message::new(
                            "html.option.label.empty",
                            Severity::Error,
                            Category::Html,
                            "Bad value “” for attribute “label” on element “option”.",
                            *span,
                        ));
                    }

                let has_label_attr = has_attr(ctx, attrs, "label");
                let has_value_attr = has_attr(ctx, attrs, "value");
                let in_datalist = ctx.current_parent().is_some_and(|p| is(ctx, p, "datalist"));
                let empty_ok = has_label_attr || (in_datalist && has_value_attr);

                // In HTML, <option/> isn't a thing, but treat it as empty content.
                if *self_closing && !empty_ok {
                    out.push(Message::new(
                        "html.option.empty_without_label",
                        Severity::Error,
                        Category::Html,
                        "Element “option” without attribute “label” must not be empty.",
                        *span,
                    ));
                    return;
                }

                self.stack.push(OptionState {
                    has_label_attr,
                    has_value_attr,
                    in_datalist,
                    saw_non_whitespace_text: false,
                });
            }
            ParseEvent::Text { text, .. } => {
                if let Some(state) = self.stack.last_mut()
                    && text.chars().any(|c| !c.is_whitespace()) {
                        state.saw_non_whitespace_text = true;
                    }
            }
            ParseEvent::EndTag { name, span } => {
                if !is(ctx, name, "option") {
                    return;
                }
                let Some(state) = self.stack.pop() else {
                    return;
                };
                let empty_ok = state.has_label_attr
                    || state.saw_non_whitespace_text
                    || (state.in_datalist && state.has_value_attr);
                if !empty_ok {
                    out.push(Message::new(
                        "html.option.empty_without_label",
                        Severity::Error,
                        Category::Html,
                        "Element “option” without attribute “label” must not be empty.",
                        *span,
                    ));
                }
            }
            _ => {}
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, _out: &mut dyn MessageSink) {
        self.stack.clear();
    }
}

fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    ctx.name_is(actual, expected)
}

fn has_attr(ctx: &ValidationContext, attrs: &[Attribute], needle: &str) -> bool {
    ctx.has_attr(attrs, needle)
}

fn attr_value<'a>(
    ctx: &ValidationContext,
    attrs: &'a [Attribute],
    needle: &str,
) -> Option<&'a str> {
    attrs
        .iter()
        .find(|a| ctx.name_is(&a.name, needle))
        .map(|a| a.value.as_deref().unwrap_or(""))
}

#[cfg(test)]
mod tests {
    use super::*;

    use html_inspector_core::{Config, InputFormat, RuleSet, ValidationContext};
    use html_inspector_html::{HtmlEventSource, SimpleHtmlEventSource};

    #[test]
    fn empty_label_value_emits_error() {
        let src = HtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            r#"<select><option label=""></option></select>"#,
        )
        .unwrap();
        let report = html_inspector_core::validate_events(
            src,
            RuleSet::new().push(OptionConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(report
            .messages
            .iter()
            .any(|m| m.code == "html.option.label.empty"));
    }

    #[test]
    fn empty_option_without_label_emits_error() {
        let src =
            HtmlEventSource::from_str("t", InputFormat::Html, "<select><option></option></select>")
                .unwrap();
        let report = html_inspector_core::validate_events(
            src,
            RuleSet::new().push(OptionConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(report
            .messages
            .iter()
            .any(|m| m.code == "html.option.empty_without_label"));
    }

    #[test]
    fn datalist_option_with_value_attr_is_allowed_to_be_empty() {
        let src = HtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<datalist><option value=\"\"></option></datalist>",
        )
        .unwrap();
        let report = html_inspector_core::validate_events(
            src,
            RuleSet::new().push(OptionConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(!report
            .messages
            .iter()
            .any(|m| m.code == "html.option.empty_without_label"));
    }

    #[test]
    fn option_with_nonempty_label_attr_is_allowed_to_be_empty() {
        let src = HtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            r#"<select><option label="x"></option></select>"#,
        )
        .unwrap();
        let report = html_inspector_core::validate_events(
            src,
            RuleSet::new().push(OptionConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(!report
            .messages
            .iter()
            .any(|m| m.code == "html.option.empty_without_label"));
    }

    #[test]
    fn self_closing_option_without_label_emits_error() {
        let src =
            SimpleHtmlEventSource::from_str("t", InputFormat::Html, "<select><option/></select>");
        let report = html_inspector_core::validate_events(
            src,
            RuleSet::new().push(OptionConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(report
            .messages
            .iter()
            .any(|m| m.code == "html.option.empty_without_label"));
    }

    #[test]
    fn option_with_only_whitespace_text_is_treated_as_empty() {
        let src = HtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<select><option>\n \t</option></select>",
        )
        .unwrap();
        let report = html_inspector_core::validate_events(
            src,
            RuleSet::new().push(OptionConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(report
            .messages
            .iter()
            .any(|m| m.code == "html.option.empty_without_label"));
    }

    #[test]
    fn html_minimized_label_attribute_is_treated_as_empty() {
        let src = SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<select><option label></option></select>",
        );
        let report = html_inspector_core::validate_events(
            src,
            RuleSet::new().push(OptionConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(report
            .messages
            .iter()
            .any(|m| m.code == "html.option.label.empty"));
    }

    #[test]
    fn xhtml_attribute_matching_uses_case_sensitive_lookups() {
        let src = HtmlEventSource::from_str(
            "t",
            InputFormat::Xhtml,
            r#"<select><option label="x">y</option></select>"#,
        )
        .unwrap();
        let report = html_inspector_core::validate_events(
            src,
            RuleSet::new().push(OptionConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(report.messages.is_empty());
    }

    #[test]
    fn rule_ignores_unhandled_events() {
        struct Sink(Vec<html_inspector_core::Message>);
        impl html_inspector_core::MessageSink for Sink {
            fn push(&mut self, msg: html_inspector_core::Message) {
                self.0.push(msg);
            }
        }
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink(Vec::new());
        let mut rule = OptionConstraints::default();
        rule.on_event(
            &ParseEvent::Comment {
                text: "x".to_string(),
                span: None,
            },
            &mut ctx,
            &mut sink,
        );
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
}
