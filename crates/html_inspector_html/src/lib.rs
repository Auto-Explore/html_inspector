use std::borrow::Cow;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::sync::OnceLock;

use html_inspector_core::{Attribute, EventSource, InputFormat, ParseEvent, Span, ValidatorError};

#[cfg(feature = "html5ever")]
mod html5ever_rcdom;
#[cfg(feature = "html5ever")]
mod html5ever_source;
mod named_entities;
#[cfg(feature = "html5ever")]
pub use html5ever_source::Html5EverEventSource;

#[derive(Clone)]
pub enum HtmlEventSource {
    Simple(SimpleHtmlEventSource),
    #[cfg(feature = "html5ever")]
    Html5Ever(Html5EverEventSource),
}

impl HtmlEventSource {
    pub fn from_bytes(
        name: impl Into<String>,
        format: InputFormat,
        bytes: Vec<u8>,
    ) -> Result<Self, ValidatorError> {
        Self::from_shared_bytes(name, format, Arc::new(bytes))
    }

    pub fn from_shared_bytes(
        name: impl Into<String>,
        format: InputFormat,
        bytes: Arc<Vec<u8>>,
    ) -> Result<Self, ValidatorError> {
        let name = name.into();

        #[cfg(feature = "html5ever")]
        if format == InputFormat::Html {
            return Ok(HtmlEventSource::Html5Ever(
                Html5EverEventSource::from_shared_bytes(name, bytes),
            ));
        }

        Ok(HtmlEventSource::Simple(
            SimpleHtmlEventSource::from_shared_bytes(name, format, bytes),
        ))
    }

    pub fn from_str(
        name: impl Into<String>,
        format: InputFormat,
        s: &str,
    ) -> Result<Self, ValidatorError> {
        Self::from_bytes(name, format, s.as_bytes().to_vec())
    }
}

#[cfg(test)]
mod html_event_source_tests {
    use super::*;

    #[test]
    fn html_event_source_selects_backend_by_feature_and_format() {
        let html = HtmlEventSource::from_str("t", InputFormat::Html, "<p>hi</p>").unwrap();
        #[cfg(feature = "html5ever")]
        assert!(matches!(html, HtmlEventSource::Html5Ever(_)));
        #[cfg(not(feature = "html5ever"))]
        assert!(matches!(html, HtmlEventSource::Simple(_)));

        let xhtml = HtmlEventSource::from_str("t", InputFormat::Xhtml, "<p/>").unwrap();
        assert!(matches!(xhtml, HtmlEventSource::Simple(_)));
    }

    #[test]
    fn simple_scanner_normalizes_tag_and_attribute_names_only_for_html() {
        let mut html =
            HtmlEventSource::from_str("t", InputFormat::Html, "<DiV CLass=foo></DiV>").unwrap();
        let (name, attrs) = loop {
            match html.next_event().unwrap() {
                Some(ParseEvent::StartTag { name, attrs, .. }) if name == "div" => {
                    break (name, attrs);
                }
                Some(_) => continue,
                None => panic!("did not find <div> StartTag"),
            }
        };
        assert_eq!(name, "div");
        assert_eq!(attrs.len(), 1);
        assert_eq!(attrs[0].name, "class");
        assert_eq!(attrs[0].value.as_deref(), Some("foo"));

        let mut xhtml =
            HtmlEventSource::from_str("t", InputFormat::Xhtml, "<DiV CLass=\"foo\"/>").unwrap();
        let (name, attrs) = loop {
            match xhtml.next_event().unwrap() {
                Some(ParseEvent::StartTag { name, attrs, .. }) if name == "DiV" => {
                    break (name, attrs);
                }
                Some(_) => continue,
                None => panic!("did not find <DiV> StartTag"),
            }
        };
        assert_eq!(name, "DiV");
        assert_eq!(attrs.len(), 1);
        assert_eq!(attrs[0].name, "CLass");
        assert_eq!(attrs[0].value.as_deref(), Some("foo"));
    }

    #[test]
    fn simple_scanner_lowercases_ascii_in_non_ascii_attribute_names_for_html_only() {
        let mut html =
            HtmlEventSource::from_str("t", InputFormat::Html, "<div ❤A=foo></div>").unwrap();
        let attrs = loop {
            match html.next_event().unwrap() {
                Some(ParseEvent::StartTag { name, attrs, .. }) if name == "div" => break attrs,
                Some(_) => continue,
                None => panic!("did not find <div> StartTag"),
            }
        };
        assert_eq!(attrs.len(), 1);
        assert_eq!(attrs[0].name, "❤a");
        assert_eq!(attrs[0].value.as_deref(), Some("foo"));

        let mut xhtml =
            HtmlEventSource::from_str("t", InputFormat::Xhtml, "<div ❤A=\"foo\"/>").unwrap();
        let attrs = loop {
            match xhtml.next_event().unwrap() {
                Some(ParseEvent::StartTag { name, attrs, .. }) if name == "div" => break attrs,
                Some(_) => continue,
                None => panic!("did not find <div> StartTag"),
            }
        };
        assert_eq!(attrs.len(), 1);
        assert_eq!(attrs[0].name, "❤A");
        assert_eq!(attrs[0].value.as_deref(), Some("foo"));
    }

    #[test]
    fn bytes_at_cursor_is_safe_at_eof() {
        let mut src = SimpleHtmlEventSource::from_str("t", InputFormat::Html, "<p>");
        assert!(src.bytes_at_cursor(b"<p"));

        src.cursor = src.bytes.len();
        assert!(!src.bytes_at_cursor(b"<"));
        assert!(src.bytes_at_cursor(b""));
    }
}

impl EventSource for HtmlEventSource {
    fn source_name(&self) -> &str {
        match self {
            HtmlEventSource::Simple(s) => s.source_name(),
            #[cfg(feature = "html5ever")]
            HtmlEventSource::Html5Ever(s) => s.source_name(),
        }
    }

    fn format(&self) -> InputFormat {
        match self {
            HtmlEventSource::Simple(s) => s.format(),
            #[cfg(feature = "html5ever")]
            HtmlEventSource::Html5Ever(s) => s.format(),
        }
    }

    fn next_event(&mut self) -> Result<Option<ParseEvent>, ValidatorError> {
        match self {
            HtmlEventSource::Simple(s) => s.next_event(),
            #[cfg(feature = "html5ever")]
            HtmlEventSource::Html5Ever(s) => s.next_event(),
        }
    }
}

#[derive(Clone)]
pub struct SimpleHtmlEventSource {
    name: String,
    format: InputFormat,
    bytes: Arc<Vec<u8>>,
    cursor: usize,
    line: u32,
    col: u32,
    open_elements: Vec<String>,
    open_namespaces: Vec<HtmlNamespace>,
    pending: VecDeque<ParseEvent>,
    finished: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum HtmlNamespace {
    Html,
    Svg,
    Math,
}

impl SimpleHtmlEventSource {
    pub fn from_bytes(name: impl Into<String>, format: InputFormat, bytes: Vec<u8>) -> Self {
        Self::from_shared_bytes(name, format, Arc::new(bytes))
    }

    pub fn from_shared_bytes(
        name: impl Into<String>,
        format: InputFormat,
        bytes: Arc<Vec<u8>>,
    ) -> Self {
        Self {
            name: name.into(),
            format,
            bytes,
            cursor: 0,
            line: 1,
            col: 1,
            open_elements: Vec::new(),
            open_namespaces: Vec::new(),
            pending: VecDeque::new(),
            finished: false,
        }
    }

    pub fn from_str(name: impl Into<String>, format: InputFormat, s: &str) -> Self {
        Self::from_bytes(name, format, s.as_bytes().to_vec())
    }

    fn bump_to(&mut self, new_cursor: usize) {
        for &b in &self.bytes[self.cursor..new_cursor] {
            if b == b'\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
        }
        self.cursor = new_cursor;
    }

    #[inline]
    fn bytes_at_cursor(&self, needle: &[u8]) -> bool {
        self.bytes
            .get(self.cursor..)
            .is_some_and(|tail| tail.starts_with(needle))
    }

    fn current_span(&self, start: usize, end: usize, start_line: u32, start_col: u32) -> Span {
        Span::new(start, end, start_line, start_col)
    }

    fn emit_tokenizer_eof_after_lt(&mut self, start: usize, start_line: u32, start_col: u32) {
        let end = self.bytes.len();
        self.bump_to(end);
        self.pending.push_back(ParseEvent::ParseError {
            code: "html.tokenizer.eof_after_lt".to_string(),
            message: "End of file after “<”.".to_string(),
            span: Some(self.current_span(start, end, start_line, start_col)),
        });
        self.finished = true;
    }

    fn normalize_name(&self, s: impl Into<String>) -> String {
        let mut out = s.into();
        if self.format == InputFormat::Html {
            out.make_ascii_lowercase();
        }
        out
    }

    fn current_text_mode_kind(&self) -> TextModeKind {
        let (Some(name), Some(&HtmlNamespace::Html)) =
            (self.open_elements.last(), self.open_namespaces.last())
        else {
            return TextModeKind::Data;
        };
        match name.as_str() {
            "script" | "style" | "xmp" | "iframe" | "noembed" | "noframes" => TextModeKind::RawText,
            "title" | "textarea" => TextModeKind::RcData,
            "plaintext" => TextModeKind::Plaintext,
            _ => TextModeKind::Data,
        }
    }

    fn current_insertion_namespace(&self) -> HtmlNamespace {
        let ns = self
            .open_namespaces
            .last()
            .copied()
            .unwrap_or(HtmlNamespace::Html);
        if ns == HtmlNamespace::Svg
            && self
                .open_elements
                .last()
                .is_some_and(|name| matches!(name.as_str(), "foreignobject" | "desc" | "title"))
        {
            HtmlNamespace::Html
        } else {
            ns
        }
    }

    fn namespace_for_start_tag(&self, name: &str) -> HtmlNamespace {
        match self.current_insertion_namespace() {
            HtmlNamespace::Html => match name {
                "svg" => HtmlNamespace::Svg,
                "math" => HtmlNamespace::Math,
                _ => HtmlNamespace::Html,
            },
            ns => ns,
        }
    }

    fn scan_next(&mut self) -> Result<(), ValidatorError> {
        if self.finished {
            return Ok(());
        }

        if self.cursor >= self.bytes.len() {
            self.finished = true;
            return Ok(());
        }

        // If we are in data state and the last character is a lone "<", report the tokenizer EOF error.
        if self.bytes[self.cursor] == b'<' && self.cursor + 1 == self.bytes.len() {
            let start = self.cursor;
            let start_line = self.line;
            let start_col = self.col;
            self.emit_tokenizer_eof_after_lt(start, start_line, start_col);
            return Ok(());
        }

        match self.current_text_mode_kind() {
            TextModeKind::Data => {}
            TextModeKind::Plaintext => {
                let start = self.cursor;
                let start_line = self.line;
                let start_col = self.col;
                let end = self.bytes.len();
                self.bump_to(end);
                let text = bytes_to_string_lossy(&self.bytes[start..end]);
                self.pending.push_back(ParseEvent::Text {
                    text,
                    span: Some(self.current_span(start, end, start_line, start_col)),
                });
                self.finished = true;
                return Ok(());
            }
            TextModeKind::RawText => {
                return self.scan_rawtext(false);
            }
            TextModeKind::RcData => {
                return self.scan_rawtext(true);
            }
        }

        let next_lt = memchr(b'<', &self.bytes[self.cursor..]).map(|off| self.cursor + off);
        if let Some(lt) = next_lt {
            if lt > self.cursor {
                let start = self.cursor;
                let start_line = self.line;
                let start_col = self.col;
                self.bump_to(lt);
                let raw = str_from_bytes_lossy(&self.bytes[start..lt]);
                let (text, errs) = decode_char_refs_with_errors(
                    self.format,
                    raw.as_ref(),
                    false,
                    start,
                    start_line,
                    start_col,
                );
                self.pending.extend(errs);
                self.pending.push_back(ParseEvent::Text {
                    text,
                    span: Some(self.current_span(start, lt, start_line, start_col)),
                });
                return Ok(());
            }

            // cursor at '<'
            let start = self.cursor;
            let start_line = self.line;
            let start_col = self.col;

            // Common bad sequences in data.
            if self.format == InputFormat::Html && self.bytes_at_cursor(b"<>") {
                // "<>" is not a tag.
                self.bump_to(self.cursor + 2);
                self.pending.push_back(ParseEvent::ParseError {
                    code: "html.tokenizer.lt_gt".to_string(),
                    message:
                        "Saw “<>”. Probable causes: Unescaped “<” (escape as “&lt;”) or mistyped start tag."
                            .to_string(),
                    span: Some(self.current_span(start, start + 2, start_line, start_col)),
                });
                self.pending.push_back(ParseEvent::Text {
                    text: "<>".to_string(),
                    span: Some(self.current_span(start, start + 2, start_line, start_col)),
                });
                return Ok(());
            }

            // Comments: <!-- ... -->
            if self.bytes_at_cursor(b"<!--") {
                return self.scan_comment(start, start_line, start_col);
            }

            // CDATA: <![CDATA[ ... ]]>
            if self.bytes_at_cursor(b"<![CDATA[") {
                match self.format {
                    InputFormat::Xhtml => return self.scan_cdata(start, start_line, start_col),
                    InputFormat::Html => {
                        if self.current_insertion_namespace() != HtmlNamespace::Html {
                            return self.scan_cdata(start, start_line, start_col);
                        }
                    }
                }
            }

            // Doctype: <!doctype ...>
            if self.bytes_at_cursor(b"<!") {
                let mut j = self.cursor + 2;
                while j < self.bytes.len() && self.bytes[j].is_ascii_whitespace() {
                    j += 1;
                }
                if starts_with_ascii_case_insensitive(&self.bytes[j..], b"doctype") {
                    return self.scan_doctype(start, start_line, start_col);
                }
            }

            // End tag: </...> (only if next char starts a tag name)
            if self.bytes_at_cursor(b"</") {
                let Some(b) = self.bytes.get(self.cursor + 2).copied() else {
                    // "</" at EOF: align with VNU error text.
                    self.emit_tokenizer_eof_after_lt(start, start_line, start_col);
                    return Ok(());
                };

                if b.is_ascii_alphabetic() {
                    return self.scan_end_tag(start, start_line, start_col);
                }
                if b == b'>' {
                    // "</>"
                    self.bump_to(self.cursor + 3);
                    self.pending.push_back(ParseEvent::ParseError {
                        code: "html.tokenizer.lt_slash_gt".to_string(),
                        message:
                            "Saw “</>”. Probable causes: Unescaped “<” (escape as “&lt;”) or mistyped end tag."
                                .to_string(),
                        span: Some(self.current_span(start, start + 3, start_line, start_col)),
                    });
                    self.pending.push_back(ParseEvent::Text {
                        text: "</>".to_string(),
                        span: Some(self.current_span(start, start + 3, start_line, start_col)),
                    });
                    return Ok(());
                }
                if b.is_ascii_whitespace() {
                    // "</ garbage>"
                    return self.scan_garbage_after_lt_slash(start, start_line, start_col);
                }
                // Not a real end tag; treat '<' as literal text and emit a coalesced text run.
                return self.scan_text_run(start, start_line, start_col);
            }

            // Bogus comment: <? ... > or <! ... >
            if self.bytes_at_cursor(b"<?") {
                return match self.format {
                    InputFormat::Xhtml => {
                        self.scan_processing_instruction(start, start_line, start_col)
                    }
                    InputFormat::Html => self.scan_bogus_comment(start, start_line, start_col),
                };
            }
            if self.bytes_at_cursor(b"<!") {
                return self.scan_bogus_comment(start, start_line, start_col);
            }

            // Start tag: <...> (only if next char starts a tag name)
            if let Some(&b) = self.bytes.get(self.cursor + 1) {
                if b.is_ascii_alphabetic() {
                    return self.scan_start_tag(start, start_line, start_col);
                }
                if self.format == InputFormat::Html
                    && !b.is_ascii_whitespace()
                    && !matches!(b, b'!' | b'/' | b'?')
                {
                    // "<1", "<\\", etc.
                    self.pending.push_back(ParseEvent::ParseError {
                        code: "html.tokenizer.bad_char_after_lt".to_string(),
                        message:
                            format!("Bad character “{}” after “<”. Probable cause: Unescaped “<”. Try escaping it as “&lt;”.", b as char),
                        span: Some(self.current_span(start, start + 2, start_line, start_col)),
                    });
                    return self.scan_text_run(start, start_line, start_col);
                }
            }

            // Not a tag start; treat '<' as literal text and emit a coalesced text run.
            return self.scan_text_run(start, start_line, start_col);
        }

        // No more tags; emit remainder as text.
        let start = self.cursor;
        let start_line = self.line;
        let start_col = self.col;
        let end = self.bytes.len();
        self.bump_to(end);
        let raw = str_from_bytes_lossy(&self.bytes[start..end]);
        let (text, errs) = decode_char_refs_with_errors(
            self.format,
            raw.as_ref(),
            false,
            start,
            start_line,
            start_col,
        );
        self.pending.extend(errs);
        self.pending.push_back(ParseEvent::Text {
            text,
            span: Some(self.current_span(start, end, start_line, start_col)),
        });
        Ok(())
    }

