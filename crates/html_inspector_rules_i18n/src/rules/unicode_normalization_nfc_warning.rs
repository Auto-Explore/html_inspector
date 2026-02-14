use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, Span, ValidationContext,
};
use unicode_normalization::{UnicodeNormalization, is_nfc};

#[derive(Default)]
pub struct UnicodeNormalizationNfcWarning {
    text_buf: String,
    text_run_span: Option<Span>,
    at_start_of_run: bool,
    already_complained_about_this_run: bool,
}

impl Rule for UnicodeNormalizationNfcWarning {
    fn id(&self) -> &'static str {
        "i18n.unicode.normalization_nfc_warning"
    }

    fn max_severity(&self) -> Severity {
        Severity::Warning
    }

    fn interest(&self) -> Interest {
        Interest::TEXT | Interest::START_TAG | Interest::END_TAG | Interest::PROCESSING_INSTRUCTION
    }

    fn init(&mut self, _ctx: &ValidationContext) {
        self.text_buf.clear();
        self.text_run_span = None;
        self.at_start_of_run = true;
        self.already_complained_about_this_run = false;
    }

    fn on_event(
        &mut self,
        event: &ParseEvent,
        _ctx: &mut ValidationContext,
        out: &mut dyn MessageSink,
    ) {
        match event {
            ParseEvent::Text { text, span } => self.on_text(text, *span, out),
            ParseEvent::StartTag {
                name, attrs, span, ..
            } => {
                self.flush_text_run(out);
                self.check_element_and_attrs(name, attrs, *span, out);
            }
            ParseEvent::EndTag { .. } => self.flush_text_run(out),
            ParseEvent::ProcessingInstruction {
                target, data, span, ..
            } => {
                self.flush_text_run(out);
                self.check_processing_instruction(target, data, *span, out);
            }
            _ => {}
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, out: &mut dyn MessageSink) {
        self.flush_text_run(out);
    }
}

impl UnicodeNormalizationNfcWarning {
    fn on_text(&mut self, text: &str, span: Option<Span>, out: &mut dyn MessageSink) {
        if self.text_run_span.is_none() {
            self.text_run_span = span;
        }

        if self.already_complained_about_this_run {
            return;
        }

        if self.at_start_of_run {
            if starts_with_composing_char(text) {
                out.push(Message::new(
                    "i18n.unicode.composing.start_text_run",
                    Severity::Warning,
                    Category::I18n,
                    "Text run starts with a composing character.",
                    span,
                ));
            }
            self.at_start_of_run = false;
        }

        let mut remaining = text;

        // If we have a pending suffix from a previous chunk, extend it with any leading composing
        // characters from this chunk, then validate the buffer if we now have a non-composing
        // character.
        if !self.text_buf.is_empty() {
            let leading_composing_end = leading_composing_end(remaining);
            self.text_buf.push_str(&remaining[..leading_composing_end]);
            remaining = &remaining[leading_composing_end..];
            if remaining.is_empty() {
                return;
            }

            if !is_nfc(&self.text_buf) {
                let normalized = self.text_buf.nfc().collect::<String>();
                self.err_about_text_run(normalized, out);
                return;
            }

            self.text_buf.clear();
        }

        // Mimic the vnu.jar NormalizationChecker behavior: validate the current chunk excluding
        // the final non-composing character (and any trailing composing characters), buffering
        // that suffix for checking once the run boundary is known.
        if remaining.is_empty() {
            return;
        }

        let Some(last_non_composing_start) = last_non_composing_char_start(remaining) else {
            self.text_buf.push_str(remaining);
            return;
        };

        if last_non_composing_start > 0 {
            let prefix = &remaining[..last_non_composing_start];
            if !is_nfc(prefix) {
                let normalized = prefix.nfc().collect::<String>();
                self.err_about_text_run(normalized, out);
                return;
            }
        }

        self.text_buf
            .push_str(&remaining[last_non_composing_start..]);
    }

    fn flush_text_run(&mut self, out: &mut dyn MessageSink) {
        if !self.already_complained_about_this_run
            && !self.text_buf.is_empty()
            && !is_nfc(&self.text_buf)
        {
            let normalized = self.text_buf.nfc().collect::<String>();
            self.err_about_text_run(normalized, out);
        }

        self.text_buf.clear();
        self.text_run_span = None;
        self.at_start_of_run = true;
        self.already_complained_about_this_run = false;
    }

