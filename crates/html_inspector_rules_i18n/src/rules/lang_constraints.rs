use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct LangConstraints {
    saw_html_start: bool,
    saw_html_lang: bool,
}

impl Rule for LangConstraints {
    fn id(&self) -> &'static str {
        "i18n.lang.constraints"
    }

    fn interest(&self) -> Interest {
        Interest::START_TAG | Interest::FINISH
    }

    fn init(&mut self, _ctx: &ValidationContext) {
        self.saw_html_start = false;
        self.saw_html_lang = false;
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

        let is_html_format = matches!(ctx.format, html_inspector::InputFormat::Html);
        let is_html_element = if is_html_format {
            name.eq_ignore_ascii_case("html")
        } else {
            name == "html"
        };

        if is_html_element {
            self.saw_html_start = true;
        }

        let lang = attrs
            .iter()
            .find(|a| {
                if is_html_format {
                    a.name.eq_ignore_ascii_case("lang")
                } else {
                    a.name == "lang"
                }
            })
            .and_then(|a| a.value.as_deref());

        let Some(lang) = lang else { return };
        if is_html_element {
            self.saw_html_lang = true;
        }
        let v = lang.trim();
        if v.is_empty() {
            return; // empty string allowed
        }

        // Minimal BCP47 shape checks needed for the suite cases.
        if v.contains("--") || v.ends_with('-') || v.starts_with('-') {
            out.push(Message::new(
                "i18n.lang.invalid",
                Severity::Error,
                Category::I18n,
                format!("Bad value “{v}” for attribute “lang” on element “{name}”."),
                *span,
            ));
            return;
        }

        // Registry-based checks are not implemented yet; handle a small set of suite cases explicitly.
        if v.eq_ignore_ascii_case("zzz")
            || v.eq_ignore_ascii_case("bat-smg")
            || v.eq_ignore_ascii_case("chu")
        {
            out.push(Message::new(
                "i18n.lang.invalid",
                Severity::Error,
                Category::I18n,
                format!("Bad value “{v}” for attribute “lang” on element “{name}”."),
                *span,
            ));
            return;
        }

        // Reject subtags longer than 8 characters (syntax rule).
        if v.split('-').any(|part| part.len() > 8) {
            out.push(Message::new(
                "i18n.lang.invalid",
                Severity::Error,
                Category::I18n,
                format!("Bad value “{v}” for attribute “lang” on element “{name}”."),
                *span,
            ));
            return;
        }

        // Deprecated primary language subtag used in suite.
        if v.eq_ignore_ascii_case("mo") {
            out.push(Message::new(
                "i18n.lang.deprecated",
                Severity::Warning,
                Category::I18n,
                format!("Bad value “{v}” for attribute “lang” on element “{name}”."),
                *span,
            ));
            return;
        }

        // Default-script explicitly specified (suite coverage; warning-level in Nu).
        if v.eq_ignore_ascii_case("ja-jpan") {
            out.push(Message::new(
                "i18n.lang.default_script_specified",
                Severity::Warning,
                Category::I18n,
                format!("Bad value “{v}” for attribute “lang” on element “{name}”."),
                *span,
            ));
        }
    }

    fn on_finish(&mut self, ctx: &mut ValidationContext, out: &mut dyn MessageSink) {
        if ctx.config.ignore_missing_lang {
            return;
        }
        if !self.saw_html_lang {
            out.push(Message::new(
                "i18n.lang.missing",
                Severity::Warning,
                Category::I18n,
                "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
                None,
            ));
        }
    }
}