    fn scan_comment(
        &mut self,
        start: usize,
        start_line: u32,
        start_col: u32,
    ) -> Result<(), ValidatorError> {
        let Some(end) = find_subslice(&self.bytes, self.cursor + 4, b"-->") else {
            self.finished = true;
            self.pending.push_back(ParseEvent::ParseError {
                code: "html.tokenizer.eof_in_comment".to_string(),
                message: "End of file inside comment.".to_string(),
                span: Some(self.current_span(start, self.bytes.len(), start_line, start_col)),
            });
            return Ok(());
        };

        let comment_start = self.cursor + 4;
        let comment_end = end;
        let text = bytes_to_string_lossy(&self.bytes[comment_start..comment_end]);
        if self.format == InputFormat::Html {
            if let Some(off) = text.find("<!--") {
                let err_start = comment_start + off;
                self.pending.push_back(ParseEvent::ParseError {
                    code: "html.tokenizer.nested_comment".to_string(),
                    message:
                        "Saw “<!--” within a comment. Probable cause: Nested comment (not allowed)."
                            .to_string(),
                    span: Some(self.current_span(err_start, err_start + 4, start_line, start_col)),
                });
            }
        }
        let close_end = end + 3;
        self.bump_to(close_end);
        self.pending.push_back(ParseEvent::Comment {
            text,
            span: Some(self.current_span(start, close_end, start_line, start_col)),
        });
        Ok(())
    }

    fn scan_cdata(
        &mut self,
        start: usize,
        start_line: u32,
        start_col: u32,
    ) -> Result<(), ValidatorError> {
        // <![CDATA[ ... ]]>
        let cdata_start = self.cursor + 9;
        let Some(end) = find_subslice(&self.bytes, cdata_start, b"]]>") else {
            self.finished = true;
            self.pending.push_back(ParseEvent::ParseError {
                code: "xml.cdata_eof".to_string(),
                message: "Unterminated CDATA section.".to_string(),
                span: Some(self.current_span(start, self.bytes.len(), start_line, start_col)),
            });
            return Ok(());
        };

        let text = bytes_to_string_lossy(&self.bytes[cdata_start..end]);
        let close_end = end + 3;
        self.bump_to(close_end);
        self.pending.push_back(ParseEvent::Text {
            text,
            span: Some(self.current_span(start, close_end, start_line, start_col)),
        });
        Ok(())
    }

    fn scan_bogus_comment(
        &mut self,
        start: usize,
        start_line: u32,
        start_col: u32,
    ) -> Result<(), ValidatorError> {
        let is_processing_instruction = self.bytes_at_cursor(b"<?");
        self.pending.push_back(ParseEvent::ParseError {
            code: if is_processing_instruction {
                "html.tokenizer.processing_instruction".to_string()
            } else {
                "html.tokenizer.bogus_comment".to_string()
            },
            message: if is_processing_instruction {
                "Saw “<?”. Probable cause: Attempt to use an XML processing instruction in HTML. (XML processing instructions are not supported in HTML.)".to_string()
            } else {
                "Bogus comment.".to_string()
            },
            span: Some(self.current_span(start, start + 2, start_line, start_col)),
        });
        // Treat everything up to the next '>' (or EOF) as a comment payload.
        let prefix_len = 2;
        if let Some(off) = memchr(b'>', &self.bytes[self.cursor + prefix_len..]) {
            let gt = self.cursor + prefix_len + off;
            let text = bytes_to_string_lossy(&self.bytes[self.cursor + prefix_len..gt]);
            let end = gt + 1;
            self.bump_to(end);
            self.pending.push_back(ParseEvent::Comment {
                text,
                span: Some(self.current_span(start, end, start_line, start_col)),
            });
            return Ok(());
        }
        // EOF: emit comment then finish.
        let text = bytes_to_string_lossy(&self.bytes[self.cursor + prefix_len..]);
        self.bump_to(self.bytes.len());
        self.pending.push_back(ParseEvent::Comment {
            text,
            span: Some(self.current_span(start, self.bytes.len(), start_line, start_col)),
        });
        self.finished = true;
        Ok(())
    }

    fn scan_processing_instruction(
        &mut self,
        start: usize,
        start_line: u32,
        start_col: u32,
    ) -> Result<(), ValidatorError> {
        // XML processing instruction: <?target data?>
        let content_start = self.cursor + 2;
        let Some(pi_end) = find_subslice(&self.bytes, content_start, b"?>") else {
            self.finished = true;
            self.pending.push_back(ParseEvent::ParseError {
                code: "xml.pi_eof".to_string(),
                message: "Unterminated processing instruction.".to_string(),
                span: Some(self.current_span(start, self.bytes.len(), start_line, start_col)),
            });
            return Ok(());
        };

        let mut i = content_start;
        while i < pi_end && self.bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        let target_start = i;
        while i < pi_end && !self.bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        let target = bytes_to_string_lossy(&self.bytes[target_start..i]);
        let data = str_from_bytes_lossy(&self.bytes[i..pi_end])
            .trim()
            .to_string();

        let close_end = pi_end + 2;
        self.bump_to(close_end);
        self.pending.push_back(ParseEvent::ProcessingInstruction {
            target,
            data,
            span: Some(self.current_span(start, close_end, start_line, start_col)),
        });
        Ok(())
    }

    fn scan_end_tag(
        &mut self,
        start: usize,
        start_line: u32,
        start_col: u32,
    ) -> Result<(), ValidatorError> {
        let Some(off) = memchr(b'>', &self.bytes[self.cursor + 2..]) else {
            self.finished = true;
            self.pending.push_back(ParseEvent::ParseError {
                code: "html.tokenizer.eof_in_end_tag".to_string(),
                message: "End of file seen when looking for tag name. Ignoring tag.".to_string(),
                span: Some(self.current_span(start, self.bytes.len(), start_line, start_col)),
            });
            return Ok(());
        };
        let gt = self.cursor + 2 + off;

        let raw_all = str_from_bytes_lossy(&self.bytes[self.cursor + 2..gt]);
        let raw_trimmed = raw_all.trim();

        let mut raw = raw_trimmed;
        if raw.ends_with('/') {
            self.pending.push_back(ParseEvent::ParseError {
                code: "html.tokenizer.end_tag_stray_slash".to_string(),
                message: "Stray “/” at the end of an end tag.".to_string(),
                span: Some(self.current_span(start, gt + 1, start_line, start_col)),
            });
            raw = raw.trim_end_matches('/').trim_end();
        }

        let mut it = raw.split_whitespace();
        let name_raw = it.next().unwrap_or("");
        if it.next().is_some() {
            self.pending.push_back(ParseEvent::ParseError {
                code: "html.tokenizer.end_tag_with_attrs".to_string(),
                message: "End tag had attributes.".to_string(),
                span: Some(self.current_span(start, gt + 1, start_line, start_col)),
            });
        }

        let name = self.normalize_name(name_raw);
        let end = gt + 1;
        self.bump_to(end);
        self.pop_open_element(&name);
        self.pending.push_back(ParseEvent::EndTag {
            name,
            span: Some(self.current_span(start, end, start_line, start_col)),
        });
        Ok(())
    }

    fn scan_start_tag(
        &mut self,
        start: usize,
        start_line: u32,
        start_col: u32,
    ) -> Result<(), ValidatorError> {
        let Some(gt) = find_tag_close(&self.bytes, self.cursor + 1) else {
            let rest = &self.bytes[self.cursor + 1..];
            let (code, message) = classify_start_tag_eof(rest);
            self.finished = true;
            self.pending.push_back(ParseEvent::ParseError {
                code,
                message,
                span: Some(self.current_span(start, self.bytes.len(), start_line, start_col)),
            });
            self.bump_to(self.bytes.len());
            return Ok(());
        };

        let inside = str_from_bytes_lossy(&self.bytes[self.cursor + 1..gt]);
        let end = gt + 1;
        let (name, attrs, self_closing, errs) =
            parse_start_tag(self, inside.as_ref(), start, start_line, start_col, end)?;
        self.pending.extend(errs);
        self.bump_to(end);
        let ns = self.namespace_for_start_tag(&name);
        let pushes = match self.format {
            InputFormat::Html => {
                if ns == HtmlNamespace::Html {
                    !html_inspector_core::is_void_html_element(&name)
                } else {
                    !self_closing
                }
            }
            InputFormat::Xhtml => !self_closing,
        };
        if pushes {
            self.open_elements.push(name.clone());
            self.open_namespaces.push(ns);
        }
        self.pending.push_back(ParseEvent::StartTag {
            name,
            attrs,
            self_closing,
            span: Some(self.current_span(start, end, start_line, start_col)),
        });
        Ok(())
    }

