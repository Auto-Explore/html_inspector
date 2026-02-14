use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct RdfaLiteConstraints;

impl Rule for RdfaLiteConstraints {
    fn id(&self) -> &'static str {
        "html.rdfa.lite_constraints"
    }

    fn max_severity(&self) -> Severity {
        Severity::Warning
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

        if ctx.format != html_inspector::InputFormat::Html {
            return;
        }

        for attr in attrs {
            let attr_name = attr.name.as_str();
            if matches!(attr_name, "datatype" | "about" | "inlist" | "rev") {
                out.push(Message::new(
                    "html.rdfa.lite.attr.disallowed",
                    Severity::Warning,
                    Category::Html,
                    format!(
                        "RDFa Core attribute “{attr_name}” is not allowed in HTML5 + RDFa 1.1 Lite documents. Consider checking against the HTML5 + RDFa 1.1 schema instead."
                    ),
                    *span,
                ));
                continue;
            }

            if attr_name == "content" && !ctx.name_is(name, "meta") {
                out.push(Message::new(
                    "html.rdfa.lite.attr.disallowed",
                    Severity::Warning,
                    Category::Html,
                    format!(
                        "RDFa Core attribute “{attr_name}” is not allowed on the “{name}” element in HTML5 + RDFa 1.1 Lite documents. Consider checking against the HTML5 + RDFa 1.1 schema instead."
                    ),
                    *span,
                ));
                continue;
            }

            if attr_name == "rel"
                && !(ctx.name_is(name, "a")
                    || ctx.name_is(name, "area")
                    || ctx.name_is(name, "link"))
            {
                out.push(Message::new(
                    "html.rdfa.lite.attr.disallowed",
                    Severity::Warning,
                    Category::Html,
                    format!(
                        "RDFa Core attribute “{attr_name}” is not allowed on the “{name}” element in HTML5 + RDFa 1.1 Lite documents. Consider checking against the HTML5 + RDFa 1.1 schema instead."
                    ),
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

    #[test]
    fn rdfa_lite_disallowed_attributes_warn() {
        let src = HtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<div datatype=\"x\" about=\"y\" inlist=\"z\" rev=\"q\"></div>",
        )
        .unwrap();
        let report = html_inspector::validate_events(
            src,
            RuleSet::new().push(RdfaLiteConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert_eq!(
            report
                .messages
                .iter()
                .filter(|m| m.code == "html.rdfa.lite.attr.disallowed")
                .count(),
            4
        );
    }

    #[test]
    fn rdfa_lite_content_only_allowed_on_meta() {
        let src =
            HtmlEventSource::from_str("t", InputFormat::Html, "<div content=\"x\"></div>").unwrap();
        let report = html_inspector::validate_events(
            src,
            RuleSet::new().push(RdfaLiteConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.rdfa.lite.attr.disallowed")
        );
    }

    #[test]
    fn rdfa_lite_rel_only_allowed_on_a_area_link() {
        let src = HtmlEventSource::from_str("t", InputFormat::Html, "<span rel=\"next\"></span>")
            .unwrap();
        let report = html_inspector::validate_events(
            src,
            RuleSet::new().push(RdfaLiteConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.rdfa.lite.attr.disallowed")
        );
    }
}
