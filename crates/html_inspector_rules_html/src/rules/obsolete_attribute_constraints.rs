use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct ObsoleteAttributeConstraints;

impl Rule for ObsoleteAttributeConstraints {
    fn id(&self) -> &'static str {
        "html.attributes.obsolete"
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
        let ParseEvent::StartTag { attrs, span, .. } = event else {
            return;
        };

        for attr in attrs {
            if ctx.name_is(&attr.name, "aria-dropeffect") {
                out.push(Message::new(
                    "html.attr.aria_dropeffect.deprecated",
                    Severity::Warning,
                    Category::Html,
                    "The “aria-dropeffect” attribute is deprecated and should not be used. Support for it is poor and is unlikely to improve.",
                    *span,
                ));
            } else if ctx.name_is(&attr.name, "aria-grabbed") {
                out.push(Message::new(
                    "html.attr.aria_grabbed.deprecated",
                    Severity::Warning,
                    Category::Html,
                    "The “aria-grabbed” attribute is deprecated and should not be used. Support for it is poor and is unlikely to improve.",
                    *span,
                ));
            }
        }

        if has_attr(ctx, attrs, "contextmenu") {
            out.push(Message::new(
                "html.attr.contextmenu.obsolete",
                Severity::Error,
                Category::Html,
                "The “contextmenu” attribute is obsolete. Use script to handle “contextmenu” event instead.",
                *span,
            ));
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    use html_inspector_core::{Config, InputFormat, RuleSet};
    use html_inspector_html::HtmlEventSource;

    #[test]
    fn aria_dropeffect_and_grabbed_warn() {
        let src = HtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            r#"<div aria-dropeffect="copy" aria-grabbed="true"></div>"#,
        )
        .unwrap();
        let report = html_inspector_core::validate_events(
            src,
            RuleSet::new().push(ObsoleteAttributeConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(report
            .messages
            .iter()
            .any(|m| m.code == "html.attr.aria_dropeffect.deprecated"));
        assert!(report
            .messages
            .iter()
            .any(|m| m.code == "html.attr.aria_grabbed.deprecated"));
    }
}
