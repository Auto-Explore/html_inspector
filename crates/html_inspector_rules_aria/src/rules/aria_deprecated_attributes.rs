use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct AriaDeprecatedAttributes;

impl Rule for AriaDeprecatedAttributes {
    fn id(&self) -> &'static str {
        "aria.deprecated_attributes"
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
        let ParseEvent::StartTag { attrs, span, .. } = event else {
            return;
        };

        if attrs.iter().any(|a| match ctx.format {
            html_inspector_core::InputFormat::Html => {
                a.name.eq_ignore_ascii_case("aria-dropeffect")
            }
            html_inspector_core::InputFormat::Xhtml => a.name == "aria-dropeffect",
        }) {
            out.push(Message::new(
                "aria.attr.aria_dropeffect.deprecated",
                Severity::Warning,
                Category::Aria,
                "The “aria-dropeffect” attribute is deprecated and should not be used. Support for it is poor and is unlikely to improve.",
                *span,
            ));
        }

        if attrs.iter().any(|a| match ctx.format {
            html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case("aria-grabbed"),
            html_inspector_core::InputFormat::Xhtml => a.name == "aria-grabbed",
        }) {
            out.push(Message::new(
                "aria.attr.aria_grabbed.deprecated",
                Severity::Warning,
                Category::Aria,
                "The “aria-grabbed” attribute is deprecated and should not be used. Support for it is poor and is unlikely to improve.",
                *span,
            ));
        }
    }
}
