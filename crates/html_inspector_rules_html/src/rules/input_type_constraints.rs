use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct InputTypeConstraints;

impl Rule for InputTypeConstraints {
    fn id(&self) -> &'static str {
        "html.input.type_constraints"
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
        if !ctx.name_is(name, "input") {
            return;
        }

        let type_value = ctx.attr_value(attrs, "type").unwrap_or("");

        let value_attr = attrs.iter().find(|a| ctx.name_is(&a.name, "value"));

        if type_value.eq_ignore_ascii_case("button") {
            let non_empty = value_attr
                .and_then(|a| a.value.as_deref())
                .is_some_and(|v| !v.is_empty());
            if !non_empty {
                out.push(Message::new(
                    "html.input.button.value.nonempty",
                    Severity::Error,
                    Category::Html,
                    "Element “input” with attribute “type” whose value is “button” must have non-empty attribute “value”.",
                    *span,
                ));
            }
        }

        if type_value.eq_ignore_ascii_case("image") {
            let has_alt = ctx.has_attr(attrs, "alt");
            if !has_alt {
                out.push(Message::new(
                    "html.input.image.alt.missing",
                    Severity::Error,
                    Category::Html,
                    "Element “input” is missing required attribute “alt”.",
                    *span,
                ));
            }
        }

        if type_value.eq_ignore_ascii_case("file") {
            if value_attr.is_some() {
                out.push(Message::new(
                    "html.input.file.value.disallowed",
                    Severity::Error,
                    Category::Html,
                    "Attribute “value” not allowed on element “input” at this point.",
                    *span,
                ));
            }

            let has_alt = ctx.has_attr(attrs, "alt");
            if has_alt {
                out.push(Message::new(
                    "html.input.file.alt.disallowed",
                    Severity::Error,
                    Category::Html,
                    "Attribute “alt” not allowed on element “input” at this point.",
                    *span,
                ));
            }

            let has_dirname = ctx.has_attr(attrs, "dirname");
            if has_dirname {
                out.push(Message::new(
                    "html.input.file.dirname.disallowed",
                    Severity::Error,
                    Category::Html,
                    "Attribute “dirname” not allowed on element “input” at this point.",
                    *span,
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use html_inspector::{Config, InputFormat, RuleSet};
    use html_inspector_html::HtmlEventSource;

    fn codes(html: &str) -> Vec<String> {
        let src = HtmlEventSource::from_str("t", InputFormat::Html, html).unwrap();
        let report = html_inspector::validate_events(
            src,
            RuleSet::new().push(InputTypeConstraints::default()),
            Config::default(),
        )
        .unwrap();
        report.messages.into_iter().map(|m| m.code).collect()
    }

    #[test]
    fn input_type_button_requires_non_empty_value() {
        assert!(
            codes(r#"<input type="button">"#)
                .iter()
                .any(|c| c == "html.input.button.value.nonempty")
        );
        assert!(
            codes(r#"<input type="button" value="">"#)
                .iter()
                .any(|c| c == "html.input.button.value.nonempty")
        );
        assert!(
            !codes(r#"<input type="button" value="x">"#)
                .iter()
                .any(|c| c == "html.input.button.value.nonempty")
        );
    }
}