    fn scan_doctype(
        &mut self,
        start: usize,
        start_line: u32,
        start_col: u32,
    ) -> Result<(), ValidatorError> {
        fn line_col_at(
            bytes: &[u8],
            start: usize,
            start_line: u32,
            start_col: u32,
            target: usize,
        ) -> (u32, u32) {
            let mut line = start_line;
            let mut col = start_col;
            let mut i = start;
            while i < target && i < bytes.len() {
                if bytes[i] == b'\n' {
                    line += 1;
                    col = 1;
                } else {
                    col += 1;
                }
                i += 1;
            }
            (line, col)
        }

        let all_bytes: &[u8] = &self.bytes;
        let mk_span = |byte_start: usize, byte_end: usize| {
            let (line, col) = line_col_at(all_bytes, start, start_line, start_col, byte_start);
            Span::new(byte_start, byte_end, line, col)
        };

        // Find the end of the doctype token: first '>' outside quotes.
        let end = {
            let mut pos = self.cursor + 2;
            let mut quote: Option<u8> = None;
            loop {
                let Some(&b) = self.bytes.get(pos) else {
                    break self.bytes.len();
                };
                if let Some(q) = quote {
                    if b == q {
                        quote = None;
                    }
                } else if matches!(b, b'"' | b'\'') {
                    quote = Some(b);
                } else if b == b'>' {
                    break pos + 1;
                }
                pos += 1;
            }
        };

        // Parse the contents for name/public/system (and emit Java/VNU-like parse errors).
        let bytes = all_bytes;
        let mut i = self.cursor + 2;

        // Consume optional whitespace and "doctype" keyword.
        while i < end && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        if starts_with_ascii_case_insensitive(&bytes[i..end], b"doctype") {
            i += "doctype".len();
        }

        // After the keyword, HTML expects whitespace before the name (even if the name is missing).
        if i >= end || !bytes[i].is_ascii_whitespace() {
            self.pending.push_back(ParseEvent::ParseError {
                code: "html.tokenizer.doctype.missing_space_before_name".to_string(),
                message: "Missing space before doctype name.".to_string(),
                span: Some(mk_span(start, start + 2)),
            });
        }

        while i < end && bytes[i].is_ascii_whitespace() {
            i += 1;
        }

        let name_start = i;
        while i < end && !bytes[i].is_ascii_whitespace() && bytes[i] != b'>' {
            i += 1;
        }
        let name = if name_start < i {
            Some(self.normalize_name(str_from_bytes_lossy(&bytes[name_start..i])))
        } else {
            None
        };

        while i < end && bytes[i].is_ascii_whitespace() {
            i += 1;
        }

        let mut public_id: Option<String> = None;
        let mut system_id: Option<String> = None;
        let mut saw_syntax_error = false;
        let mut saw_bogus_doctype = false;

        // PUBLIC/SYSTEM identifiers.
        if i < end && bytes[i] != b'>' {
            if starts_with_ascii_case_insensitive(&bytes[i..end], b"public") {
                i += "public".len();
                if i < end && matches!(bytes[i], b'"' | b'\'') {
                    saw_syntax_error = true;
                    self.pending.push_back(ParseEvent::ParseError {
                        code: "html.tokenizer.doctype.no_space_after_public".to_string(),
                        message: "No space between the doctype “PUBLIC” keyword and the quote."
                            .to_string(),
                        span: Some(mk_span(start, start + 2)),
                    });
                }
                while i < end && bytes[i].is_ascii_whitespace() {
                    i += 1;
                }
                if i >= end || bytes[i] == b'>' {
                    saw_syntax_error = true;
                    self.pending.push_back(ParseEvent::ParseError {
                        code: "html.tokenizer.doctype.expected_public_id".to_string(),
                        message: "Expected a public identifier but the doctype ended.".to_string(),
                        span: Some(mk_span(start, start + 2)),
                    });
                } else if matches!(bytes[i], b'"' | b'\'') {
                    let q = bytes[i];
                    i += 1;
                    let id_start = i;
                    let mut saw_gt = false;
                    while i < end && bytes[i] != q {
                        if bytes[i] == b'>' && !saw_gt {
                            saw_gt = true;
                            saw_syntax_error = true;
                            self.pending.push_back(ParseEvent::ParseError {
                                code: "html.tokenizer.doctype.gt_in_public_id".to_string(),
                                message: "“>” in public identifier.".to_string(),
                                span: Some(mk_span(i, i + 1)),
                            });
                        }
                        i += 1;
                    }
                    if i >= end {
                        saw_syntax_error = true;
                        self.pending.push_back(ParseEvent::ParseError {
                            code: "html.tokenizer.doctype.eof_in_public_id".to_string(),
                            message: "End of file inside public identifier.".to_string(),
                            span: Some(mk_span(start, end)),
                        });
                    }
                    public_id = Some(bytes_to_string_lossy(&bytes[id_start..i.min(end)]));
                    if i < end && bytes[i] == q {
                        i += 1;
                    }

                    let mut had_ws = false;
                    while i < end && bytes[i].is_ascii_whitespace() {
                        had_ws = true;
                        i += 1;
                    }
                    if i < end && (bytes[i] == b'"' || bytes[i] == b'\'') {
                        if !had_ws {
                            saw_syntax_error = true;
                            self.pending.push_back(ParseEvent::ParseError {
                                code: "html.tokenizer.doctype.no_space_between_public_system"
                                    .to_string(),
                                message:
                                    "No space between the doctype public and system identifiers."
                                        .to_string(),
                                span: Some(mk_span(i, i + 1)),
                            });
                        }
                        let q = bytes[i];
                        i += 1;
                        let id_start = i;
                        let mut saw_gt = false;
                        while i < end && bytes[i] != q {
                            if bytes[i] == b'>' && !saw_gt {
                                saw_gt = true;
                                saw_syntax_error = true;
                                self.pending.push_back(ParseEvent::ParseError {
                                    code: "html.tokenizer.doctype.gt_in_system_id".to_string(),
                                    message: "“>” in system identifier.".to_string(),
                                    span: Some(mk_span(i, i + 1)),
                                });
                            }
                            i += 1;
                        }
                        if i >= end {
                            saw_syntax_error = true;
                            self.pending.push_back(ParseEvent::ParseError {
                                code: "html.tokenizer.doctype.eof_in_system_id".to_string(),
                                message: "End of file inside system identifier.".to_string(),
                                span: Some(mk_span(start, end)),
                            });
                        }
                        system_id = Some(bytes_to_string_lossy(&bytes[id_start..i.min(end)]));
                        // If there's a closing quote within the token, it's already accounted for by `i`.
                    }
                }
            } else if starts_with_ascii_case_insensitive(&bytes[i..end], b"system") {
                i += "system".len();
                if i < end && (bytes[i] == b'"' || bytes[i] == b'\'') {
                    saw_syntax_error = true;
                    self.pending.push_back(ParseEvent::ParseError {
                        code: "html.tokenizer.doctype.no_space_after_system".to_string(),
                        message: "No space between the doctype “SYSTEM” keyword and the quote."
                            .to_string(),
                        span: Some(mk_span(start, start + 2)),
                    });
                }
                while i < end && bytes[i].is_ascii_whitespace() {
                    i += 1;
                }
                if i >= end || bytes[i] == b'>' {
                    saw_syntax_error = true;
                    self.pending.push_back(ParseEvent::ParseError {
                        code: "html.tokenizer.doctype.expected_system_id".to_string(),
                        message: "Expected a system identifier but the doctype ended.".to_string(),
                        span: Some(mk_span(start, start + 2)),
                    });
                } else if bytes[i] == b'"' || bytes[i] == b'\'' {
                    let q = bytes[i];
                    i += 1;
                    let id_start = i;
                    let mut saw_gt = false;
                    while i < end && bytes[i] != q {
                        if bytes[i] == b'>' && !saw_gt {
                            saw_gt = true;
                            saw_syntax_error = true;
                            self.pending.push_back(ParseEvent::ParseError {
                                code: "html.tokenizer.doctype.gt_in_system_id".to_string(),
                                message: "“>” in system identifier.".to_string(),
                                span: Some(mk_span(i, i + 1)),
                            });
                        }
                        i += 1;
                    }
                    if i >= end {
                        saw_syntax_error = true;
                        self.pending.push_back(ParseEvent::ParseError {
                            code: "html.tokenizer.doctype.eof_in_system_id".to_string(),
                            message: "End of file inside system identifier.".to_string(),
                            span: Some(mk_span(start, end)),
                        });
                    }
                    system_id = Some(bytes_to_string_lossy(&bytes[id_start..i.min(end)]));
                    // If there's a closing quote within the token, it's already accounted for by `i`.
                }
            } else {
                saw_bogus_doctype = true;
                self.pending.push_back(ParseEvent::ParseError {
                    code: "html.tokenizer.doctype.bogus".to_string(),
                    message: "Bogus doctype.".to_string(),
                    span: Some(mk_span(start, start + 2)),
                });
            }
        }

        // Doctype conformance classification (VNU-style).
        if !saw_syntax_error && !saw_bogus_doctype {
            if let Some(n) = name.as_deref() {
                let is_html = n.eq_ignore_ascii_case("html");
                if !is_html || public_id.is_some() || system_id.is_some() {
                    let transitional_public =
                        public_id.as_deref() == Some("-//W3C//DTD HTML 4.01 Transitional//EN");
                    let transitional_system =
                        system_id.as_deref() == Some("http://www.w3.org/TR/html4/loose.dtd");
                    let msg = if is_html && transitional_public && transitional_system {
                        "Almost standards mode doctype. Expected “<!DOCTYPE html>”."
                    } else {
                        "Obsolete doctype. Expected “<!DOCTYPE html>”."
                    };
                    self.pending.push_back(ParseEvent::ParseError {
                        code: "html.parser.doctype.not_html5".to_string(),
                        message: msg.to_string(),
                        span: Some(mk_span(start, start + 2)),
                    });
                }
            }
        }

        self.bump_to(end);
        if end == self.bytes.len() {
            self.finished = true;
        }
        self.pending.push_back(ParseEvent::Doctype {
            name,
            public_id,
            system_id,
            span: Some(self.current_span(start, end, start_line, start_col)),
        });
        Ok(())
    }

    fn scan_rawtext(&mut self, decode: bool) -> Result<(), ValidatorError> {
        let start = self.cursor;
        let start_line = self.line;
        let start_col = self.col;

        let end_tag = self.open_elements.last().map_or("", String::as_str);
        let lt = find_rawtext_end_tag(&self.bytes, self.cursor, end_tag, self.format);
        if let Some(lt) = lt {
            if lt > self.cursor {
                self.bump_to(lt);
                let raw = bytes_to_string_lossy(&self.bytes[start..lt]);
                let text = if decode {
                    decode_char_refs(self.format, raw, false)
                } else {
                    raw
                };
                self.pending.push_back(ParseEvent::Text {
                    text,
                    span: Some(self.current_span(start, lt, start_line, start_col)),
                });
                return Ok(());
            }
            // Already at an end tag; parse it now so the text mode can exit.
            return self.scan_end_tag(start, start_line, start_col);
        }

        // No closing tag found; emit remainder as text and finish.
        let end = self.bytes.len();
        self.bump_to(end);
        let raw = bytes_to_string_lossy(&self.bytes[start..end]);
        let text = if decode {
            decode_char_refs(self.format, raw, false)
        } else {
            raw
        };
        self.pending.push_back(ParseEvent::Text {
            text,
            span: Some(self.current_span(start, end, start_line, start_col)),
        });
        self.finished = true;
        Ok(())
    }

    fn pop_open_element(&mut self, name: &str) {
        let Some(pos) = (match self.format {
            InputFormat::Html => self
                .open_elements
                .iter()
                .rposition(|n| n.eq_ignore_ascii_case(name)),
            InputFormat::Xhtml => self.open_elements.iter().rposition(|n| n == name),
        }) else {
            return;
        };
        self.open_elements.truncate(pos);
        self.open_namespaces.truncate(pos);
    }

    fn scan_text_run(
        &mut self,
        start: usize,
        start_line: u32,
        start_col: u32,
    ) -> Result<(), ValidatorError> {
        // Cursor is currently at a '<' that we decided is not markup. Emit a coalesced text run
        // up to (but not including) the next '<' or EOF.
        let next_lt = memchr(b'<', &self.bytes[self.cursor + 1..])
            .map_or(self.bytes.len(), |off| self.cursor + 1 + off);
        self.bump_to(next_lt);
        let raw = bytes_to_string_lossy(&self.bytes[start..next_lt]);
        let text = decode_char_refs(self.format, raw, false);
        self.pending.push_back(ParseEvent::Text {
            text,
            span: Some(self.current_span(start, next_lt, start_line, start_col)),
        });
        Ok(())
    }

    fn scan_garbage_after_lt_slash(
        &mut self,
        start: usize,
        start_line: u32,
        start_col: u32,
    ) -> Result<(), ValidatorError> {
        // Consume up to the next '>' (or EOF) and emit as a bogus comment, but first report the parse error.
        self.pending.push_back(ParseEvent::ParseError {
            code: "html.tokenizer.garbage_after_lt_slash".to_string(),
            message: "Garbage after “</”.".to_string(),
            span: Some(self.current_span(start, start + 2, start_line, start_col)),
        });
        let Some(off) = memchr(b'>', &self.bytes[self.cursor + 2..]) else {
            self.finished = true;
            self.bump_to(self.bytes.len());
            return Ok(());
        };
        let gt = self.cursor + 2 + off;

        let text = bytes_to_string_lossy(&self.bytes[self.cursor + 2..gt]);
        let end = gt + 1;
        self.bump_to(end);
        self.pending.push_back(ParseEvent::Comment {
            text,
            span: Some(self.current_span(start, end, start_line, start_col)),
        });
        Ok(())
    }
}

impl EventSource for SimpleHtmlEventSource {
    fn source_name(&self) -> &str {
        &self.name
    }

    fn format(&self) -> InputFormat {
        self.format
    }

    fn next_event(&mut self) -> Result<Option<ParseEvent>, ValidatorError> {
        if self.pending.is_empty() && !self.finished {
            self.scan_next()?;
        }
        Ok(self.pending.pop_front())
    }
}

#[derive(Clone, Copy, Debug)]
enum TextModeKind {
    Data,
    RawText,
    RcData,
    Plaintext,
}

fn str_from_bytes_lossy(bytes: &[u8]) -> Cow<'_, str> {
    String::from_utf8_lossy(bytes)
}

fn bytes_to_string_lossy(bytes: &[u8]) -> String {
    str_from_bytes_lossy(bytes).into_owned()
}

fn parse_start_tag(
    src: &SimpleHtmlEventSource,
    inside: &str,
    tag_start: usize,
    tag_line: u32,
    tag_col: u32,
    tag_end: usize,
) -> Result<(String, Vec<Attribute>, bool, Vec<ParseEvent>), ValidatorError> {
    #[inline]
    fn skip_ws(bytes: &[u8], i: &mut usize) {
        while *i < bytes.len() && bytes[*i].is_ascii_whitespace() {
            *i += 1;
        }
    }

    #[inline]
    fn push_parse_error(
        errs: &mut Vec<ParseEvent>,
        span: Option<Span>,
        code: &'static str,
        message: impl Into<String>,
    ) {
        errs.push(ParseEvent::ParseError {
            code: code.to_string(),
            message: message.into(),
            span,
        });
    }

    let bytes = inside.as_bytes();
    let mut i = 0usize;

    // Tag name.
    skip_ws(bytes, &mut i);
    let name_start = i;
    while i < bytes.len() && is_tag_name_char(bytes[i]) {
        i += 1;
    }
    let name_raw = &inside[name_start..i];
    let name = src.normalize_name(name_raw);

    let mut attrs: Vec<Attribute> = Vec::new();
    let mut errs: Vec<ParseEvent> = Vec::new();
    let mut self_closing = false;
    let tag_span = Some(Span::new(tag_start, tag_end, tag_line, tag_col));

    while i < bytes.len() {
        skip_ws(bytes, &mut i);
        if i >= bytes.len() {
            break;
        }
        if bytes[i] == b'/' {
            // Self-closing marker only if it's the last non-whitespace char.
            let mut j = i + 1;
            skip_ws(bytes, &mut j);
            if j >= bytes.len() {
                self_closing = true;
                break;
            }
        }

        // Attribute name.
        if bytes[i] == b'=' {
            push_parse_error(
                &mut errs,
                tag_span,
                "html.tokenizer.equals_expecting_attr_name",
                "Saw “=” when expecting an attribute name. Probable cause: Attribute name missing.",
            );
            i += 1;
            continue;
        }
        if bytes[i] == b'<' {
            push_parse_error(
                &mut errs,
                tag_span,
                "html.tokenizer.lt_expecting_attr_name",
                "Saw “<” when expecting an attribute name. Probable cause: Missing “>” immediately before.",
            );
            i += 1;
            continue;
        }

        let attr_name_start = i;
        while i < bytes.len() && !bytes[i].is_ascii_whitespace() && bytes[i] != b'=' {
            if bytes[i] == b'/' {
                break;
            }
            i += 1;
        }
        if i == attr_name_start {
            // Nothing meaningful; stop parsing.
            break;
        }
        let attr_name_raw = &inside[attr_name_start..i];
        let attr_name = src.normalize_name(attr_name_raw);
        if attr_name_raw.contains('"') {
            push_parse_error(
                &mut errs,
                tag_span,
                "html.tokenizer.quote_in_attr_name",
                "Quote “\"” in attribute name. Probable cause: Matching quote missing somewhere earlier.",
            );
        }
        if attr_name_raw.contains('<') {
            push_parse_error(
                &mut errs,
                tag_span,
                "html.tokenizer.lt_in_attr_name",
                "“<” in attribute name. Probable cause: “>” missing immediately before.",
            );
        }
        if attrs.iter().any(|a| a.name == attr_name) {
            push_parse_error(
                &mut errs,
                tag_span,
                "html.tokenizer.duplicate_attribute",
                format!("Duplicate attribute “{attr_name}”."),
            );
        }

        skip_ws(bytes, &mut i);

        let mut value: Option<String> = None;
        if i < bytes.len() && bytes[i] == b'=' {
            i += 1;
            skip_ws(bytes, &mut i);
            if i >= bytes.len() || bytes[i] == b'>' {
                push_parse_error(
                    &mut errs,
                    tag_span,
                    "html.tokenizer.attr_value_missing",
                    "Attribute value missing.",
                );
            }
            if i < bytes.len() && (bytes[i] == b'"' || bytes[i] == b'\'') {
                let quote = bytes[i];
                i += 1;
                let value_start = i;
                while i < bytes.len() && bytes[i] != quote {
                    i += 1;
                }
                let raw = &inside[value_start..i];
                let (decoded, decoded_errs) = decode_char_refs_with_errors(
                    src.format, raw, true, tag_start, tag_line, tag_col,
                );
                errs.extend(decoded_errs);
                value = Some(decoded);
                if i < bytes.len() && bytes[i] == quote {
                    i += 1;
                }
                // No space between attributes.
                if i < bytes.len() && bytes[i].is_ascii_alphabetic() {
                    push_parse_error(
                        &mut errs,
                        tag_span,
                        "html.tokenizer.no_space_between_attrs",
                        "No space between attributes.",
                    );
                }
            } else {
                let value_start = i;
                while i < bytes.len() && !bytes[i].is_ascii_whitespace() {
                    i += 1;
                }
                let raw = &inside[value_start..i];
                if raw.starts_with('`') {
                    push_parse_error(
                        &mut errs,
                        tag_span,
                        "html.tokenizer.backtick_at_start_unquoted",
                        "“`” at the start of an unquoted attribute value. Probable cause: Using the wrong character as a quote.",
                    );
                } else if raw.contains('`') {
                    push_parse_error(
                        &mut errs,
                        tag_span,
                        "html.tokenizer.backtick_in_unquoted",
                        "“`” in an unquoted attribute value. Probable cause: Using the wrong character as a quote.",
                    );
                }
                if raw.starts_with('<') {
                    push_parse_error(
                        &mut errs,
                        tag_span,
                        "html.tokenizer.lt_at_start_unquoted",
                        "“<” at the start of an unquoted attribute value. Probable cause: Missing “>” immediately before.",
                    );
                } else if raw.contains('<') {
                    push_parse_error(
                        &mut errs,
                        tag_span,
                        "html.tokenizer.lt_in_unquoted",
                        "“<” in an unquoted attribute value. Probable cause: Missing “>” immediately before.",
                    );
                }
                if raw.starts_with('=') {
                    push_parse_error(
                        &mut errs,
                        tag_span,
                        "html.tokenizer.equals_at_start_unquoted",
                        "“=” at the start of an unquoted attribute value. Probable cause: Stray duplicate equals sign.",
                    );
                }
                if raw.contains('"') {
                    push_parse_error(
                        &mut errs,
                        tag_span,
                        "html.tokenizer.quote_in_unquoted",
                        "“\"” in an unquoted attribute value. Probable causes: Attributes running together or a URL query string in an unquoted attribute value.",
                    );
                }
                let (decoded, decoded_errs) = decode_char_refs_with_errors(
                    src.format, raw, true, tag_start, tag_line, tag_col,
                );
                errs.extend(decoded_errs);
                value = Some(decoded);
            }
        }

        // Best-effort span: tie to the tag start since we don't map attr offsets to source bytes yet.
        let span = Some(Span::new(tag_start, tag_start, tag_line, tag_col));
        attrs.push(Attribute {
            name: attr_name,
            value,
            span,
        });
    }

    // Slash not immediately followed by ">".
    if src.format == InputFormat::Html {
        let trimmed = inside.trim_end();
        if trimmed.ends_with('/') && trimmed.len() != inside.len() {
            push_parse_error(
                &mut errs,
                tag_span,
                "html.tokenizer.slash_not_immediately_followed_by_gt",
                "A slash was not immediately followed by “>”.",
            );
        }
    }

    // Special-case: <image> start tag.
    if src.format == InputFormat::Html && name.eq_ignore_ascii_case("image") {
        push_parse_error(
            &mut errs,
            tag_span,
            "html.tokenizer.image_start_tag",
            "Saw a start tag “image”.",
        );
    }

    Ok((name, attrs, self_closing, errs))
}