    fn err_about_text_run(&mut self, normalized_text: String, out: &mut dyn MessageSink) {
        out.push(Message::new(
            "i18n.unicode.nfc.not_nfc",
            Severity::Warning,
            Category::I18n,
            format!(
                "Text run is not in Unicode Normalization Form C. Should instead be “{normalized_text}”. (Copy and paste that into your source document to replace the un-normalized text.)"
            ),
            self.text_run_span,
        ));
        self.already_complained_about_this_run = true;
    }

    fn check_processing_instruction(
        &mut self,
        target: &str,
        data: &str,
        span: Option<Span>,
        out: &mut dyn MessageSink,
    ) {
        if !target.is_empty() && starts_with_composing_char(target) {
            out.push(Message::new(
                "i18n.unicode.composing.pi_target",
                Severity::Warning,
                Category::I18n,
                "Processing instruction target starts with a composing character.",
                span,
            ));
        }
        if !data.is_empty() {
            if starts_with_composing_char(data) {
                out.push(Message::new(
                    "i18n.unicode.composing.pi_data",
                    Severity::Warning,
                    Category::I18n,
                    "Processing instruction data starts with a composing character.",
                    span,
                ));
            } else if !is_nfc(data) {
                out.push(Message::new(
                    "i18n.unicode.nfc.pi_data_not_nfc",
                    Severity::Warning,
                    Category::I18n,
                    "Processing instruction data in not in Unicode Normalization Form C.",
                    span,
                ));
            }
        }
    }

    fn check_element_and_attrs(
        &mut self,
        name: &str,
        attrs: &[html_inspector::Attribute],
        span: Option<Span>,
        out: &mut dyn MessageSink,
    ) {
        if !name.is_empty() && starts_with_composing_char(name) {
            out.push(Message::new(
                "i18n.unicode.composing.element_name",
                Severity::Warning,
                Category::I18n,
                format!("Element name “ {name}” starts with a composing character."),
                span,
            ));
        }

        for attr in attrs {
            if !attr.name.is_empty() && starts_with_composing_char(&attr.name) {
                out.push(Message::new(
                    "i18n.unicode.composing.attribute_name",
                    Severity::Warning,
                    Category::I18n,
                    format!(
                        "Attribute name “ {}” starts with a composing character.",
                        attr.name
                    ),
                    attr.span.or(span),
                ));
            }

            let Some(value) = &attr.value else {
                continue;
            };
            if value.is_empty() {
                continue;
            }

            if starts_with_composing_char(value) {
                out.push(Message::new(
                    "i18n.unicode.composing.attribute_value",
                    Severity::Warning,
                    Category::I18n,
                    format!(
                        "The value of attribute “{}” on element “{}” starts with a composing character.",
                        attr.name, name
                    ),
                    attr.span.or(span),
                ));
            } else if !is_nfc(value) {
                out.push(Message::new(
                    "i18n.unicode.nfc.attribute_value_not_nfc",
                    Severity::Warning,
                    Category::I18n,
                    format!(
                        "The value of attribute “{}” on element “{}” is not in Unicode Normalization Form C.",
                        attr.name, name
                    ),
                    attr.span.or(span),
                ));
            }
        }
    }
}

fn is_composing_char(c: char) -> bool {
    if unicode_normalization::char::canonical_combining_class(c) != 0 {
        return true;
    }
    matches!(
        unicode_normalization::is_nfc_quick(std::iter::once(c)),
        unicode_normalization::IsNormalized::Maybe
    )
}

fn starts_with_composing_char(s: &str) -> bool {
    s.chars().next().is_some_and(is_composing_char)
}

fn leading_composing_end(s: &str) -> usize {
    for (idx, ch) in s.char_indices() {
        if !is_composing_char(ch) {
            return idx;
        }
    }
    s.len()
}

fn last_non_composing_char_start(s: &str) -> Option<usize> {
    let mut last = None;
    for (idx, ch) in s.char_indices() {
        if !is_composing_char(ch) {
            last = Some(idx);
        }
    }
    last
}
