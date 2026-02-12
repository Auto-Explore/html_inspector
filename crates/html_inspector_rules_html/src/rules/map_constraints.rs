use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

use super::shared::{attr_value, eq_name};

#[derive(Default)]
pub struct MapConstraints;

impl Rule for MapConstraints {
    fn id(&self) -> &'static str {
        "html.map.constraints"
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
        if !eq_name(ctx, name, "map") {
            return;
        }

        let id = attr_value(ctx, attrs, "id");
        let name_attr = attr_value(ctx, attrs, "name");

        if let Some((id, name_attr)) = id.zip(name_attr) {
            if id != name_attr {
                out.push(Message::new(
                    "html.map.id_name.mismatch",
                    Severity::Error,
                    Category::Html,
                    "The “id” attribute on a “map” element must have an the same value as the “name” attribute.",
                    *span,
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use html_inspector_core::{Config, InputFormat, RuleSet};
    use html_inspector_html::HtmlEventSource;

    use super::MapConstraints;

    fn validate(html: &str) -> Vec<String> {
        let src = HtmlEventSource::from_str("t", InputFormat::Html, html).unwrap();
        let report = html_inspector_core::validate_events(
            src,
            RuleSet::new().push(MapConstraints::default()),
            Config::default(),
        )
        .unwrap();
        report.messages.into_iter().map(|m| m.code).collect()
    }

    #[test]
    fn map_id_name_mismatch_is_reported() {
        assert_eq!(
            validate(r#"<map id="a" name="b"></map>"#),
            vec!["html.map.id_name.mismatch"]
        );
    }

    #[test]
    fn map_missing_id_or_name_is_not_reported() {
        assert!(validate(r#"<map id="a"></map>"#).is_empty());
        assert!(validate(r#"<map name="a"></map>"#).is_empty());
    }
}