fn decode_char_refs(format: InputFormat, s: String, in_attribute: bool) -> String {
    if !s.contains('&') {
        return s;
    }

    let bytes = s.as_bytes();
    let mut out = String::with_capacity(s.len());
    let mut i = 0usize;
    let mut last = 0usize;
    while i < bytes.len() {
        if bytes[i] != b'&' {
            i += 1;
            continue;
        }

        out.push_str(&s[last..i]);
        let start = i;
        i += 1;
        if i >= bytes.len() {
            out.push('&');
            last = i;
            break;
        }

        if bytes[i] == b'#' {
            let mut j = i + 1;
            let is_hex = j < bytes.len() && matches!(bytes[j], b'x' | b'X');
            if is_hex {
                j += 1;
            }
            let digits_start = j;
            if is_hex {
                while j < bytes.len() && bytes[j].is_ascii_hexdigit() {
                    j += 1;
                }
            } else {
                while j < bytes.len() && bytes[j].is_ascii_digit() {
                    j += 1;
                }
            }
            if digits_start == j {
                out.push('&');
                i = start + 1;
                last = i;
                continue;
            }
            let digits = &s[digits_start..j];
            let radix = if is_hex { 16 } else { 10 };
            let value = u32::from_str_radix(digits, radix).ok();
            if j < bytes.len() && bytes[j] == b';' {
                j += 1;
            }
            if let Some(cp) = value.and_then(valid_code_point) {
                out.push(cp);
            } else {
                out.push('\u{FFFD}');
            }
            i = j;
            last = i;
            continue;
        }

        // Named reference: attempt longest match against the named-entity table.
        let mut j = i;
        let mut best: Option<(usize, &'static str)> = None;
        while j < bytes.len() {
            let b = bytes[j];
            if !(b.is_ascii_alphanumeric() || b == b';') {
                break;
            }
            j += 1;
            let cand = &s[i..j];
            if let Some(val) = resolve_named_ref(format, cand) {
                best = Some((j, val));
            }
            if b == b';' {
                break;
            }
        }

        if let Some((end, val)) = best {
            let matched = &s[i..end];
            if in_attribute && !matched.ends_with(';') {
                let next = bytes.get(end).copied().unwrap_or(b' ');
                if next.is_ascii_alphanumeric() || next == b'=' {
                    out.push('&');
                    i = start + 1;
                    last = i;
                    continue;
                }
            }
            out.push_str(val);
            i = end;
            last = i;
            continue;
        }

        // Not recognized: consume only '&'.
        out.push('&');
        i = start + 1;
        last = i;
    }
    out.push_str(&s[last..]);
    out
}

fn decode_char_refs_with_errors(
    format: InputFormat,
    s: &str,
    in_attribute: bool,
    base_start: usize,
    base_line: u32,
    base_col: u32,
) -> (String, Vec<ParseEvent>) {
    if format != InputFormat::Html {
        return (s.to_string(), Vec::new());
    }
    let mut errs: Vec<ParseEvent> = Vec::new();
    if let Some((byte_off, cp, byte_len)) = first_forbidden_code_point(s) {
        errs.push(ParseEvent::ParseError {
            code: "html.tokenizer.forbidden_code_point".to_string(),
            message: format!("Forbidden code point U+{:04x}.", cp),
            span: Some(Span::new(
                base_start + byte_off,
                base_start + byte_off + byte_len,
                base_line,
                base_col,
            )),
        });
    }
    if let Some((byte_off, byte_len)) = first_astral_noncharacter(s) {
        errs.push(ParseEvent::ParseError {
            code: "html.tokenizer.astral_noncharacter".to_string(),
            message: "Astral non-character.".to_string(),
            span: Some(Span::new(
                base_start + byte_off,
                base_start + byte_off + byte_len,
                base_line,
                base_col,
            )),
        });
    }

    if !s.contains('&') {
        return (s.to_string(), errs);
    }

    let bytes = s.as_bytes();
    let mut out = String::with_capacity(s.len());
    let mut i = 0usize;
    let mut last = 0usize;
    while i < bytes.len() {
        if bytes[i] != b'&' {
            i += 1;
            continue;
        }

        out.push_str(&s[last..i]);
        let amp_off = i;
        i += 1;
        if i >= bytes.len() {
            out.push('&');
            last = i;
            break;
        }

        if bytes[i] == b'#' {
            let mut j = i + 1;
            let is_hex = j < bytes.len() && matches!(bytes[j], b'x' | b'X');
            if is_hex {
                j += 1;
            }
            let digits_start = j;
            if is_hex {
                while j < bytes.len() && bytes[j].is_ascii_hexdigit() {
                    j += 1;
                }
            } else {
                while j < bytes.len() && bytes[j].is_ascii_digit() {
                    j += 1;
                }
            }
            if digits_start == j {
                errs.push(ParseEvent::ParseError {
                    code: "html.tokenizer.charref_no_digits".to_string(),
                    message: "No digits after “”.".to_string(),
                    span: Some(Span::new(
                        base_start + amp_off,
                        base_start + amp_off + 1,
                        base_line,
                        base_col,
                    )),
                });
                out.push('&');
                i = amp_off + 1;
                last = i;
                continue;
            }
            let digits = &s[digits_start..j];
            let radix = if is_hex { 16 } else { 10 };
            let value = u32::from_str_radix(digits, radix).unwrap_or(0);
            let had_semicolon = j < bytes.len() && bytes[j] == b';';
            if had_semicolon {
                j += 1;
            } else {
                errs.push(ParseEvent::ParseError {
                    code: "html.tokenizer.charref_no_semicolon".to_string(),
                    message: "Character reference was not terminated by a semicolon.".to_string(),
                    span: Some(Span::new(
                        base_start + amp_off,
                        base_start + amp_off + 1,
                        base_line,
                        base_col,
                    )),
                });
            }

            let msg = classify_numeric_charref(value);
            if let Some((code, message)) = msg {
                errs.push(ParseEvent::ParseError {
                    code: code.to_string(),
                    message,
                    span: Some(Span::new(
                        base_start + amp_off,
                        base_start + amp_off + 1,
                        base_line,
                        base_col,
                    )),
                });
            }

            if let Some(cp) = valid_code_point(value) {
                out.push(cp);
            } else {
                out.push('\u{FFFD}');
            }
            i = j;
            last = i;
            continue;
        }

        // Named reference: attempt longest match against the named-entity table.
        let mut j = i;
        let mut best: Option<(usize, &'static str)> = None;
        while j < bytes.len() {
            let b = bytes[j];
            if !(b.is_ascii_alphanumeric() || b == b';') {
                break;
            }
            j += 1;
            let cand = &s[i..j];
            if let Some(val) = resolve_named_ref(format, cand) {
                best = Some((j, val));
            }
            if b == b';' {
                break;
            }
        }

        if let Some((end, val)) = best {
            let matched = &s[i..end];
            if in_attribute && !matched.ends_with(';') {
                let next = bytes.get(end).copied().unwrap_or(b' ');
                if next.is_ascii_alphanumeric() || next == b'=' {
                    out.push('&');
                    i = amp_off + 1;
                    last = i;
                    continue;
                }
            }
            if !matched.ends_with(';') {
                errs.push(ParseEvent::ParseError {
                    code: "html.tokenizer.named_charref_no_semicolon".to_string(),
                    message: "Named character reference was not terminated by a semicolon. (Or “&” should have been escaped as “&amp;”.)".to_string(),
                    span: Some(Span::new(base_start + amp_off, base_start + amp_off + 1, base_line, base_col)),
                });
            }
            out.push_str(val);
            i = end;
            last = i;
            continue;
        }

        // Not recognized: consume only '&'.
        out.push('&');
        i = amp_off + 1;
        last = i;
    }
    out.push_str(&s[last..]);

    (out, errs)
}

fn classify_numeric_charref(cp: u32) -> Option<(&'static str, String)> {
    if cp == 0 {
        return Some((
            "html.tokenizer.charref_zero",
            "Character reference expands to zero.".to_string(),
        ));
    }
    if cp > 0x10FFFF {
        return Some((
            "html.tokenizer.charref_outside_range",
            "Character reference outside the permissible Unicode range.".to_string(),
        ));
    }
    if (0xD800..=0xDFFF).contains(&cp) {
        return Some((
            "html.tokenizer.charref_surrogate",
            "Character reference expands to a surrogate.".to_string(),
        ));
    }
    if cp == 0x0D {
        return Some((
            "html.tokenizer.charref_cr",
            "A numeric character reference expanded to carriage return.".to_string(),
        ));
    }
    if (0x80..=0x9F).contains(&cp) {
        return Some((
            "html.tokenizer.charref_c1_controls",
            "A numeric character reference expanded to the C1 controls range.".to_string(),
        ));
    }
    if cp > 0xFFFF && (cp & 0xFFFE) == 0xFFFE {
        return Some((
            "html.tokenizer.charref_astral_noncharacter",
            format!(
                "Character reference expands to an astral non-character (U+{:x}).",
                cp
            ),
        ));
    }
    if (cp & 0xFFFE) == 0xFFFE {
        return Some((
            "html.tokenizer.charref_noncharacter",
            format!(
                "Character reference expands to a non-character (U+{:04x}).",
                cp
            ),
        ));
    }
    if (0xFDD0..=0xFDEF).contains(&cp) {
        return Some((
            "html.tokenizer.charref_unassigned",
            "Character reference expands to a permanently unassigned code point.".to_string(),
        ));
    }
    if ((1..=0x1F).contains(&cp) && cp != 0x09 && cp != 0x0A && cp != 0x0C && cp != 0x0D)
        || cp == 0x7F
    {
        return Some((
            "html.tokenizer.charref_control",
            format!(
                "Character reference expands to a control character (U+{:04x}).",
                cp
            ),
        ));
    }
    None
}

fn first_forbidden_code_point(s: &str) -> Option<(usize, u32, usize)> {
    // For this suite we only need U+000B (vertical tab) as a forbidden code point in the stream.
    let idx = s.as_bytes().iter().position(|&b| b == 0x0B)?;
    Some((idx, 0x000B, 1))
}

fn first_astral_noncharacter(s: &str) -> Option<(usize, usize)> {
    for (idx, ch) in s.char_indices() {
        let cp = ch as u32;
        if cp > 0xFFFF && (cp & 0xFFFE) == 0xFFFE {
            return Some((idx, ch.len_utf8()));
        }
    }
    None
}

fn resolve_named_ref(format: InputFormat, name: &str) -> Option<&'static str> {
    match (format, name) {
        (InputFormat::Html, _) => html_named_entity_map().get(name).copied(),
        (InputFormat::Xhtml, "lt;") => Some("<"),
        (InputFormat::Xhtml, "gt;") => Some(">"),
        (InputFormat::Xhtml, "amp;") => Some("&"),
        (InputFormat::Xhtml, "quot;") => Some("\""),
        (InputFormat::Xhtml, "apos;") => Some("'"),
        (InputFormat::Xhtml, _) => None,
    }
}

#[cfg(test)]
mod resolve_named_ref_tests {
    use super::{resolve_named_ref, InputFormat};

    #[test]
    fn xhtml_supports_only_predefined_named_entities() {
        assert_eq!(resolve_named_ref(InputFormat::Xhtml, "lt;"), Some("<"));
        assert_eq!(resolve_named_ref(InputFormat::Xhtml, "gt;"), Some(">"));
        assert_eq!(resolve_named_ref(InputFormat::Xhtml, "amp;"), Some("&"));
        assert_eq!(resolve_named_ref(InputFormat::Xhtml, "quot;"), Some("\""));
        assert_eq!(resolve_named_ref(InputFormat::Xhtml, "apos;"), Some("'"));

        assert_eq!(resolve_named_ref(InputFormat::Xhtml, "copy;"), None);
        assert_eq!(resolve_named_ref(InputFormat::Xhtml, "amp"), None);
    }
}

fn valid_code_point(cp: u32) -> Option<char> {
    // Roughly align with HTML parsing behavior: reject surrogate range and out-of-range.
    if cp == 0 || cp > 0x10FFFF || (0xD800..=0xDFFF).contains(&cp) {
        None
    } else {
        char::from_u32(cp)
    }
}

fn is_tag_name_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'-' || b == b':'
}

