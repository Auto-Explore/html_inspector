use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct OlStartConstraints;

impl Rule for OlStartConstraints {
    fn id(&self) -> &'static str {
        "html.ol.start_constraints"
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
        if !is(ctx, name, "ol") {
            return;
        }

        let Some(start) = attr_value(ctx, attrs, "start") else {
            return;
        };

        if !is_valid_integer(start) {
            out.push(Message::new(
                "html.ol.start.invalid",
                Severity::Error,
                Category::Html,
                format!("Bad value “{start}” for attribute “start” on element “ol”."),
                *span,
            ));
        }
    }
}

fn is_valid_integer(v: &str) -> bool {
    let bytes = v.as_bytes();
    match bytes {
        [] => false,
        [b'-', rest @ ..] => !rest.is_empty() && rest.iter().all(|b| b.is_ascii_digit()),
        _ => bytes.iter().all(|b| b.is_ascii_digit()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use html_inspector_core::{Config, InputFormat, RuleSet};
    use html_inspector_html::HtmlEventSource;

    fn validate(format: InputFormat, html: &str) -> html_inspector_core::Report {
        let src = HtmlEventSource::from_str("t", format, html).unwrap();
        html_inspector_core::validate_events(
            src,
            RuleSet::new().push(OlStartConstraints::default()),
            Config::default(),
        )
        .unwrap()
    }

    #[test]
    fn accepts_positive_and_negative_integers() {
        for html in [
            r#"<ol start="1"></ol>"#,
            r#"<ol start="-1"></ol>"#,
            r#"<ol start="001"></ol>"#,
        ] {
            let report = validate(InputFormat::Html, html);
            assert!(
                !report
                    .messages
                    .iter()
                    .any(|m| m.code == "html.ol.start.invalid")
            );
        }
    }

    #[test]
    fn rejects_empty_and_malformed_values() {
        for html in [
            r#"<ol start=""></ol>"#,
            r#"<ol start></ol>"#,
            r#"<ol start="+1"></ol>"#,
            r#"<ol start="-"></ol>"#,
            r#"<ol start="--1"></ol>"#,
            r#"<ol start="1a"></ol>"#,
            r#"<ol start="١"></ol>"#,
        ] {
            let report = validate(InputFormat::Html, html);
            assert!(
                report
                    .messages
                    .iter()
                    .any(|m| m.code == "html.ol.start.invalid")
            );
        }
    }

    #[test]
    fn xhtml_matching_is_case_sensitive_for_tag_and_attribute_names() {
        let report = validate(InputFormat::Xhtml, r#"<ol start="+1"></ol>"#);
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.ol.start.invalid")
        );

        let report = validate(InputFormat::Xhtml, r#"<OL start="+1"></OL>"#);
        assert!(
            !report
                .messages
                .iter()
                .any(|m| m.code == "html.ol.start.invalid")
        );

        let report = validate(InputFormat::Xhtml, r#"<ol Start="+1"></ol>"#);
        assert!(
            !report
                .messages
                .iter()
                .any(|m| m.code == "html.ol.start.invalid")
        );
    }
}

fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    ctx.name_is(actual, expected)
}

fn attr_value<'a>(
    ctx: &ValidationContext,
    attrs: &'a [html_inspector_core::Attribute],
    needle: &str,
) -> Option<&'a str> {
    attrs
        .iter()
        .find(|a| ctx.name_is(&a.name, needle))
        .map(|a| a.value.as_deref().unwrap_or(""))
}
