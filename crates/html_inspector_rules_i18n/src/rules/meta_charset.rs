use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct MetaCharsetUtf8 {
    seen_charset_meta: bool,
}

impl Rule for MetaCharsetUtf8 {
    fn id(&self) -> &'static str {
        "i18n.meta.charset_utf8"
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
        if !ctx.name_is(name, "meta") {
            return;
        }

        let charset = ctx
            .attr_value(attrs, "charset")
            .map(str::trim)
            .filter(|s| !s.is_empty());

        let Some(charset) = charset else { return };

        // Nu checks this regardless of charset value.
        if self.seen_charset_meta {
            out.push(Message::new(
                "i18n.meta.charset.multiple",
                Severity::Error,
                Category::I18n,
                "A document must not include more than one “meta” element with a “charset” attribute.",
                *span,
            ));
            return;
        }
        self.seen_charset_meta = true;

        if !charset.eq_ignore_ascii_case("utf-8") && !charset.eq_ignore_ascii_case("utf8") {
            let severity = match ctx.config.severity_profile {
                // Many real-world documents (and WPT fixtures) intentionally exercise legacy encodings.
                // In risk mode, treat this as a warning rather than a hard error.
                html_inspector::SeverityProfile::Risk => Severity::Warning,
                html_inspector::SeverityProfile::Conformance => Severity::Error,
            };
            out.push(Message::new(
                "i18n.meta.charset.mismatch",
                severity,
                Category::I18n,
                format!(
                    "Internal encoding declaration “{charset}” disagrees with the actual encoding of the document (“utf-8”)."
                ),
                *span,
            ));
        }
    }
}