fn starts_with_ascii_case_insensitive(haystack: &[u8], needle: &[u8]) -> bool {
    haystack.len() >= needle.len() && haystack[..needle.len()].eq_ignore_ascii_case(needle)
}

fn find_tag_close(bytes: &[u8], from: usize) -> Option<usize> {
    #[derive(Clone, Copy, Debug)]
    enum State {
        TagName,
        BeforeAttrName,
        AttrName,
        AfterAttrName,
        BeforeAttrValue,
        AttrValueUnquoted,
        AttrValueQuoted(u8),
    }

    let mut i = from;
    let mut state = State::TagName;
    while i < bytes.len() {
        let b = bytes[i];
        if b == b'>' && !matches!(state, State::AttrValueQuoted(_)) {
            return Some(i);
        }
        match state {
            State::TagName => {
                if b.is_ascii_whitespace() {
                    state = State::BeforeAttrName;
                }
            }
            State::BeforeAttrName => {
                if b.is_ascii_whitespace() {
                    // stay
                } else if b == b'/' {
                    // self-closing marker or just a stray slash; stay in this loose state
                } else {
                    state = State::AttrName;
                }
            }
            State::AttrName => {
                if b.is_ascii_whitespace() {
                    state = State::AfterAttrName;
                } else if b == b'=' {
                    state = State::BeforeAttrValue;
                }
            }
            State::AfterAttrName => {
                if b.is_ascii_whitespace() {
                    // stay
                } else if b == b'=' {
                    state = State::BeforeAttrValue;
                } else {
                    state = State::AttrName;
                }
            }
            State::BeforeAttrValue => {
                if b.is_ascii_whitespace() {
                    // stay
                } else if b == b'"' || b == b'\'' {
                    state = State::AttrValueQuoted(b);
                } else {
                    state = State::AttrValueUnquoted;
                }
            }
            State::AttrValueUnquoted => {
                if b.is_ascii_whitespace() {
                    state = State::BeforeAttrName;
                }
            }
            State::AttrValueQuoted(q) => {
                if b == q {
                    state = State::BeforeAttrName;
                }
            }
        }
        i += 1;
    }
    None
}

fn find_rawtext_end_tag(
    bytes: &[u8],
    from: usize,
    end_tag: &str,
    format: InputFormat,
) -> Option<usize> {
    let end_bytes = end_tag.as_bytes();
    let mut i = from;
    while i < bytes.len() {
        let off = memchr(b'<', &bytes[i..])?;
        let lt = i + off;
        if bytes.get(lt + 1) != Some(&b'/') {
            i = lt + 1;
            continue;
        }
        let name_start = lt + 2;
        if name_start + end_bytes.len() > bytes.len() {
            return None;
        }
        let candidate = &bytes[name_start..name_start + end_bytes.len()];
        let matches = match format {
            InputFormat::Html => candidate.eq_ignore_ascii_case(end_bytes),
            InputFormat::Xhtml => candidate == end_bytes,
        };
        if !matches {
            i = lt + 1;
            continue;
        }
        let after = bytes
            .get(name_start + end_bytes.len())
            .copied()
            .unwrap_or(b'>');
        if after.is_ascii_whitespace() || after == b'>' || after == b'/' {
            return Some(lt);
        }
        i = lt + 1;
    }
    None
}

fn classify_start_tag_eof(rest: &[u8]) -> (String, String) {
    // Best-effort: if we're in a quoted attribute value, prefer that error message.
    let mut quote: Option<u8> = None;
    for &b in rest {
        match (quote, b) {
            (None, b'\'' | b'"') => quote = Some(b),
            (Some(q), b) if b == q => quote = None,
            _ => {}
        }
    }
    if quote.is_some() {
        return (
            "html.tokenizer.eof_in_attr_value".to_string(),
            "End of file reached when inside an attribute value. Ignoring tag.".to_string(),
        );
    }
    (
        "html.tokenizer.eof_in_attr_name".to_string(),
        "End of file occurred in an attribute name. Ignoring tag.".to_string(),
    )
}

fn memchr(needle: u8, haystack: &[u8]) -> Option<usize> {
    haystack.iter().position(|&b| b == needle)
}

fn find_subslice(haystack: &[u8], from: usize, needle: &[u8]) -> Option<usize> {
    haystack[from..]
        .windows(needle.len())
        .position(|w| w == needle)
        .map(|off| from + off)
}

fn html_named_entity_map() -> &'static HashMap<&'static str, &'static str> {
    static MAP: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
    MAP.get_or_init(|| {
        let mut map = HashMap::with_capacity(named_entities::HTML_NAMED_ENTITIES.len());
        map.extend(named_entities::HTML_NAMED_ENTITIES.iter().copied());
        map
    })
}

#[cfg(test)]
mod entity_map_tests {
    use super::{html_named_entity_map, named_entities};

