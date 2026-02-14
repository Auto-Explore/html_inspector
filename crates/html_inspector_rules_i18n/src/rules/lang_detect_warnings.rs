use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, Span, ValidationContext,
    starts_with_ascii_ci,
};

#[derive(Default)]
pub struct LangDetectWarnings {
    html_lang: Option<String>,
    html_dir: Option<String>,
    html_span: Option<Span>,
    hebrew: u32,
    cjk: u32,
    arabic: u32,
}

impl Rule for LangDetectWarnings {
    fn id(&self) -> &'static str {
        "i18n.lang.detect_warnings"
    }

    fn max_severity(&self) -> Severity {
        Severity::Warning
    }

    fn interest(&self) -> Interest {
        Interest::START_TAG | Interest::TEXT
    }

    fn on_event(
        &mut self,
        event: &ParseEvent,
        ctx: &mut ValidationContext,
        _out: &mut dyn MessageSink,
    ) {
        match event {
            ParseEvent::StartTag {
                name, attrs, span, ..
            } => {
                if !ctx.name_is(name, "html") {
                    return;
                }
                self.html_span = *span;
                self.html_lang = ctx.attr_value(attrs, "lang").map(str::to_owned);
                self.html_dir = ctx.attr_value(attrs, "dir").map(str::to_owned);
            }
            ParseEvent::Text { text, .. } => {
                // Suite behavior: ignore <pre> content when guessing the document language.
                if ctx.has_ancestor("pre") {
                    return;
                }
                for ch in text.chars() {
                    if is_hebrew(ch) {
                        self.hebrew += 1;
                    } else if is_cjk(ch) {
                        self.cjk += 1;
                    } else if is_arabic(ch) {
                        self.arabic += 1;
                    }
                }
            }
            _ => {}
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, out: &mut dyn MessageSink) {
        let detected = if self.arabic >= 20 && self.arabic > self.hebrew && self.arabic > self.cjk {
            Some("ar")
        } else if self.hebrew >= 20 && self.hebrew > self.cjk {
            Some("he")
        } else if self.cjk >= 20 && self.cjk > self.hebrew {
            Some("zh-hant")
        } else {
            None
        };

        let Some(detected) = detected else {
            self.reset();
            return;
        };

        let span = self.html_span;

        if detected == "zh-hant"
            && let Some(lang) = self.html_lang.as_deref()
            && !starts_with_ascii_ci(lang, "zh-hant")
        {
            out.push(Message::new(
                        "i18n.lang.detect.zh_hant.mismatch",
                        Severity::Warning,
                        Category::I18n,
                        format!(
                            "This document appears to be written in Traditional Chinese but the “html” start tag has “lang=\"{lang}\"”. Consider using “lang=\"zh-hant\"” (or variant) instead."
                        ),
                        span,
                    ));
            self.reset();
            return;
        }

        if detected == "he" {
            if let Some(lang) = self.html_lang.as_deref()
                && !starts_with_ascii_ci(lang, "he")
            {
                out.push(Message::new(
                        "i18n.lang.detect.he.mismatch",
                        Severity::Warning,
                        Category::I18n,
                        format!(
                            "This document appears to be written in Hebrew but the “html” start tag has “lang=\"{lang}\"”. Consider using “lang=\"he\"” (or variant) instead."
                        ),
                        span,
                    ));
                self.reset();
                return;
            }

            if let Some(dir) = self.html_dir.as_deref()
                && dir.eq_ignore_ascii_case("ltr")
            {
                out.push(Message::new(
                        "i18n.lang.detect.he.dir_ltr",
                        Severity::Warning,
                        Category::I18n,
                        format!(
                            "This document appears to be written in Hebrew but the “html” start tag has “dir=\"{dir}\"”. Consider using “dir=\"rtl\"” instead."
                        ),
                        span,
                    ));
                self.reset();
                return;
            }
        }

        if detected == "ar" {
            if let Some(lang) = self.html_lang.as_deref()
                && !starts_with_ascii_ci(lang, "ar")
            {
                out.push(Message::new(
                        "i18n.lang.detect.ar.mismatch",
                        Severity::Warning,
                        Category::I18n,
                        format!(
                            "This document appears to be written in Arabic but the “html” start tag has “lang=\"{lang}\"”. Consider using “lang=\"ar\"” (or variant) instead."
                        ),
                        span,
                    ));
                self.reset();
                return;
            }

            if let Some(dir) = self.html_dir.as_deref()
                && dir.eq_ignore_ascii_case("ltr")
            {
                out.push(Message::new(
                        "i18n.lang.detect.ar.dir_ltr",
                        Severity::Warning,
                        Category::I18n,
                        format!(
                            "This document appears to be written in Arabic but the “html” start tag has “dir=\"{dir}\"”. Consider using “dir=\"rtl\"” instead."
                        ),
                        span,
                    ));
                self.reset();
                return;
            }
        }

        self.reset();
    }
}

impl LangDetectWarnings {
    fn reset(&mut self) {
        *self = Self::default();
    }
}

fn is_hebrew(ch: char) -> bool {
    matches!(ch as u32, 0x0590..=0x05FF)
}

fn is_cjk(ch: char) -> bool {
    matches!(ch as u32, 0x4E00..=0x9FFF)
}

fn is_arabic(ch: char) -> bool {
    matches!(
        ch as u32,
        0x0600..=0x06FF
            | 0x0750..=0x077F
            | 0x08A0..=0x08FF
            | 0xFB50..=0xFDFF
            | 0xFE70..=0xFEFF
    )
}