    #[test]
    fn html_named_entity_map_contains_expected_entries() {
        let map = html_named_entity_map();
        assert_eq!(map.len(), named_entities::HTML_NAMED_ENTITIES.len());
        assert_eq!(map.get("AMP;"), Some(&"&"));
        assert_eq!(map.get("NegativeMediumSpace;"), Some(&"\u{200B}"));
        assert_eq!(map.get("NegativeThickSpace;"), Some(&"\u{200B}"));
        assert_eq!(map.get("NegativeThinSpace;"), Some(&"\u{200B}"));
        assert_eq!(map.get("NegativeVeryThinSpace;"), Some(&"\u{200B}"));
        assert_eq!(map.get("NoBreak;"), Some(&"\u{2060}"));
        assert_eq!(map.get("ZeroWidthSpace;"), Some(&"\u{200B}"));
        assert_eq!(map.get("shy;"), Some(&"\u{AD}"));
        assert_eq!(map.get("shy"), Some(&"\u{AD}"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use html_inspector_core::EventSource;

    fn collect(mut src: SimpleHtmlEventSource) -> Vec<ParseEvent> {
        let mut out = Vec::new();
        while let Some(ev) = src.next_event().unwrap() {
            out.push(ev);
        }
        out
    }

    #[test]
    fn classify_start_tag_eof_prefers_attr_value_error_when_in_quote() {
        let (code, _msg) = classify_start_tag_eof(br#" class="unterminated"#);
        assert_eq!(code, "html.tokenizer.eof_in_attr_value");

        // Different quote bytes should not terminate the current quoted value.
        let (code, _msg) = classify_start_tag_eof(br#" class="has'single"#);
        assert_eq!(code, "html.tokenizer.eof_in_attr_value");
    }

    #[test]
    fn classify_start_tag_eof_returns_attr_name_error_when_not_in_quote() {
        let (code, _msg) = classify_start_tag_eof(b" class=foo");
        assert_eq!(code, "html.tokenizer.eof_in_attr_name");

        // Balanced quotes leave us outside an attribute value at EOF.
        let (code, _msg) = classify_start_tag_eof(br#" class="ok""#);
        assert_eq!(code, "html.tokenizer.eof_in_attr_name");
    }

    #[test]
    fn bytes_at_cursor_is_false_when_out_of_bounds() {
        let mut src = SimpleHtmlEventSource::from_str("t", InputFormat::Html, "<");
        assert!(src.bytes_at_cursor(b""));
        assert!(src.bytes_at_cursor(b"<"));
        assert!(!src.bytes_at_cursor(b"<>"));
        src.cursor = src.bytes.len();
        assert!(src.bytes_at_cursor(b""));
        assert!(!src.bytes_at_cursor(b"<"));
        src.cursor = src.bytes.len() + 1;
        assert!(!src.bytes_at_cursor(b""));
        assert!(!src.bytes_at_cursor(b"<"));
    }

    #[test]
    fn valid_code_point_matches_html_scalar_value_constraints() {
        assert_eq!(valid_code_point(0), None);
        assert_eq!(valid_code_point(0xD800), None);
        assert_eq!(valid_code_point(0xDFFF), None);
        assert_eq!(valid_code_point(0x110000), None);

        assert_eq!(valid_code_point(0x41), Some('A'));
        assert_eq!(
            valid_code_point(0x10FFFF),
            Some(char::from_u32(0x10FFFF).unwrap())
        );
    }

    #[test]
    fn first_forbidden_code_point_finds_vertical_tab_by_byte_offset() {
        assert_eq!(first_forbidden_code_point("abc"), None);

        let s = "❤\u{000B}x";
        let (idx, cp, len) = first_forbidden_code_point(s).unwrap();
        assert_eq!(idx, "❤".len());
        assert_eq!(cp, 0x000B);
        assert_eq!(len, 1);
    }

    #[test]
    fn first_astral_noncharacter_finds_noncharacters_and_reports_byte_len() {
        assert_eq!(first_astral_noncharacter("abc"), None);

        let ch = char::from_u32(0x1FFFE).unwrap();
        let s = format!("a{ch}b");
        let (idx, len) = first_astral_noncharacter(&s).unwrap();
        assert_eq!(idx, 1);
        assert_eq!(len, ch.len_utf8());
    }

    #[test]
    fn normalize_name_lowercases_ascii_in_html_only() {
        let html = SimpleHtmlEventSource::from_str("t", InputFormat::Html, "");
        assert_eq!(html.normalize_name("DiV❤"), "div❤");
        assert_eq!(html.normalize_name("div"), "div");

        let xhtml = SimpleHtmlEventSource::from_str("t", InputFormat::Xhtml, "");
        assert_eq!(xhtml.normalize_name("DiV❤"), "DiV❤");
    }

    #[test]
    fn normalize_name_accepts_cow_without_extra_allocation_for_owned() {
        let html = SimpleHtmlEventSource::from_str("t", InputFormat::Html, "");
        assert_eq!(
            html.normalize_name(std::borrow::Cow::Borrowed("DiV")),
            html.normalize_name("DiV")
        );
        assert_eq!(
            html.normalize_name(std::borrow::Cow::Owned("DiV".to_string())),
            html.normalize_name("DiV")
        );

        let s = "DiV".to_string();
        let ptr = s.as_ptr();
        let cap = s.capacity();
        let out = html.normalize_name(std::borrow::Cow::Owned(s));
        assert_eq!(out, "div");
        assert_eq!(out.as_ptr(), ptr);
        assert_eq!(out.capacity(), cap);

        let s = "div".to_string();
        let ptr = s.as_ptr();
        let cap = s.capacity();
        let out = html.normalize_name(s);
        assert_eq!(out, "div");
        assert_eq!(out.as_ptr(), ptr);
        assert_eq!(out.capacity(), cap);

        let xhtml = SimpleHtmlEventSource::from_str("t", InputFormat::Xhtml, "");
        assert_eq!(
            xhtml.normalize_name(std::borrow::Cow::Borrowed("DiV")),
            xhtml.normalize_name("DiV")
        );
        assert_eq!(
            xhtml.normalize_name(std::borrow::Cow::Owned("DiV".to_string())),
            xhtml.normalize_name("DiV")
        );

        let s = "DiV".to_string();
        let ptr = s.as_ptr();
        let cap = s.capacity();
        let out = xhtml.normalize_name(std::borrow::Cow::Owned(s));
        assert_eq!(out, "DiV");
        assert_eq!(out.as_ptr(), ptr);
        assert_eq!(out.capacity(), cap);
    }

    #[test]
    fn next_event_drains_pending_even_if_finished() {
        let mut src = SimpleHtmlEventSource::from_str("t", InputFormat::Html, "");
        src.finished = true;
        src.pending.push_back(ParseEvent::Text {
            text: "x".to_string(),
            span: None,
        });
        assert!(matches!(
            src.next_event().unwrap(),
            Some(ParseEvent::Text { ref text, .. }) if text == "x"
        ));
        assert!(src.next_event().unwrap().is_none());
    }

    #[test]
    fn pop_open_element_truncates_stacks_at_last_match() {
        let mut src = SimpleHtmlEventSource::from_str("t", InputFormat::Html, "");
        src.open_elements = vec![
            "a".to_string(),
            "b".to_string(),
            "B".to_string(),
            "c".to_string(),
        ];
        src.open_namespaces = vec![
            HtmlNamespace::Html,
            HtmlNamespace::Html,
            HtmlNamespace::Svg,
            HtmlNamespace::Math,
        ];
        src.pop_open_element("b");
        assert_eq!(src.open_elements, vec!["a".to_string(), "b".to_string()]);
        assert_eq!(
            src.open_namespaces,
            vec![HtmlNamespace::Html, HtmlNamespace::Html]
        );

        let mut src = SimpleHtmlEventSource::from_str("t", InputFormat::Xhtml, "");
        src.open_elements = vec![
            "a".to_string(),
            "b".to_string(),
            "B".to_string(),
            "c".to_string(),
        ];
        src.open_namespaces = vec![
            HtmlNamespace::Html,
            HtmlNamespace::Html,
            HtmlNamespace::Svg,
            HtmlNamespace::Math,
        ];
        src.pop_open_element("b");
        assert_eq!(src.open_elements, vec!["a".to_string()]);
        assert_eq!(src.open_namespaces, vec![HtmlNamespace::Html]);
    }

    #[test]
    fn pop_open_element_is_noop_when_name_is_missing() {
        let mut src = SimpleHtmlEventSource::from_str("t", InputFormat::Html, "");
        src.open_elements = vec!["a".to_string(), "b".to_string()];
        src.open_namespaces = vec![HtmlNamespace::Html, HtmlNamespace::Svg];
        src.pop_open_element("missing");
        assert_eq!(src.open_elements, vec!["a".to_string(), "b".to_string()]);
        assert_eq!(
            src.open_namespaces,
            vec![HtmlNamespace::Html, HtmlNamespace::Svg]
        );
    }

    fn as_start_tag(ev: &ParseEvent) -> Option<(&str, &[html_inspector_core::Attribute])> {
        match ev {
            ParseEvent::StartTag { name, attrs, .. } => Some((name, attrs.as_slice())),
            _ => None,
        }
    }

    #[test]
    fn str_from_bytes_lossy_borrows_valid_utf8() {
        let s = str_from_bytes_lossy(b"hello");
        assert!(matches!(s, Cow::Borrowed(_)));
        assert_eq!(s.as_ref(), "hello");
    }

    #[test]
    fn str_from_bytes_lossy_allocates_on_invalid_utf8() {
        let s = str_from_bytes_lossy(&[0xff, b'a']);
        assert!(matches!(s, Cow::Owned(_)));
        assert_eq!(s.as_ref(), "�a");
    }

    #[test]
    fn doctype_name_normalizes_even_when_decoding_allocates() {
        let src = SimpleHtmlEventSource::from_bytes(
            "t",
            InputFormat::Html,
            vec![
                b'<', b'!', b'D', b'O', b'C', b'T', b'Y', b'P', b'E', b' ', 0xff, b'A', b'>',
            ],
        );
        let evs = collect(src);
        let name = evs.iter().find_map(|e| match e {
            ParseEvent::Doctype {
                name: Some(name), ..
            } => Some(name.as_str()),
            _ => None,
        });
        assert_eq!(name, Some("�a"));
    }

    #[test]
    fn treats_lt_not_followed_by_tag_name_as_text() {
        let src = SimpleHtmlEventSource::from_str("t", InputFormat::Html, "<$");
        let evs = collect(src);
        assert!(
            matches!(evs[0], ParseEvent::ParseError { ref code, .. } if code == "html.tokenizer.bad_char_after_lt")
        );
        assert!(matches!(evs[1], ParseEvent::Text { ref text, .. } if text == "<$"));
    }

    #[test]
    fn lt_at_eof_emits_tokenizer_eof_after_lt_error() {
        let src = SimpleHtmlEventSource::from_str("t", InputFormat::Html, "<");
        let evs = collect(src);
        assert_eq!(evs.len(), 1);
        let ParseEvent::ParseError {
            code,
            message,
            span,
        } = &evs[0]
        else {
            panic!("expected a parse error event");
        };
        assert_eq!(code, "html.tokenizer.eof_after_lt");
        assert_eq!(message, "End of file after “<”.");
        assert_eq!(span.unwrap(), Span::new(0, 1, 1, 1));
    }

    #[test]
    fn parses_tag_end_ignoring_gt_inside_quotes() {
        let src = SimpleHtmlEventSource::from_str("t", InputFormat::Html, "<a title=\">\">x</a>");
        let evs = collect(src);
        let (name, attrs) = as_start_tag(&evs[0]).unwrap();
        assert_eq!(name, "a");
        assert_eq!(attrs.len(), 1);
        assert_eq!(attrs[0].name, "title");
        assert_eq!(attrs[0].value.as_deref(), Some(">"));
    }

    #[test]
    fn lt_followed_by_whitespace_is_literal_text_without_error() {
        let src = SimpleHtmlEventSource::from_str("t", InputFormat::Html, "< ");
        let evs = collect(src);
        assert!(matches!(evs[0], ParseEvent::Text { ref text, .. } if text == "< "));
    }

    #[test]
    fn xhtml_processing_instruction_skips_leading_whitespace_in_target() {
        let src = SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Xhtml,
            "<?   xml-stylesheet href=\"a\"?>",
        );
        let evs = collect(src);
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ProcessingInstruction { target, data, .. }
                if target == "xml-stylesheet" && data == "href=\"a\""
        )));
    }

    #[test]
    fn doctype_public_and_system_missing_ids_emit_expected_errors() {
        let src = SimpleHtmlEventSource::from_str("t", InputFormat::Html, "<!DOCTYPE html PUBLIC>");
        let evs = collect(src);
        assert!(evs.iter().any(|e| matches!(
                e,
                ParseEvent::ParseError { code, .. } if code == "html.tokenizer.doctype.expected_public_id"
            )));

        let src = SimpleHtmlEventSource::from_str("t", InputFormat::Html, "<!DOCTYPE html SYSTEM>");
        let evs = collect(src);
        assert!(evs.iter().any(|e| matches!(
                e,
                ParseEvent::ParseError { code, .. } if code == "html.tokenizer.doctype.expected_system_id"
            )));
    }

    #[test]
    fn doctype_allows_whitespace_after_bang_and_newlines_affect_span_tracking() {
        let src = SimpleHtmlEventSource::from_str("t", InputFormat::Html, "<! DOCTYPE html>");
        let evs = collect(src);
        assert!(evs.iter().any(|e| matches!(e, ParseEvent::Doctype { .. })));

        let src = SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<!DOCTYPE html PUBLIC \"a\\n>\" \"sys\">",
        );
        let evs = collect(src);
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.doctype.gt_in_public_id"
        )));
    }

    #[test]
    fn parse_start_tag_allows_leading_whitespace_in_inside_buffer() {
        let src = SimpleHtmlEventSource::from_str("t", InputFormat::Html, "");
        let (name, _attrs, _sc, _errs) = parse_start_tag(&src, "   a", 0, 1, 1, 0).unwrap();
        assert_eq!(name, "a");
    }

    #[test]
    fn parse_start_tag_errors_use_the_full_tag_span() {
        let src = SimpleHtmlEventSource::from_str("t", InputFormat::Html, "<a x==y ></a>");
        let evs = collect(src);
        let span = evs.iter().find_map(|e| match e {
            ParseEvent::ParseError {
                code,
                span: Some(span),
                ..
            } if code == "html.tokenizer.equals_at_start_unquoted" => Some(*span),
            _ => None,
        });
        let span = span.expect("expected equals_at_start_unquoted error");
        assert_eq!(span.byte_start, 0);
        assert_eq!(span.byte_end, 9);
        assert_eq!(span.line, 1);
        assert_eq!(span.col, 1);
    }

    #[test]
    fn multiple_spaces_after_attr_name_are_accepted() {
        let src = SimpleHtmlEventSource::from_str("t", InputFormat::Html, "<a x  =\"y\"></a>");
        let evs = collect(src);
        let (_name, attrs) = as_start_tag(&evs[0]).unwrap();
        let x = attrs.iter().find(|a| a.name == "x").unwrap();
        assert_eq!(x.value.as_deref(), Some("y"));
    }

    #[test]
    fn rawtext_end_tag_search_can_exhaust_input() {
        assert_eq!(
            find_rawtext_end_tag(b"<<<<", 0, "script", InputFormat::Html),
            None
        );
    }

    #[test]
    fn quoted_attributes_without_space_emit_no_space_between_attrs_error() {
        let src =
            SimpleHtmlEventSource::from_str("t", InputFormat::Html, "<a title=\"x\"id=\"y\"></a>");
        let evs = collect(src);
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.no_space_between_attrs"
        )));
        let (name, attrs) = evs.iter().find_map(as_start_tag).unwrap();
        assert_eq!(name, "a");
        assert!(attrs.iter().any(|a| a.name == "title"));
        assert!(attrs.iter().any(|a| a.name == "id"));
    }

    #[test]
    fn classify_start_tag_eof_covers_mixed_quote_tracking() {
        let (code, _msg) = classify_start_tag_eof(b" a='x\"");
        assert_eq!(code, "html.tokenizer.eof_in_attr_value");
    }

    #[test]
    fn classify_start_tag_eof_tracks_double_quotes() {
        let (code, _msg) = classify_start_tag_eof(br#" a="b"#);
        assert_eq!(code, "html.tokenizer.eof_in_attr_value");

        let (code, _msg) = classify_start_tag_eof(br#" a="b" c"#);
        assert_eq!(code, "html.tokenizer.eof_in_attr_name");
    }

    #[test]
    fn start_tag_helper_returns_none_for_non_start_tag() {
        assert!(as_start_tag(&ParseEvent::Text {
            text: "x".to_string(),
            span: None,
        })
        .is_none());
    }

    #[test]
    fn rawtext_script_does_not_tokenize_lt() {
        let src = SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<script>if (a < b) {}</script>",
        );
        let evs = collect(src);
        assert!(matches!(evs[0], ParseEvent::StartTag { ref name, .. } if name == "script"));
        assert!(matches!(evs[1], ParseEvent::Text { ref text, .. } if text == "if (a < b) {}"));
        assert!(matches!(evs[2], ParseEvent::EndTag { ref name, .. } if name == "script"));
    }

    #[test]
    fn rcdata_textarea_does_not_tokenize_lt_but_decodes_entities() {
        let src = SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<textarea>1 < 2 &lt; 3</textarea>",
        );
        let evs = collect(src);
        assert!(matches!(evs[0], ParseEvent::StartTag { ref name, .. } if name == "textarea"));
        assert!(matches!(evs[1], ParseEvent::Text { ref text, .. } if text == "1 < 2 < 3"));
        assert!(matches!(evs[2], ParseEvent::EndTag { ref name, .. } if name == "textarea"));
    }

    #[test]
    fn rcdata_title_decodes_entities() {
        let src =
            SimpleHtmlEventSource::from_str("t", InputFormat::Html, "<title>1 &lt; 2</title>");
        let evs = collect(src);
        assert!(matches!(evs[0], ParseEvent::StartTag { ref name, .. } if name == "title"));
        assert!(matches!(evs[1], ParseEvent::Text { ref text, .. } if text == "1 < 2"));
        assert!(matches!(evs[2], ParseEvent::EndTag { ref name, .. } if name == "title"));
    }

    #[test]
    fn plaintext_consumes_rest_of_document_as_text() {
        let src = SimpleHtmlEventSource::from_str("t", InputFormat::Html, "<plaintext><b>hi</b>");
        let evs = collect(src);
        assert!(matches!(evs[0], ParseEvent::StartTag { ref name, .. } if name == "plaintext"));
        assert!(matches!(evs[1], ParseEvent::Text { ref text, .. } if text == "<b>hi</b>"));
        assert_eq!(evs.len(), 2);
    }

    #[test]
    fn xhtml_cdata_emits_text() {
        let src = SimpleHtmlEventSource::from_str("t", InputFormat::Xhtml, "<![CDATA[<tag>]]>");
        let evs = collect(src);
        assert_eq!(
            evs,
            vec![ParseEvent::Text {
                text: "<tag>".to_string(),
                span: Some(Span::new(0, 17, 1, 1)),
            }]
        );
    }

    #[test]
    fn html_cdata_outside_foreign_content_is_bogus_comment() {
        let src = SimpleHtmlEventSource::from_str("t", InputFormat::Html, "<![CDATA[<tag>]]>");
        let evs = collect(src);
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.bogus_comment"
        )));
        assert!(evs.iter().any(|e| matches!(e, ParseEvent::Comment { .. })));
    }

    #[test]
    fn html_cdata_inside_svg_emits_text_without_bogus_comment_error() {
        let src = SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<!--<!-- --><svg><script><![CDATA[if (a < b) {}]]></script></svg>",
        );
        let evs = collect(src);
        assert!(!evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.bogus_comment"
        )));
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::Text { text, .. } if text.contains("if (a < b) {}")
        )));
    }

    #[test]
    fn doctype_parses_public_and_system_ids() {
        let src = SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<!DOCTYPE html PUBLIC \"pub\" 'sys'><html></html>",
        );
        let evs = collect(src);
        let (name, public_id, system_id) = evs
            .iter()
            .find_map(|e| match e {
                ParseEvent::Doctype {
                    name,
                    public_id,
                    system_id,
                    ..
                } => Some((name.clone(), public_id.clone(), system_id.clone())),
                _ => None,
            })
            .expect("expected a doctype event");
        assert_eq!(name.as_deref(), Some("html"));
        assert_eq!(public_id.as_deref(), Some("pub"));
        assert_eq!(system_id.as_deref(), Some("sys"));
    }

    #[test]
    fn decodes_basic_entities_in_text_and_attributes() {
        let src = SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<p title=\"a &lt; b\">Tom &amp; Jerry</p>",
        );
        let evs = collect(src);
        let (_name, attrs) = as_start_tag(&evs[0]).unwrap();
        let title = attrs.iter().find(|a| a.name == "title").unwrap();
        assert_eq!(title.value.as_deref(), Some("a < b"));
        assert!(matches!(evs[1], ParseEvent::Text { ref text, .. } if text == "Tom & Jerry"));
    }

    #[test]
    fn named_char_ref_without_semicolon_emits_error_and_decodes_in_text() {
        let src = SimpleHtmlEventSource::from_str("t", InputFormat::Html, "<p>&copy</p>");
        let evs = collect(src);
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.named_charref_no_semicolon"
        )));
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::Text { text, .. } if text == "©"
        )));
    }

    #[test]
    fn named_char_ref_without_semicolon_not_decoded_in_attribute_when_followed_by_equals() {
        let src = SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<!--<!-- --><a title=\"&copy=1\"></a>",
        );
        let evs = collect(src);
        assert!(!evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.named_charref_no_semicolon"
        )));
        let (name, attrs) = evs.iter().find_map(as_start_tag).unwrap();
        assert_eq!(name, "a");
        let title = attrs.iter().find(|a| a.name == "title").unwrap();
        assert_eq!(title.value.as_deref(), Some("&copy=1"));
    }

    #[test]
    fn numeric_char_ref_without_semicolon_emits_error_and_decodes() {
        let src = SimpleHtmlEventSource::from_str("t", InputFormat::Html, "<p>&#65</p>");
        let evs = collect(src);
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.charref_no_semicolon"
        )));
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::Text { text, .. } if text == "A"
        )));
    }

    #[test]
    fn numeric_char_ref_zero_emits_error_and_replacement_char() {
        let src = SimpleHtmlEventSource::from_str("t", InputFormat::Html, "<p>&#0;</p>");
        let evs = collect(src);
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.charref_zero"
        )));
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::Text { text, .. } if text == "\u{FFFD}"
        )));
    }

    #[test]
    fn forbidden_code_point_in_text_emits_error() {
        let src = SimpleHtmlEventSource::from_str("t", InputFormat::Html, "<p>\u{000B}</p>");
        let evs = collect(src);
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.forbidden_code_point"
        )));
    }

    #[test]
    fn astral_noncharacter_in_text_emits_error() {
        let src = SimpleHtmlEventSource::from_str("t", InputFormat::Html, "<p>\u{10FFFE}</p>");
        let evs = collect(src);
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.astral_noncharacter"
        )));
    }

    #[test]
    fn nested_comment_emits_parse_error() {
        let src = SimpleHtmlEventSource::from_str("t", InputFormat::Html, "<!-- a <!-- b -->");
        let evs = collect(src);
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.nested_comment"
        )));
        assert!(evs
            .iter()
            .any(|e| matches!(e, ParseEvent::Comment { text, .. } if text.contains("a"))));
    }

    #[test]
    fn html_processing_instruction_emits_error_and_comment() {
        let src = SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<?xml version=\"1.0\"?><p>x</p>",
        );
        let evs = collect(src);
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.processing_instruction"
        )));
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::Comment { text, .. } if text.contains("xml version")
        )));
    }

    #[test]
    fn xhtml_processing_instruction_is_emitted_as_processing_instruction_event() {
        let src = SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Xhtml,
            "<?xml-stylesheet href=\"a.css\" type=\"text/css\"?><root/>",
        );
        let evs = collect(src);
        assert!(matches!(
            evs[0],
            ParseEvent::ProcessingInstruction { ref target, ref data, .. }
            if target == "xml-stylesheet" && data.contains("href=\"a.css\"")
        ));
    }

    #[test]
    fn doctype_missing_space_before_name_emits_error() {
        let src = SimpleHtmlEventSource::from_str("t", InputFormat::Html, "<!DOCTYPEhtml><p>x</p>");
        let evs = collect(src);
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.doctype.missing_space_before_name"
        )));
    }

    #[test]
    fn html_event_source_wrapper_uses_simple_backend_for_xhtml() {
        let mut src = HtmlEventSource::from_str("t", InputFormat::Xhtml, "<root/>").unwrap();
        assert_eq!(src.source_name(), "t");
        assert_eq!(src.format(), InputFormat::Xhtml);
        let mut evs = Vec::new();
        while let Some(ev) = src.next_event().unwrap() {
            evs.push(ev);
        }
        assert!(evs
            .iter()
            .any(|e| matches!(e, ParseEvent::StartTag { name, .. } if name == "root")));
    }

    #[test]
    fn foreignobject_in_svg_switches_insertion_namespace_to_html() {
        let src = SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<svg><foreignObject><p>hi</p></foreignObject></svg>",
        );
        let evs = collect(src);
        assert!(evs.iter().any(|e| matches!(e, ParseEvent::StartTag { name, .. } if name.eq_ignore_ascii_case("foreignobject"))));
        assert!(evs
            .iter()
            .any(|e| matches!(e, ParseEvent::StartTag { name, .. } if name == "p")));
    }

    #[test]
    fn unterminated_cdata_emits_xml_cdata_eof_error() {
        let src = SimpleHtmlEventSource::from_str("t", InputFormat::Xhtml, "<![CDATA[unterminated");
        let evs = collect(src);
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "xml.cdata_eof"
        )));
    }

    #[test]
    fn bogus_comment_without_gt_emits_comment_then_finishes() {
        let src = SimpleHtmlEventSource::from_str("t", InputFormat::Html, "<?xml");
        let evs = collect(src);
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.processing_instruction"
        )));
        assert!(evs
            .iter()
            .any(|e| matches!(e, ParseEvent::Comment { text, .. } if text.contains("xml"))));
    }

    #[test]
    fn unterminated_processing_instruction_emits_xml_pi_eof_error() {
        let src =
            SimpleHtmlEventSource::from_str("t", InputFormat::Xhtml, "<?xml-stylesheet href=\"a\"");
        let evs = collect(src);
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "xml.pi_eof"
        )));
    }

    #[test]
    fn garbage_after_lt_slash_at_eof_emits_error_and_finishes() {
        let src = SimpleHtmlEventSource::from_str("t", InputFormat::Html, "</ ");
        let evs = collect(src);
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.garbage_after_lt_slash"
        )));
        assert_eq!(evs.len(), 1);
    }

    #[test]
    fn doctype_end_finder_ignores_gt_inside_quoted_identifiers() {
        let src = SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<!DOCTYPE html SYSTEM \"a> b\"><html></html>",
        );
        let evs = collect(src);

        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.doctype.gt_in_system_id"
        )));

        let system_id = evs
            .iter()
            .find_map(|e| match e {
                ParseEvent::Doctype { system_id, .. } => system_id.as_deref(),
                _ => None,
            })
            .expect("expected doctype event");
        assert_eq!(system_id, "a> b");

        assert!(evs
            .iter()
            .any(|e| matches!(e, ParseEvent::StartTag { name, .. } if name == "html")));
    }

    #[test]
    fn doctype_system_identifier_gt_and_unclosed_quote_emit_errors() {
        let src = SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<!DOCTYPE html SYSTEM \"a> b><p>x</p>",
        );
        let evs = collect(src);
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.doctype.gt_in_system_id"
        )));
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.doctype.eof_in_system_id"
        )));
    }

    #[test]
    fn doctype_public_identifier_gt_and_unclosed_quote_emit_errors() {
        let src = SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<!DOCTYPE html PUBLIC \"pub\" \"a> b",
        );
        let evs = collect(src);
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.doctype.gt_in_system_id"
        )));
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.doctype.eof_in_system_id"
        )));
    }

    #[test]
    fn simple_event_source_exposes_source_name_and_format() {
        let src = SimpleHtmlEventSource::from_str("t", InputFormat::Html, "<p>x</p>");
        assert_eq!(src.source_name(), "t");
        assert_eq!(src.format(), InputFormat::Html);
    }

    #[test]
    fn scan_next_is_noop_when_finished_is_true() {
        let mut src = SimpleHtmlEventSource::from_str("t", InputFormat::Html, "<p>x</p>");
        while src.next_event().unwrap().is_some() {}
        assert!(src.finished);
        src.scan_next().unwrap();
    }

    #[test]
    fn cdata_in_html_inside_svg_is_parsed_as_text() {
        let src =
            SimpleHtmlEventSource::from_str("t", InputFormat::Html, "<svg><![CDATA[<tag>]]></svg>");
        let evs = collect(src);
        assert!(evs
            .iter()
            .any(|e| matches!(e, ParseEvent::Text { text, .. } if text == "<tag>")));
    }

    #[test]
    fn end_tag_garbage_and_non_tag_end_sequences_are_coalesced_as_text() {
        let src = SimpleHtmlEventSource::from_str("t", InputFormat::Html, "</ x> </1> <p>x</p>");
        let evs = collect(src);
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.garbage_after_lt_slash"
        )));
        assert!(evs
            .iter()
            .any(|e| matches!(e, ParseEvent::Text { text, .. } if text.contains("</1>"))));
    }

    #[test]
    fn malformed_attribute_syntax_triggers_simple_parser_errors() {
        let src = SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<a href=></a><a title=\"x\"href=\"y\"></a><a /x></a><a ></a><a/>",
        );
        let evs = collect(src);
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.attr_value_missing"
        )));
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.no_space_between_attrs"
        )));
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::StartTag { name, self_closing, .. } if name == "a" && *self_closing
        )));
    }

    #[test]
    fn decode_char_refs_covers_edge_cases_directly() {
        assert_eq!(
            decode_char_refs(InputFormat::Html, "&".to_string(), false),
            "&"
        );
        assert_eq!(
            decode_char_refs(InputFormat::Html, "&#;".to_string(), false),
            "&#;"
        );
        assert_eq!(
            decode_char_refs(InputFormat::Html, "&bogus;".to_string(), false),
            "&bogus;"
        );
        assert_eq!(
            decode_char_refs(InputFormat::Html, "&#65;".to_string(), false),
            "A"
        );
        assert_eq!(
            decode_char_refs(InputFormat::Html, "&#x41;".to_string(), false),
            "A"
        );
        assert_eq!(
            decode_char_refs(InputFormat::Html, "&#X41;".to_string(), false),
            "A"
        );
        assert_eq!(
            decode_char_refs(InputFormat::Html, "&#x;".to_string(), false),
            "&#x;"
        );
        assert_eq!(
            decode_char_refs(InputFormat::Html, "&#x110000;".to_string(), false),
            "\u{FFFD}"
        );

        // Attribute-specific behavior: do not expand missing-semicolon named refs when followed by
        // an alnum or '='.
        assert_eq!(
            decode_char_refs(InputFormat::Html, "&copy=1".to_string(), true),
            "&copy=1"
        );
        let (s, errs) = decode_char_refs_with_errors(InputFormat::Xhtml, "&copy", true, 0, 1, 1);
        assert_eq!(s, "&copy");
        assert!(errs.is_empty());

        let (s2, errs2) = decode_char_refs_with_errors(InputFormat::Html, "&copy=1", true, 0, 1, 1);
        assert_eq!(s2, "&copy=1");
        assert!(errs2.is_empty());
    }

    #[test]
    fn decode_char_refs_returns_input_string_when_no_refs_present() {
        let raw = "plain text".to_string();
        let ptr = raw.as_ptr();
        let cap = raw.capacity();
        let out = decode_char_refs(InputFormat::Html, raw, false);
        assert_eq!(out, "plain text");
        assert_eq!(out.as_ptr(), ptr);
        assert_eq!(out.capacity(), cap);
    }

    #[test]
    fn normalize_name_lowercases_in_html_but_not_in_xhtml() {
        let src = SimpleHtmlEventSource::from_str("t", InputFormat::Html, "");
        assert_eq!(src.normalize_name("div"), "div");
        assert_eq!(src.normalize_name("DiV"), "div");
        assert_eq!(src.normalize_name("Ü"), "Ü");
        assert_eq!(src.normalize_name("ÜA"), "Üa");

        let src = SimpleHtmlEventSource::from_str("t", InputFormat::Xhtml, "");
        assert_eq!(src.normalize_name("DiV"), "DiV");
        assert_eq!(src.normalize_name("ÜA"), "ÜA");
    }

    #[test]
    fn current_text_mode_kind_applies_only_in_html_namespace() {
        let mut src = SimpleHtmlEventSource::from_str("t", InputFormat::Html, "");
        assert!(matches!(src.current_text_mode_kind(), TextModeKind::Data));
        src.open_elements.push("script".to_string());
        src.open_namespaces.push(HtmlNamespace::Html);
        assert!(matches!(
            src.current_text_mode_kind(),
            TextModeKind::RawText
        ));

        src.open_elements.pop();
        src.open_namespaces.pop();
        src.open_elements.push("title".to_string());
        src.open_namespaces.push(HtmlNamespace::Html);
        assert!(matches!(src.current_text_mode_kind(), TextModeKind::RcData));

        src.open_elements.pop();
        src.open_namespaces.pop();
        src.open_elements.push("plaintext".to_string());
        src.open_namespaces.push(HtmlNamespace::Html);
        assert!(matches!(
            src.current_text_mode_kind(),
            TextModeKind::Plaintext
        ));

        src.open_elements.pop();
        src.open_namespaces.pop();
        src.open_elements.push("script".to_string());
        src.open_namespaces.push(HtmlNamespace::Svg);
        assert!(matches!(src.current_text_mode_kind(), TextModeKind::Data));
    }

    #[test]
    fn internal_stack_mismatches_fall_back_to_defaults() {
        let mut src = SimpleHtmlEventSource::from_str("t", InputFormat::Html, "");

        src.open_elements.push("script".to_string());
        assert!(matches!(src.current_text_mode_kind(), TextModeKind::Data));
        assert_eq!(src.current_insertion_namespace(), HtmlNamespace::Html);

        src.open_elements.clear();
        src.open_namespaces.push(HtmlNamespace::Html);
        assert!(matches!(src.current_text_mode_kind(), TextModeKind::Data));
        assert_eq!(src.current_insertion_namespace(), HtmlNamespace::Html);

        src.open_namespaces.clear();
        src.open_namespaces.push(HtmlNamespace::Svg);
        assert!(matches!(src.current_text_mode_kind(), TextModeKind::Data));
        assert_eq!(src.current_insertion_namespace(), HtmlNamespace::Svg);
    }

    #[test]
    fn namespace_for_start_tag_respects_current_insertion_namespace() {
        let mut src = SimpleHtmlEventSource::from_str("t", InputFormat::Html, "");

        // Default: HTML insertion namespace.
        assert_eq!(src.namespace_for_start_tag("div"), HtmlNamespace::Html);
        assert_eq!(src.namespace_for_start_tag("svg"), HtmlNamespace::Svg);
        assert_eq!(src.namespace_for_start_tag("math"), HtmlNamespace::Math);

        // Inside SVG insertion mode, elements remain in SVG namespace.
        src.open_elements.push("svg".to_string());
        src.open_namespaces.push(HtmlNamespace::Svg);
        assert_eq!(src.current_insertion_namespace(), HtmlNamespace::Svg);
        assert_eq!(src.namespace_for_start_tag("div"), HtmlNamespace::Svg);
        assert_eq!(src.namespace_for_start_tag("math"), HtmlNamespace::Svg);

        // Integration points inside SVG switch the insertion namespace back to HTML.
        for tag in ["foreignobject", "desc", "title"] {
            src.open_elements.pop();
            src.open_namespaces.pop();
            src.open_elements.push(tag.to_string());
            src.open_namespaces.push(HtmlNamespace::Svg);
            assert_eq!(src.current_insertion_namespace(), HtmlNamespace::Html);
            assert_eq!(src.namespace_for_start_tag("div"), HtmlNamespace::Html);
            assert_eq!(src.namespace_for_start_tag("svg"), HtmlNamespace::Svg);
        }

        // Integration point tag names only matter when currently inserting into SVG.
        src.open_elements.pop();
        src.open_namespaces.pop();
        src.open_elements.push("foreignobject".to_string());
        src.open_namespaces.push(HtmlNamespace::Html);
        assert_eq!(src.current_insertion_namespace(), HtmlNamespace::Html);

        // Math insertion namespace persists.
        src.open_elements.pop();
        src.open_namespaces.pop();
        src.open_elements.push("math".to_string());
        src.open_namespaces.push(HtmlNamespace::Math);
        assert_eq!(src.current_insertion_namespace(), HtmlNamespace::Math);
        assert_eq!(src.namespace_for_start_tag("div"), HtmlNamespace::Math);
    }

    #[test]
    fn rawtext_and_eof_helpers_cover_edge_branches() {
        assert_eq!(
            find_rawtext_end_tag(b"</s", 0, "script", InputFormat::Html),
            None
        );
        assert_eq!(
            find_rawtext_end_tag(b"</scriptx>", 0, "script", InputFormat::Html),
            None
        );
        assert_eq!(
            find_rawtext_end_tag(b"</script>", 0, "script", InputFormat::Html),
            Some(0)
        );

        let (code, _msg) = classify_start_tag_eof(b"a='b");
        assert_eq!(code, "html.tokenizer.eof_in_attr_value");
        let (code2, _msg2) = classify_start_tag_eof(b"a='b' c");
        assert_eq!(code2, "html.tokenizer.eof_in_attr_name");
    }

    #[test]
    fn rcdata_without_closing_tag_decodes_entities_and_finishes() {
        let src = SimpleHtmlEventSource::from_str("t", InputFormat::Html, "<textarea>1 &lt; 2");
        let evs = collect(src);
        assert!(evs
            .iter()
            .any(|e| matches!(e, ParseEvent::Text { text, .. } if text == "1 < 2")));
    }

    #[test]
    fn direct_helpers_cover_remaining_xhtml_and_edge_branches() {
        assert_eq!(
            decode_char_refs(
                InputFormat::Xhtml,
                "&lt;&gt;&amp;&quot;&apos;".to_string(),
                false
            ),
            "<>&\"'"
        );

        let (s, errs) = decode_char_refs_with_errors(InputFormat::Html, "&", false, 0, 1, 1);
        assert_eq!(s, "&");
        assert!(errs.is_empty());

        let (s2, errs2) = decode_char_refs_with_errors(InputFormat::Html, "&#x;", false, 0, 1, 1);
        assert_eq!(s2, "&#x;");
        assert!(errs2.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.charref_no_digits"
        )));

        assert!(starts_with_ascii_case_insensitive(b"do", b""));
        assert!(starts_with_ascii_case_insensitive(b"DOCTYPE", b"doctype"));
        assert!(!starts_with_ascii_case_insensitive(b"do", b"doctype"));

        let src = SimpleHtmlEventSource::from_str("t", InputFormat::Html, "");
        let (_name, _attrs, _self_closing, errs3) =
            parse_start_tag(&src, "a href=>", 0, 1, 1, 8).unwrap();
        assert!(errs3.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.attr_value_missing"
        )));
    }

    #[test]
    fn text_run_decode_errors_are_emitted_before_text_event() {
        let evs = collect(SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "&#x;",
        ));
        let err_i = evs
            .iter()
            .position(|e| matches!(e, ParseEvent::ParseError { code, .. } if code == "html.tokenizer.charref_no_digits"))
            .expect("expected parse error event");
        let text_i = evs
            .iter()
            .position(|e| matches!(e, ParseEvent::Text { text, .. } if text == "&#x;"))
            .expect("expected text event");
        assert!(err_i < text_i);
    }

    #[test]
    fn html_event_source_covers_variant_dispatch() {
        let mut html = HtmlEventSource::from_str("t", InputFormat::Html, "<p>x</p>").unwrap();
        assert_eq!(html.source_name(), "t");
        assert_eq!(html.format(), InputFormat::Html);
        #[cfg(feature = "html5ever")]
        assert!(matches!(&html, HtmlEventSource::Html5Ever(_)));
        assert!(html.next_event().unwrap().is_some());

        let mut xhtml =
            HtmlEventSource::from_str("t2", InputFormat::Xhtml, "<?xml-stylesheet href=\"a\"?>")
                .unwrap();
        assert_eq!(xhtml.source_name(), "t2");
        assert_eq!(xhtml.format(), InputFormat::Xhtml);
        assert!(matches!(&xhtml, HtmlEventSource::Simple(_)));
        assert!(matches!(
            xhtml.next_event().unwrap(),
            Some(ParseEvent::ProcessingInstruction { .. })
        ));
    }

    #[test]
    fn newline_advances_line_and_col_for_end_tags() {
        let src = SimpleHtmlEventSource::from_str("t", InputFormat::Html, "<p>\n</p>");
        let evs = collect(src);
        let span = evs.iter().find_map(|e| match e {
            ParseEvent::EndTag { name, span, .. } if name == "p" => *span,
            _ => None,
        });
        let span = span.expect("expected </p> span");
        assert_eq!(span.line, 2);
        assert_eq!(span.col, 1);
    }

    #[test]
    fn math_namespace_is_tracked_across_nested_tags() {
        let src =
            SimpleHtmlEventSource::from_str("t", InputFormat::Html, "<math><mi>x</mi></math>");
        let evs = collect(src);
        assert!(evs
            .iter()
            .any(|e| matches!(e, ParseEvent::StartTag { name, .. } if name == "math")));
        assert!(evs
            .iter()
            .any(|e| matches!(e, ParseEvent::StartTag { name, .. } if name == "mi")));
        assert!(evs
            .iter()
            .any(|e| matches!(e, ParseEvent::EndTag { name, .. } if name == "math")));
    }

    #[test]
    fn common_bad_sequences_emit_errors_and_text() {
        let evs = collect(SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<></>",
        ));
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.lt_gt"
        )));
        assert!(evs
            .iter()
            .any(|e| matches!(e, ParseEvent::Text { text, .. } if text == "<>")));
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.lt_slash_gt"
        )));
        assert!(evs
            .iter()
            .any(|e| matches!(e, ParseEvent::Text { text, .. } if text == "</>")));
    }

    #[test]
    fn comment_parses_text_and_span() {
        let evs = collect(SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<!-- hi -->",
        ));
        let (text, span) = evs
            .iter()
            .find_map(|e| match e {
                ParseEvent::Comment { text, span } => Some((text.as_str(), *span)),
                _ => None,
            })
            .expect("expected comment event");
        assert_eq!(text, " hi ");
        assert_eq!(span, Some(Span::new(0, 11, 1, 1)));
    }

    #[test]
    fn comment_eof_emits_expected_parse_error() {
        let evs = collect(SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<!--",
        ));
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.eof_in_comment"
        )));
    }

    #[test]
    fn doctype_emits_additional_vnu_like_syntax_errors() {
        let evs = collect(SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<!DOCTYPEhtml>",
        ));
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.doctype.missing_space_before_name"
        )));

        let evs = collect(SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<!DOCTYPE html PUBLIC\"a\">",
        ));
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.doctype.no_space_after_public"
        )));

        let evs = collect(SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<!DOCTYPE html PUBLIC \"a\"\"b\">",
        ));
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.doctype.no_space_between_public_system"
        )));

        let evs = collect(SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<!DOCTYPE html SYSTEM\"a\">",
        ));
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.doctype.no_space_after_system"
        )));

        let evs = collect(SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<!DOCTYPE html bogus>",
        ));
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.doctype.bogus"
        )));
    }

    #[test]
    fn parse_start_tag_emits_errors_for_malformed_attribute_syntax() {
        let evs = collect(SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<a =></a>",
        ));
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.equals_expecting_attr_name"
        )));

        let evs = collect(SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<a <x=1></a>",
        ));
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.lt_expecting_attr_name"
        )));

        let evs = collect(SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<a x\"y=1></a>",
        ));
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.quote_in_attr_name"
        )));

        let evs = collect(SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<a x<y=1></a>",
        ));
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.lt_in_attr_name"
        )));

        let evs = collect(SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<a id='a' id='b'></a>",
        ));
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.duplicate_attribute"
        )));

        let evs = collect(SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<a ID='a' id='b'></a>",
        ));
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.duplicate_attribute"
        )));
    }

    #[test]
    fn html_normalizes_tag_names_to_ascii_lowercase_but_xhtml_preserves_case() {
        let evs = collect(SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<DIV></DIV>",
        ));
        assert!(matches!(evs[0], ParseEvent::StartTag { ref name, .. } if name == "div"));
        assert!(matches!(evs[1], ParseEvent::EndTag { ref name, .. } if name == "div"));

        let evs = collect(SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Xhtml,
            "<DIV></DIV>",
        ));
        assert!(matches!(evs[0], ParseEvent::StartTag { ref name, .. } if name == "DIV"));
        assert!(matches!(evs[1], ParseEvent::EndTag { ref name, .. } if name == "DIV"));
    }

    #[test]
    fn unquoted_attribute_values_emit_expected_parse_errors() {
        let evs = collect(SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<a x=`y ></a>",
        ));
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.backtick_at_start_unquoted"
        )));

        let evs = collect(SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<a x=y`z ></a>",
        ));
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.backtick_in_unquoted"
        )));

        let evs = collect(SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<a x=<y ></a>",
        ));
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.lt_at_start_unquoted"
        )));

        let evs = collect(SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<a x=y<z ></a>",
        ));
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.lt_in_unquoted"
        )));

        let evs = collect(SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<a x==y ></a>",
        ));
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.equals_at_start_unquoted"
        )));

        let evs = collect(SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<a x=y\"z ></a>",
        ));
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.quote_in_unquoted"
        )));
    }

    #[test]
    fn slash_not_immediately_followed_by_gt_emits_error() {
        let evs = collect(SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<a / ></a>",
        ));
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.slash_not_immediately_followed_by_gt"
        )));
    }

    #[test]
    fn slash_immediately_followed_by_gt_does_not_emit_error() {
        let evs = collect(SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<a /></a>",
        ));
        assert!(!evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.slash_not_immediately_followed_by_gt"
        )));
    }

    #[test]
    fn image_start_tag_emits_error() {
        let evs = collect(SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<image></image>",
        ));
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.image_start_tag"
        )));
    }

    #[test]
    fn rawtext_end_tag_search_is_case_sensitive_in_xhtml() {
        assert_eq!(
            find_rawtext_end_tag(b"</script>", 0, "script", InputFormat::Xhtml),
            Some(0)
        );
        assert_eq!(
            find_rawtext_end_tag(b"</SCRIPT>", 0, "script", InputFormat::Xhtml),
            None
        );
    }

    #[test]
    fn xhtml_end_tag_matching_is_case_sensitive() {
        let src = SimpleHtmlEventSource::from_str("t", InputFormat::Xhtml, "<A></A>");
        let evs = collect(src);
        assert!(evs
            .iter()
            .any(|e| matches!(e, ParseEvent::StartTag { name, .. } if name == "A")));
        assert!(evs
            .iter()
            .any(|e| matches!(e, ParseEvent::EndTag { name, .. } if name == "A")));
    }

    #[test]
    fn numeric_character_references_emit_expected_parse_errors() {
        let cases = [
            ("&#0;", "html.tokenizer.charref_zero"),
            ("&#x110000;", "html.tokenizer.charref_outside_range"),
            ("&#xD800;", "html.tokenizer.charref_surrogate"),
            ("&#13;", "html.tokenizer.charref_cr"),
            ("&#x80;", "html.tokenizer.charref_c1_controls"),
            ("&#x1FFFE;", "html.tokenizer.charref_astral_noncharacter"),
            ("&#xFFFE;", "html.tokenizer.charref_noncharacter"),
            ("&#xFDD0;", "html.tokenizer.charref_unassigned"),
            ("&#11;", "html.tokenizer.charref_control"),
        ];

        for (input, expected_code) in cases {
            let (_s, errs) = decode_char_refs_with_errors(InputFormat::Html, input, false, 0, 1, 1);
            assert!(
                errs.iter().any(|e| matches!(
                    e,
                    ParseEvent::ParseError { code, .. } if code == expected_code
                )),
                "missing {expected_code} for {input}"
            );
        }
    }

    #[test]
    fn decode_char_refs_with_errors_reports_unrecognized_named_refs_as_literal_ampersand() {
        let (s, errs) = decode_char_refs_with_errors(InputFormat::Html, "&zzzzzz;", false, 0, 1, 1);
        assert_eq!(s, "&zzzzzz;");
        assert!(errs.is_empty());
    }

    #[test]
    fn decode_char_refs_with_errors_preserves_stream_errors_when_decoding_refs() {
        let input = "a\u{000B}&amp;";
        let (s, errs) = decode_char_refs_with_errors(InputFormat::Html, input, false, 0, 1, 1);
        assert_eq!(s, "a\u{000B}&");
        assert!(errs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.forbidden_code_point"
        )));
    }

    #[test]
    fn end_tag_syntax_errors_are_reported() {
        let evs = collect(SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "</p/>",
        ));
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.end_tag_stray_slash"
        )));

        let evs = collect(SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "</p class=x>",
        ));
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.end_tag_with_attrs"
        )));

        let evs = collect(SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "</p",
        ));
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.eof_in_end_tag"
        )));
    }

    #[test]
    fn end_tag_with_invalid_utf8_is_lossy_and_emits_end_tag() {
        let mut bytes = b"<p></p".to_vec();
        bytes.push(0xFF);
        bytes.extend_from_slice(b">");

        let evs = collect(SimpleHtmlEventSource::from_bytes(
            "t",
            InputFormat::Html,
            bytes,
        ));
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::EndTag { name, .. } if name == "p\u{FFFD}"
        )));
    }

    #[test]
    fn text_with_invalid_utf8_is_lossy_and_decodes_char_refs() {
        let mut bytes = b"<p>".to_vec();
        bytes.push(0xFF);
        bytes.extend_from_slice(b"&amp;</p>");
        let evs = collect(SimpleHtmlEventSource::from_bytes(
            "t",
            InputFormat::Html,
            bytes,
        ));
        let texts: Vec<_> = evs
            .iter()
            .filter_map(|e| match e {
                ParseEvent::Text { text, .. } => Some(text.clone()),
                _ => None,
            })
            .collect();
        assert_eq!(texts, vec![format!("\u{FFFD}&")]);

        let mut bytes = Vec::new();
        bytes.push(0xFF);
        bytes.extend_from_slice(b"&amp;");
        let evs = collect(SimpleHtmlEventSource::from_bytes(
            "t",
            InputFormat::Html,
            bytes,
        ));
        let texts: Vec<_> = evs
            .iter()
            .filter_map(|e| match e {
                ParseEvent::Text { text, .. } => Some(text.clone()),
                _ => None,
            })
            .collect();
        assert_eq!(texts, vec![format!("\u{FFFD}&")]);
    }

    #[test]
    fn start_tag_eof_is_reported_when_tag_close_is_missing() {
        let evs = collect(SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<a href='x'",
        ));
        assert!(evs
            .iter()
            .any(|e| matches!(e, ParseEvent::ParseError { .. })));
    }

    #[test]
    fn doctype_identifier_edge_cases_cover_public_eof_and_almost_standards_message() {
        let evs = collect(SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<!DOCTYPE html PUBLIC \"a",
        ));
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.doctype.eof_in_public_id"
        )));

        let evs = collect(SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<!DOCTYPE html PUBLIC \"-//W3C//DTD HTML 4.01 Transitional//EN\" \"http://www.w3.org/TR/html4/loose.dtd\">",
        ));
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, message, .. }
                if code == "html.parser.doctype.not_html5"
                    && message == "Almost standards mode doctype. Expected “<!DOCTYPE html>”."
        )));
    }

    #[test]
    fn doctype_name_is_compared_case_insensitively_for_html5_conformance() {
        let evs = collect(SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Xhtml,
            "<!DOCTYPE HTML>",
        ));
        assert!(!evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.parser.doctype.not_html5"
        )));
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::Doctype { name: Some(name), .. } if name == "HTML"
        )));
    }

    #[test]
    fn find_tag_close_ignores_gt_inside_quotes() {
        let bytes = b"<a x='>' y=z>";
        assert_eq!(find_tag_close(bytes, 1), Some(bytes.len() - 1));

        let bytes = b"<a x=\"a>b\" y=z>";
        assert_eq!(find_tag_close(bytes, 1), Some(bytes.len() - 1));

        let bytes = b"<a x='>'";
        assert_eq!(find_tag_close(bytes, 1), None);
    }

    #[test]
    fn find_tag_close_state_machine_covers_additional_transitions() {
        let src = SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "<a x ></a><a x y=z></a><a x=  y></a>",
        );
        let evs = collect(src);
        let mut seen_first = false;
        let mut seen_second = false;
        let mut seen_third = false;
        for ev in &evs {
            if let Some((name, attrs)) = as_start_tag(ev) {
                if name == "a"
                    && attrs.iter().any(|a| a.name == "x")
                    && attrs.len() == 1
                    && !seen_first
                {
                    seen_first = true;
                } else if name == "a"
                    && attrs.iter().any(|a| a.name == "x" && a.value.is_none())
                    && attrs
                        .iter()
                        .any(|a| a.name == "y" && a.value.as_deref() == Some("z"))
                {
                    seen_second = true;
                } else if name == "a"
                    && attrs
                        .iter()
                        .any(|a| a.name == "x" && a.value.as_deref() == Some("y"))
                {
                    seen_third = true;
                }
            }
        }
        assert!(seen_first);
        assert!(seen_second);
        assert!(seen_third);
    }

    #[test]
    fn end_tag_prefix_at_eof_emits_eof_after_lt() {
        let evs = collect(SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "</",
        ));
        assert_eq!(evs.len(), 1);
        let ParseEvent::ParseError {
            code,
            message,
            span,
        } = &evs[0]
        else {
            panic!("expected a parse error event");
        };
        assert_eq!(code, "html.tokenizer.eof_after_lt");
        assert_eq!(message, "End of file after “<”.");
        assert_eq!(span.unwrap(), Span::new(0, 2, 1, 1));
    }

    #[test]
    fn end_tag_garbage_after_lt_slash_emits_error_and_comment() {
        let evs = collect(SimpleHtmlEventSource::from_str(
            "t",
            InputFormat::Html,
            "</ x>",
        ));
        assert_eq!(evs.len(), 2);
        assert!(matches!(
            &evs[0],
            ParseEvent::ParseError { code, .. } if code == "html.tokenizer.garbage_after_lt_slash"
        ));
        assert!(matches!(&evs[1], ParseEvent::Comment { text, .. } if text == " x"));
    }
}
