use std::borrow::Cow;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

use html5ever::tendril::StrTendril;
use html5ever::tendril::TendrilSink;
use html5ever::{parse_document, ParseOpts};
use html_inspector_core::{Attribute, EventSource, InputFormat, ParseEvent, ValidatorError};
use markup5ever::ns;
use markup5ever::TokenizerResult;
use markup5ever_rcdom::{Handle, NodeData, RcDom};

#[inline]
fn normalize_double_xmlns_prefix(attr_name: &mut String) {
    const XMLNS: &str = "xmlns:";

    let rest = attr_name.as_str();
    let stripped = rest.trim_start_matches(XMLNS);
    let drop_len = rest.len() - stripped.len();

    // `drop_len` is always a UTF-8 boundary because it advances in ASCII-sized prefix chunks.

    // Only normalize "double xmlns:" (or more), preserving valid single-prefix forms like
    // `xmlns:foo`.
    if drop_len >= 2 * XMLNS.len() {
        attr_name.drain(..drop_len);
    }
}

#[cfg(test)]
mod normalize_double_xmlns_prefix_tests {
    use super::normalize_double_xmlns_prefix;

    #[test]
    fn does_not_normalize_single_prefix() {
        let mut s = "xmlns:foo".to_string();
        normalize_double_xmlns_prefix(&mut s);
        assert_eq!(s, "xmlns:foo");
    }

    #[test]
    fn normalizes_double_and_triple_prefixes() {
        let mut s = "xmlns:xmlns:foo".to_string();
        normalize_double_xmlns_prefix(&mut s);
        assert_eq!(s, "foo");

        let mut s = "xmlns:xmlns:xmlns:foo".to_string();
        normalize_double_xmlns_prefix(&mut s);
        assert_eq!(s, "foo");
    }

    #[test]
    fn leaves_non_matching_repeated_prefixes_untouched() {
        let mut s = "xmlns:Xmlns:foo".to_string();
        normalize_double_xmlns_prefix(&mut s);
        assert_eq!(s, "xmlns:Xmlns:foo");
    }

    #[test]
    fn allows_empty_rest_after_normalization() {
        let mut s = "xmlns:xmlns:".to_string();
        normalize_double_xmlns_prefix(&mut s);
        assert_eq!(s, "");
    }
}

#[derive(Clone)]
struct StagedEvent {
    event: ParseEvent,
    flush_syntax_errors_before: Option<usize>,
}

#[derive(Clone)]
struct WalkFrame {
    handle: Handle,
    state: WalkState,
    element: Option<ElementFrame>,
}

#[derive(Clone)]
struct ElementFrame {
    tag: String,
    is_void: bool,
}

#[derive(Clone, Copy)]
enum WalkState {
    Enter,
    Children { next: usize },
    Template { next: usize },
    Exit,
}

impl WalkFrame {
    fn new(handle: Handle) -> Self {
        Self {
            handle,
            state: WalkState::Enter,
            element: None,
        }
    }
}

#[derive(Clone)]
pub struct Html5EverEventSource {
    name: String,
    bytes: Option<Arc<Vec<u8>>>,
    root: Option<Handle>,
    walk_stack: Vec<WalkFrame>,
    staged: Option<StagedEvent>,
    start_tag_spans: HashMap<String, VecDeque<html_inspector_core::Span>>,
    end_tag_spans: HashMap<String, VecDeque<html_inspector_core::Span>>,
    syntax_errors: VecDeque<ParseEvent>,
}

impl Html5EverEventSource {
    pub fn from_bytes(name: impl Into<String>, bytes: Vec<u8>) -> Self {
        Self::from_shared_bytes(name, Arc::new(bytes))
    }

    pub fn from_shared_bytes(name: impl Into<String>, bytes: Arc<Vec<u8>>) -> Self {
        Self {
            name: name.into(),
            bytes: Some(bytes),
            root: None,
            walk_stack: Vec::new(),
            staged: None,
            start_tag_spans: HashMap::new(),
            end_tag_spans: HashMap::new(),
            syntax_errors: VecDeque::new(),
        }
    }

    fn ensure_parsed(&mut self) -> Result<(), ValidatorError> {
        if self.root.is_some() {
            return Ok(());
        }

        self.push_simple_syntax_parse_errors()?;

        // The html5ever parser copies input into tendrils; drop our copy early to reduce peak heap.
        // This is safe after the prescan because we only need tag-span maps + queued syntax errors afterwards.
        let bytes = self.bytes.take().unwrap_or_default();

        let mut opts = ParseOpts::default();
        opts.tree_builder.exact_errors = true;

        // `html5ever`’s `TendrilSink` driver only drains the tokenizer while it returns
        // `Script(_)`. Some documents (e.g. those with multiple `<meta charset=...>` tags)
        // cause the tokenizer to yield `EncodingIndicator(_)`, leaving data buffered and
        // triggering an internal assertion during `finish()`. Drive the tokenizer ourselves
        // so we can also ignore `EncodingIndicator(_)` and fully drain the buffer.
        let parser = parse_document(RcDom::default(), opts);
        if !bytes.is_empty() {
            let decoded = crate::str_from_bytes_lossy(bytes.as_ref());
            let tendril = StrTendril::from_slice(decoded.as_ref());
            parser.input_buffer.push_back(tendril);
            loop {
                match parser.tokenizer.feed(&parser.input_buffer) {
                    TokenizerResult::Done => break,
                    TokenizerResult::Script(_) | TokenizerResult::EncodingIndicator(_) => continue,
                }
            }
        }

        // Drop the original input bytes as soon as we've copied them into html5ever’s tendril.
        // This reduces peak heap while the DOM/tree builder allocates.
        drop(bytes);
        let dom: RcDom = parser.finish();
        let root = dom.document;
        self.root = Some(root.clone());
        self.walk_stack = vec![WalkFrame::new(root)];

        Ok(())
    }

    fn pop_ready_syntax_error(&mut self, before_byte_start: usize) -> Option<ParseEvent> {
        match self.syntax_errors.front() {
            Some(ParseEvent::ParseError { span: Some(s), .. })
                if s.byte_start <= before_byte_start =>
            {
                self.syntax_errors.pop_front()
            }
            _ => None,
        }
    }

    fn pop_span_for_tag(
        spans: &mut HashMap<String, VecDeque<html_inspector_core::Span>>,
        tag: &str,
    ) -> Option<html_inspector_core::Span> {
        if let Some(span) = spans.get_mut(tag).and_then(VecDeque::pop_front) {
            return Some(span);
        }

        match html_inspector_core::ascii_lowercase_cow_if_needed(tag) {
            Cow::Borrowed(_) => None,
            Cow::Owned(tag_lc) => spans.get_mut(tag_lc.as_str()).and_then(VecDeque::pop_front),
        }
    }

    #[inline]
    fn push_span_for_tag(
        spans: &mut HashMap<String, VecDeque<html_inspector_core::Span>>,
        tag: &str,
        span: html_inspector_core::Span,
    ) {
        if let Some(q) = spans.get_mut(tag) {
            q.push_back(span);
        } else {
            spans.insert(tag.to_owned(), VecDeque::from([span]));
        }
    }

    fn next_dom_event(&mut self) -> Option<StagedEvent> {
        loop {
            let frame = self.walk_stack.last_mut()?;
            match frame.state {
                WalkState::Enter => match &frame.handle.data {
                    NodeData::Document => {
                        frame.state = WalkState::Children { next: 0 };
                        continue;
                    }
                    NodeData::Doctype {
                        name,
                        public_id,
                        system_id,
                    } => {
                        let non_empty =
                            |s: &StrTendril| (!s.is_empty()).then(|| s.as_ref().to_owned());
                        let name = non_empty(name);
                        let public_id = non_empty(public_id);
                        let system_id = non_empty(system_id);
                        self.walk_stack.pop();
                        return Some(StagedEvent {
                            event: ParseEvent::Doctype {
                                name,
                                public_id,
                                system_id,
                                span: None,
                            },
                            flush_syntax_errors_before: None,
                        });
                    }
                    NodeData::Text { contents } => {
                        let text = contents.borrow().as_ref().to_owned();
                        self.walk_stack.pop();
                        if text.is_empty() {
                            continue;
                        }
                        return Some(StagedEvent {
                            event: ParseEvent::Text { text, span: None },
                            flush_syntax_errors_before: None,
                        });
                    }
                    NodeData::Comment { contents } => {
                        let text = contents.as_ref().to_owned();
                        self.walk_stack.pop();
                        return Some(StagedEvent {
                            event: ParseEvent::Comment { text, span: None },
                            flush_syntax_errors_before: None,
                        });
                    }
                    NodeData::Element { name, attrs, .. } => {
                        let mut tag = name.local.as_ref().to_owned();
                        let is_html_ns = name.ns == ns!(html);
                        if is_html_ns {
                            tag.make_ascii_lowercase();
                        }
                        let span = Self::pop_span_for_tag(&mut self.start_tag_spans, &tag);
                        let flush_syntax_errors_before = span.as_ref().map(|s| s.byte_start);

                        let attrs_borrow = attrs.borrow();
                        let mut out_attrs = Vec::with_capacity(attrs_borrow.len());
                        for a in attrs_borrow.iter() {
                            let local = a.name.local.as_ref();
                            let mut attr_name = if a.name.ns == ns!(xmlns) {
                                if local.is_empty() || local == "xmlns" {
                                    "xmlns".to_owned()
                                } else {
                                    let mut out =
                                        String::with_capacity("xmlns:".len() + local.len());
                                    out.push_str("xmlns:");
                                    out.push_str(local);
                                    out
                                }
                            } else if let Some(prefix) = a.name.prefix.as_ref() {
                                let prefix = prefix.as_ref();
                                let mut out = String::with_capacity(prefix.len() + 1 + local.len());
                                out.push_str(prefix);
                                out.push(':');
                                out.push_str(local);
                                out
                            } else {
                                local.to_owned()
                            };

                            // html5ever can sometimes surface xmlns:* as prefix + local pairs; normalize any "xmlns:" prefix.
                            normalize_double_xmlns_prefix(&mut attr_name);
                            if is_html_ns {
                                attr_name.make_ascii_lowercase();
                            }

                            out_attrs.push(Attribute {
                                name: attr_name,
                                value: Some(a.value.as_ref().to_owned()),
                                span: None,
                            });
                        }

                        frame.element = Some(ElementFrame {
                            is_void: html_inspector_core::is_void_html_element(&tag),
                            tag: tag.clone(),
                        });
                        frame.state = WalkState::Children { next: 0 };

                        return Some(StagedEvent {
                            event: ParseEvent::StartTag {
                                name: tag,
                                attrs: out_attrs,
                                self_closing: false,
                                span,
                            },
                            flush_syntax_errors_before,
                        });
                    }
                    _ => {
                        self.walk_stack.pop();
                        continue;
                    }
                },
                WalkState::Children { next } => {
                    let child = frame.handle.children.borrow().get(next).cloned();
                    if let Some(child) = child {
                        frame.state = WalkState::Children { next: next + 1 };
                        self.walk_stack.push(WalkFrame::new(child));
                        continue;
                    }

                    match &frame.handle.data {
                        NodeData::Document => {
                            self.walk_stack.pop();
                            continue;
                        }
                        NodeData::Element {
                            template_contents, ..
                        } => {
                            let has_template = template_contents.borrow().is_some();
                            frame.state = if has_template {
                                WalkState::Template { next: 0 }
                            } else {
                                WalkState::Exit
                            };
                            continue;
                        }
                        _ => {
                            self.walk_stack.pop();
                            continue;
                        }
                    }
                }
                WalkState::Template { next } => {
                    let NodeData::Element {
                        template_contents, ..
                    } = &frame.handle.data
                    else {
                        frame.state = WalkState::Exit;
                        continue;
                    };
                    let child = template_contents
                        .borrow()
                        .as_ref()
                        .and_then(|t| t.children.borrow().get(next).cloned());
                    if let Some(child) = child {
                        frame.state = WalkState::Template { next: next + 1 };
                        self.walk_stack.push(WalkFrame::new(child));
                        continue;
                    }
                    frame.state = WalkState::Exit;
                    continue;
                }
                WalkState::Exit => {
                    let element = frame.element.take();
                    self.walk_stack.pop();

                    let Some(element) = element else {
                        continue;
                    };
                    if element.is_void {
                        continue;
                    }

                    let span = Self::pop_span_for_tag(&mut self.end_tag_spans, &element.tag);
                    let flush_syntax_errors_before = span.as_ref().map(|s| s.byte_start);
                    return Some(StagedEvent {
                        event: ParseEvent::EndTag {
                            name: element.tag,
                            span,
                        },
                        flush_syntax_errors_before,
                    });
                }
            }
        }
    }

    fn push_simple_syntax_parse_errors(&mut self) -> Result<(), ValidatorError> {
        // html5ever’s DOM doesn’t preserve some tokenization details we need for conformance rules
        // and parse-error parity. Pre-scan using the built-in lightweight scanner and surface
        // parse errors + a few additional syntax-level errors that the DOM backend would drop.
        let Some(bytes) = self.bytes.as_ref() else {
            return Ok(());
        };
        let mut errors: Vec<ParseEvent> = Vec::new();
        let mut scan = crate::SimpleHtmlEventSource::from_shared_bytes(
            self.name.clone(),
            InputFormat::Html,
            bytes.clone(),
        );
        let mut saw_doctype = false;
        let mut reported_missing_doctype = false;
        let mut body_closed = false;
        let mut body_opened = false;
        let mut open_stack: Vec<(String, Option<html_inspector_core::Span>)> = Vec::new();
        let mut open_script = 0usize;
        let mut first_unclosed_script_span = None;
        let mut reported_meta_charset_after_1024 = false;
        let mut reported_nested_form = false;
        let mut reported_nested_heading = false;
        let mut reported_nested_table = false;
        let mut reported_select_in_table = false;
        let mut reported_input_in_table = false;
        let mut reported_nonspace_in_noscript_head = false;
        let mut reported_nonspace_after_body = false;
        let mut reported_misplaced_table_text = false;
        let mut reported_stray_start_tag_after_body = false;
        let mut in_head = true;
        let mut in_noscript_in_head = false;
        fn in_table_without_cell(
            open_stack: &[(String, Option<html_inspector_core::Span>)],
        ) -> bool {
            let mut in_table = false;
            let mut in_cell = false;
            for (name, _) in open_stack {
                match name.as_str() {
                    "table" => in_table = true,
                    "td" | "th" => in_cell = true,
                    _ => {}
                }
            }
            in_table && !in_cell
        }
        while let Some(ev) = scan.next_event()? {
            match ev {
                ParseEvent::Doctype { span, .. } => {
                    if saw_doctype {
                        errors.push(ParseEvent::ParseError {
                            code: "html.tokenizer.stray_doctype".to_string(),
                            message: "Stray doctype.".to_string(),
                            span,
                        });
                    }
                    saw_doctype = true;
                }
                ParseEvent::StartTag {
                    name,
                    attrs,
                    self_closing,
                    span,
                    ..
                } => {
                    if let Some(span) = span {
                        Self::push_span_for_tag(&mut self.start_tag_spans, name.as_str(), span);
                    }
                    let lc = name;
                    let in_template = open_stack.iter().any(|(n, _)| n == "template");
                    let in_foreign_content =
                        open_stack.iter().any(|(n, _)| n == "svg" || n == "math");
                    if !in_foreign_content && lc == "image" {
                        if let Some(span) = span {
                            Self::push_span_for_tag(&mut self.start_tag_spans, "img", span);
                        }
                    }
                    if lc == "head" {
                        in_head = true;
                    } else if lc == "body" {
                        if body_opened && !body_closed {
                            errors.push(ParseEvent::ParseError {
                                code: "html.parser.body.already_open".to_string(),
                                message:
                                    "Start tag “body” seen but an element of the same type was already open."
                                        .to_string(),
                                span,
                            });
                            errors.push(ParseEvent::ParseError {
                                code: "html.parser.cannot_recover".to_string(),
                                message: "Cannot recover after last error. Any further errors will be ignored."
                                    .to_string(),
                                span,
                            });
                        }
                        body_opened = true;
                        in_head = false;
                        in_noscript_in_head = false;
                    }

                    if in_head && lc == "noscript" && !self_closing {
                        in_noscript_in_head = true;
                    }
                    if in_noscript_in_head && lc == "picture" {
                        errors.push(ParseEvent::ParseError {
                            code: "html.parser.bad_start_tag_in_noscript_head".to_string(),
                            message: "Bad start tag in “picture” in “noscript” in “head”."
                                .to_string(),
                            span,
                        });
                    }
                    // Tags inside <template> do not affect the main document’s insertion mode (in particular,
                    // they must not cause us to treat the body as implicitly opened while still in <head>).
                    if in_head && !in_template && !Self::is_head_metadata_tag(&lc) {
                        in_head = false;
                        in_noscript_in_head = false;
                        body_opened = true;
                    }
                    if is_p_end_tag_implying_start_tag(&lc) {
                        if let Some(pos) = open_stack.iter().rposition(|(n, _)| n == "p") {
                            if open_stack.last().is_none_or(|(n, _)| n != "p") {
                                errors.push(ParseEvent::ParseError {
                                    code: "html.parse.p.end_tag_implied_open_elements".to_string(),
                                    message: "End tag “p” implied, but there were open elements."
                                        .to_string(),
                                    span,
                                });
                            }
                            open_stack.truncate(pos);
                        }
                    }
                    // Head insertion mode "anything else": implicitly close <head>.
                    // This is a small, targeted subset needed for VNU/Java parse-error parity.
                    if !in_template
                        && lc != "html"
                        && lc != "head"
                        && lc != "body"
                        && open_stack.iter().any(|(n, _)| n == "head")
                        && !open_stack.iter().any(|(n, _)| n == "body")
                    {
                        let allowed_in_head = matches!(
                            lc.as_str(),
                            "base"
                                | "basefont"
                                | "bgsound"
                                | "link"
                                | "meta"
                                | "noframes"
                                | "noscript"
                                | "script"
                                | "style"
                                | "template"
                                | "title"
                        );
                        if !allowed_in_head {
                            if let Some(pos) = open_stack.iter().rposition(|(n, _)| n == "head") {
                                open_stack.truncate(pos);
                            }
                        }
                    }
                    if !saw_doctype && !reported_missing_doctype {
                        errors.push(ParseEvent::ParseError {
                            code: "html.parser.start_tag_without_doctype".to_string(),
                            message: "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”."
                                .to_string(),
                            span,
                        });
                        reported_missing_doctype = true;
                    }
                    if !reported_meta_charset_after_1024
                        && lc == "meta"
                        && span.is_some_and(|s| s.byte_start >= 1024)
                        && attrs.iter().any(|a| a.name == "charset")
                    {
                        errors.push(ParseEvent::ParseError {
                            code: "html.meta.charset_after_1024".to_string(),
                            message: "A “charset” attribute on a “meta” element found after the first 1024 bytes."
                                .to_string(),
                            span,
                        });
                        reported_meta_charset_after_1024 = true;
                    }
                    if lc == "p"
                        && open_stack.len() >= 2
                        && open_stack.last().is_some_and(|(n, _)| n == "div")
                        && open_stack
                            .get(open_stack.len() - 2)
                            .is_some_and(|(n, _)| n == "dl")
                    {
                        errors.push(ParseEvent::ParseError {
                            code: "html.validator.element_not_allowed".to_string(),
                            message: "Element “p” not allowed as child of “div” in this context."
                                .to_string(),
                            span,
                        });
                    }
                    if let Some(role) = attrs
                        .iter()
                        .find(|a| a.name == "role")
                        .and_then(|a| a.value.as_deref())
                    {
                        let role_trim = role.trim();
                        let mut token_count = 0usize;
                        let bad_token = role_trim.split_whitespace().any(|token| {
                            token_count += 1;
                            !is_potential_aria_role_token(token)
                        });
                        if bad_token {
                            errors.push(ParseEvent::ParseError {
                                code: "html.attr.role.bad_value".to_string(),
                                message: format!(
                                    "Bad value “{role_trim}” for attribute “role” on element “{lc}”."
                                ),
                                span,
                            });
                        } else {
                            // Avoid emitting "discarding unrecognized token" for single-token role values;
                            // those are reported via the element-specific role checks (e.g. img role constraints).
                            if token_count > 1 {
                                for token in role_trim.split_whitespace() {
                                    let token_lc =
                                        html_inspector_core::ascii_lowercase_cow_if_needed(token);
                                    let is_defined =
                                        is_defined_non_abstract_aria_role(token_lc.as_ref());
                                    if !is_defined {
                                        errors.push(ParseEvent::ParseError {
                                            code: "aria.role.unrecognized_token".to_string(),
                                            message: format!(
                                                "Discarding unrecognized token “{token}” from value of attribute “role”. Browsers ignore any token that is not a defined ARIA non-abstract role."
                                            ),
                                            span,
                                        });
                                    }
                                }
                            }
                        }
                    }
                    if !reported_nested_form
                        && lc == "form"
                        && open_stack.iter().any(|(n, _)| n == "form")
                    {
                        errors.push(ParseEvent::ParseError {
                            code: "html.parser.nested_form".to_string(),
                            message: "Saw a “form” start tag, but there was already an active “form” element. Nested forms are not allowed. Ignoring the tag.".to_string(),
                            span,
                        });
                        reported_nested_form = true;
                    }
                    if !reported_nested_heading
                        && matches!(lc.as_str(), "h1" | "h2" | "h3" | "h4" | "h5" | "h6")
                        && open_stack.iter().any(|(n, _)| {
                            matches!(n.as_str(), "h1" | "h2" | "h3" | "h4" | "h5" | "h6")
                        })
                    {
                        errors.push(ParseEvent::ParseError {
                            code: "html.parser.nested_heading".to_string(),
                            message: "Heading cannot be a child of another heading.".to_string(),
                            span,
                        });
                        reported_nested_heading = true;
                    }
                    if !reported_nested_table && lc == "table" && in_table_without_cell(&open_stack)
                    {
                        errors.push(ParseEvent::ParseError {
                            code: "html.parser.nested_table".to_string(),
                            message:
                                "Start tag for “table” seen but the previous “table” is still open."
                                    .to_string(),
                            span,
                        });
                        reported_nested_table = true;
                    }
                    if !reported_select_in_table
                        && lc == "select"
                        && open_stack.iter().any(|(n, _)| n == "table")
                    {
                        errors.push(ParseEvent::ParseError {
                            code: "html.parser.select_in_table".to_string(),
                            message: "Start tag “select” seen in “table”.".to_string(),
                            span,
                        });
                        reported_select_in_table = true;
                    }
                    if !reported_input_in_table
                        && lc == "input"
                        && in_table_without_cell(&open_stack)
                    {
                        errors.push(ParseEvent::ParseError {
                            code: "html.parser.input_in_table".to_string(),
                            message: "Start tag “input” seen in “table”.".to_string(),
                            span,
                        });
                        reported_input_in_table = true;
                    }
                    if !reported_stray_start_tag_after_body
                        && body_closed
                        && lc != "html"
                        && lc != "head"
                        && lc != "body"
                    {
                        errors.push(ParseEvent::ParseError {
                            code: "html.parser.stray_start_tag".to_string(),
                            message: format!("Stray start tag “{lc}”."),
                            span,
                        });
                        reported_stray_start_tag_after_body = true;
                    }
                    if lc == "script" && !self_closing {
                        if open_script == 0 {
                            first_unclosed_script_span = span;
                        }
                        open_script += 1;
                    }
                    if self_closing
                        && !in_foreign_content
                        && !html_inspector_core::is_void_html_element(&lc)
                    {
                        errors.push(ParseEvent::ParseError {
                            code: "html.parse.self_closing.non_void".to_string(),
                            message: "Self-closing syntax (“/>”) used on a non-void HTML element. Ignoring the slash and treating as a start tag.".to_string(),
                            span,
                        });
                    }
                    if !self_closing && !html_inspector_core::is_void_html_element(&lc) {
                        open_stack.push((lc, span));
                    }
                }
                ParseEvent::EndTag { name, span } => {
                    if let Some(span) = span {
                        Self::push_span_for_tag(&mut self.end_tag_spans, name.as_str(), span);
                    }
                    if !saw_doctype && !reported_missing_doctype {
                        errors.push(ParseEvent::ParseError {
                            code: "html.parser.end_tag_without_doctype".to_string(),
                            message: "End tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”."
                                .to_string(),
                            span,
                        });
                        reported_missing_doctype = true;
                    }

                    let lc = name.as_str();
                    if lc == "body" {
                        body_closed = true;
                        in_head = false;
                        in_noscript_in_head = false;
                    } else if lc == "head" {
                        in_head = false;
                        in_noscript_in_head = false;
                    } else if lc == "noscript" {
                        in_noscript_in_head = false;
                    } else if body_closed && lc != "html" {
                        errors.push(ParseEvent::ParseError {
                            code: "html.parser.end_tag_after_body_closed".to_string(),
                            message: "Saw an end tag after “body” had been closed.".to_string(),
                            span,
                        });
                    }

                    if lc == "script" && open_script > 0 {
                        open_script -= 1;
                        if open_script == 0 {
                            first_unclosed_script_span = None;
                        }
                    }

                    if html_inspector_core::is_void_html_element(lc) {
                        if lc == "br" {
                            errors.push(ParseEvent::ParseError {
                                code: "html.void_element.end_tag".to_string(),
                                message: "End tag “br”.".to_string(),
                                span,
                            });
                        } else {
                            errors.push(ParseEvent::ParseError {
                                code: "html.parser.stray_end_tag".to_string(),
                                message: format!("Stray end tag “{lc}”."),
                                span,
                            });
                        }
                    } else if lc == "p" && !open_stack.iter().any(|(n, _)| n == "p") {
                        errors.push(ParseEvent::ParseError {
                            code: "html.parser.no_p_in_scope".to_string(),
                            message: "No “p” element in scope but a “p” end tag seen.".to_string(),
                            span,
                        });
                    } else if !open_stack.iter().any(|(n, _)| n == lc) {
                        errors.push(ParseEvent::ParseError {
                            code: "html.parser.stray_end_tag".to_string(),
                            message: format!("Stray end tag “{lc}”."),
                            span,
                        });
                    } else if let Some(pos) = open_stack.iter().rposition(|(n, _)| n == lc) {
                        open_stack.truncate(pos);
                    }
                }
                ParseEvent::ParseError { ref code, .. } => {
                    let in_foreign_content =
                        open_stack.iter().any(|(n, _)| n == "svg" || n == "math");
                    if in_foreign_content && code == "html.tokenizer.image_start_tag" {
                        continue;
                    }
                    errors.push(ev);
                }
                ParseEvent::Text { ref text, span } => {
                    let check_nonspace_after_body = !reported_nonspace_after_body && body_closed;
                    let check_misplaced_table_text = !reported_misplaced_table_text
                        && open_stack.last().is_some_and(|(n, _)| n == "table")
                        && !open_stack.iter().any(|(n, _)| n == "td" || n == "th");
                    let check_nonspace_in_noscript_head = !reported_nonspace_in_noscript_head
                        && open_stack.iter().any(|(n, _)| n == "head")
                        && open_stack.iter().any(|(n, _)| n == "noscript");

                    if check_nonspace_after_body
                        || check_misplaced_table_text
                        || check_nonspace_in_noscript_head
                    {
                        let has_non_space = text.chars().any(|c| !c.is_whitespace());
                        if has_non_space {
                            if check_nonspace_after_body {
                                errors.push(ParseEvent::ParseError {
                                    code: "html.parser.nonspace_after_body".to_string(),
                                    message: "Non-space character after body.".to_string(),
                                    span,
                                });
                                reported_nonspace_after_body = true;
                            }
                            if check_misplaced_table_text {
                                errors.push(ParseEvent::ParseError {
                                    code: "html.parser.misplaced_table_text".to_string(),
                                    message: "Misplaced non-space characters inside a table."
                                        .to_string(),
                                    span,
                                });
                                reported_misplaced_table_text = true;
                            }
                            if check_nonspace_in_noscript_head {
                                errors.push(ParseEvent::ParseError {
                                    code: "html.parser.nonspace_in_noscript_head".to_string(),
                                    message: "Non-space character inside “noscript” inside “head”."
                                        .to_string(),
                                    span,
                                });
                                reported_nonspace_in_noscript_head = true;
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        if open_script > 0 {
            errors.push(ParseEvent::ParseError {
                code: "html.parser.eof_in_rawtext".to_string(),
                message: "End of file seen when expecting text or an end tag.".to_string(),
                span: first_unclosed_script_span,
            });
        } else if let Some((_, span)) = open_stack.iter().rev().find(|(n, _)| n == "picture") {
            errors.push(ParseEvent::ParseError {
                code: "html.parser.eof.open_elements".to_string(),
                message: "End of file seen and there were open elements.".to_string(),
                span: *span,
            });
        }

        errors.sort_by_key(|ev| match ev {
            ParseEvent::ParseError { span: Some(s), .. } => s.byte_start,
            _ => usize::MAX,
        });
        self.syntax_errors = errors.into();

        Ok(())
    }

    fn is_head_metadata_tag(tag_lc: &str) -> bool {
        matches!(
            tag_lc,
            "html"
                | "head"
                | "base"
                | "basefont"
                | "bgsound"
                | "link"
                | "meta"
                | "noframes"
                | "noscript"
                | "script"
                | "style"
                | "template"
                | "title"
        )
    }
}

fn is_defined_non_abstract_aria_role(role_lc: &str) -> bool {
    // Keep in sync with the VNU/Java role list: defined, non-abstract roles plus
    // the special-case "none"/"presentation" aliases.
    matches!(
        role_lc,
        "alert"
            | "alertdialog"
            | "application"
            | "article"
            | "banner"
            | "button"
            | "cell"
            | "checkbox"
            | "columnheader"
            | "combobox"
            | "complementary"
            | "contentinfo"
            | "definition"
            | "dialog"
            | "directory"
            | "document"
            | "feed"
            | "figure"
            | "form"
            | "grid"
            | "gridcell"
            | "group"
            | "heading"
            | "img"
            | "link"
            | "list"
            | "listbox"
            | "listitem"
            | "log"
            | "main"
            | "marquee"
            | "math"
            | "menu"
            | "menubar"
            | "menuitem"
            | "menuitemcheckbox"
            | "menuitemradio"
            | "navigation"
            | "none"
            | "note"
            | "option"
            | "presentation"
            | "progressbar"
            | "radio"
            | "radiogroup"
            | "region"
            | "row"
            | "rowgroup"
            | "rowheader"
            | "scrollbar"
            | "search"
            | "searchbox"
            | "separator"
            | "slider"
            | "spinbutton"
            | "status"
            | "switch"
            | "tab"
            | "table"
            | "tablist"
            | "tabpanel"
            | "term"
            | "textbox"
            | "timer"
            | "toolbar"
            | "tooltip"
            | "tree"
            | "treegrid"
            | "treeitem"
    )
}

fn is_potential_aria_role_token(token: &str) -> bool {
    let mut bytes = token.bytes();
    let Some(first) = bytes.next() else {
        return false;
    };
    if !first.is_ascii_alphabetic() {
        return false;
    }
    bytes.all(|b| b.is_ascii_alphanumeric() || b == b'-')
}

impl EventSource for Html5EverEventSource {
    fn source_name(&self) -> &str {
        &self.name
    }

    fn format(&self) -> InputFormat {
        InputFormat::Html
    }

    fn next_event(&mut self) -> Result<Option<ParseEvent>, ValidatorError> {
        self.ensure_parsed()?;

        loop {
            if let Some(staged) = self.staged.take() {
                if let Some(before) = staged.flush_syntax_errors_before {
                    if let Some(ev) = self.pop_ready_syntax_error(before) {
                        self.staged = Some(staged);
                        return Ok(Some(ev));
                    }
                }
                return Ok(Some(staged.event));
            }

            if self.walk_stack.is_empty() {
                return Ok(self.syntax_errors.pop_front());
            }

            self.staged = self.next_dom_event();
        }
    }
}

fn is_p_end_tag_implying_start_tag(name_lc: &str) -> bool {
    matches!(
        name_lc,
        "address"
            | "article"
            | "aside"
            | "blockquote"
            | "details"
            | "div"
            | "dl"
            | "fieldset"
            | "figcaption"
            | "figure"
            | "footer"
            | "form"
            | "h1"
            | "h2"
            | "h3"
            | "h4"
            | "h5"
            | "h6"
            | "header"
            | "hgroup"
            | "hr"
            | "main"
            | "menu"
            | "nav"
            | "ol"
            | "p"
            | "pre"
            | "section"
            | "table"
            | "ul"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HtmlEventSource;
    use html_inspector_core::InputFormat;

    fn collect(mut src: Html5EverEventSource) -> Vec<ParseEvent> {
        let mut out = Vec::new();
        while let Some(ev) = src.next_event().unwrap() {
            out.push(ev);
        }
        out
    }

    #[test]
    fn empty_input_parses_and_inserts_html_element() {
        let evs = collect(Html5EverEventSource::from_bytes("t", Vec::new()));
        assert!(evs
            .iter()
            .any(|e| matches!(e, ParseEvent::StartTag { name, .. } if name == "html")));
        assert!(!evs
            .iter()
            .any(|e| matches!(e, ParseEvent::ParseError { .. })));
    }

    #[test]
    fn next_event_is_stable_after_eof() {
        let mut src = Html5EverEventSource::from_bytes("t", Vec::new());
        while src.next_event().unwrap().is_some() {}
        assert!(src.next_event().unwrap().is_none());
        assert!(src.next_event().unwrap().is_none());
    }

    #[test]
    fn pop_ready_syntax_error_pops_only_when_ready() {
        let mut src = Html5EverEventSource::from_bytes("t", Vec::new());
        src.syntax_errors = VecDeque::from([
            ParseEvent::ParseError {
                code: "e1".to_string(),
                message: "m1".to_string(),
                span: Some(html_inspector_core::Span::new(5, 6, 1, 1)),
            },
            ParseEvent::ParseError {
                code: "e2".to_string(),
                message: "m2".to_string(),
                span: Some(html_inspector_core::Span::new(10, 11, 1, 1)),
            },
        ]);

        assert!(src.pop_ready_syntax_error(4).is_none());
        assert!(matches!(
            src.syntax_errors.front(),
            Some(ParseEvent::ParseError {
                span: Some(s), ..
            }) if s.byte_start == 5
        ));

        let popped = src.pop_ready_syntax_error(5).unwrap();
        assert!(matches!(
            popped,
            ParseEvent::ParseError {
                span: Some(s), ..
            } if s.byte_start == 5
        ));
        assert!(matches!(
            src.syntax_errors.front(),
            Some(ParseEvent::ParseError {
                span: Some(s), ..
            }) if s.byte_start == 10
        ));
    }

    #[test]
    fn input_bytes_are_dropped_after_first_parse() {
        let mut src = Html5EverEventSource::from_bytes("t", b"<p>hi</p>".to_vec());
        assert!(src.bytes.is_some());

        let _ = src.next_event().unwrap();
        assert!(src.root.is_some());
        assert!(src.bytes.is_none());
    }

    #[test]
    fn table_inserts_tbody() {
        let evs = collect(Html5EverEventSource::from_bytes(
            "t",
            b"<table><tr><td>1</td></tr></table>".to_vec(),
        ));
        let tags: Vec<String> = evs
            .iter()
            .filter_map(|e| match e {
                ParseEvent::StartTag { name, .. } => Some(format!("<{name}>")),
                ParseEvent::EndTag { name, .. } => Some(format!("</{name}>")),
                _ => None,
            })
            .collect();
        // HTML tree builder inserts <tbody>.
        assert!(tags.contains(&"<tbody>".to_string()));
        assert!(tags.contains(&"</tbody>".to_string()));
    }

    #[test]
    fn p_is_closed_before_div() {
        let evs = collect(Html5EverEventSource::from_bytes(
            "t",
            b"<p>one<div>two</div>".to_vec(),
        ));
        let mut seen_end_p = false;
        let mut seen_start_div = false;
        for e in evs {
            match e {
                ParseEvent::EndTag { name, .. } if name == "p" => seen_end_p = true,
                ParseEvent::StartTag { name, .. } if name == "div" => {
                    seen_start_div = true;
                    break;
                }
                _ => {}
            }
        }
        assert!(seen_end_p && seen_start_div, "expected </p> before <div>");
    }

    #[test]
    fn html_event_source_uses_html5ever_backend_when_enabled() {
        let mut src = HtmlEventSource::from_str("t", InputFormat::Html, "<p>hi</p>").unwrap();
        // HtmlEventSource::from_bytes selects html5ever for HTML when the feature is enabled.
        assert!(matches!(src, HtmlEventSource::Html5Ever(_)));

        let mut seen_p = false;
        while let Some(ev) = src.next_event().unwrap() {
            if matches!(ev, ParseEvent::StartTag { ref name, .. } if name == "p") {
                seen_p = true;
                break;
            }
        }
        assert!(seen_p);
    }

    #[test]
    fn image_element_aliases_to_img_with_spans() {
        let input = "<image src=x><img src=y>";
        let expected_image = input.find("<image").unwrap();
        let expected_img = input.find("<img").unwrap();

        let evs = collect(Html5EverEventSource::from_bytes(
            "t",
            input.as_bytes().to_vec(),
        ));
        let img_starts: Vec<usize> = evs
            .iter()
            .filter_map(|e| match e {
                ParseEvent::StartTag {
                    name,
                    span: Some(s),
                    ..
                } if name == "img" => Some(s.byte_start),
                _ => None,
            })
            .collect();
        assert!(
            img_starts.len() >= 2,
            "expected at least two <img> start tags with spans, got: {img_starts:?}"
        );
        assert_eq!(img_starts[0], expected_image);
        assert_eq!(img_starts[1], expected_img);
    }

    #[test]
    fn pop_span_for_tag_falls_back_to_ascii_lowercase() {
        let mut spans = HashMap::new();
        spans.insert(
            "foreignobject".to_string(),
            VecDeque::from([html_inspector_core::Span::new(1, 2, 1, 1)]),
        );

        let span = Html5EverEventSource::pop_span_for_tag(&mut spans, "foreignObject").unwrap();
        assert_eq!(span, html_inspector_core::Span::new(1, 2, 1, 1));
        assert!(Html5EverEventSource::pop_span_for_tag(&mut spans, "foreignObject").is_none());
    }

    #[test]
    fn pop_span_for_tag_falls_back_to_ascii_lowercase_with_utf8_prefix() {
        let mut spans = HashMap::new();
        spans.insert(
            "🦀a".to_string(),
            VecDeque::from([html_inspector_core::Span::new(1, 2, 1, 1)]),
        );

        let span = Html5EverEventSource::pop_span_for_tag(&mut spans, "🦀A").unwrap();
        assert_eq!(span, html_inspector_core::Span::new(1, 2, 1, 1));
    }

    #[test]
    fn pop_span_for_tag_prefers_exact_key_then_falls_back_to_ascii_lowercase() {
        let mut spans: HashMap<String, VecDeque<html_inspector_core::Span>> = HashMap::new();
        Html5EverEventSource::push_span_for_tag(
            &mut spans,
            "DIV",
            html_inspector_core::Span::new(1, 2, 1, 1),
        );
        Html5EverEventSource::push_span_for_tag(
            &mut spans,
            "div",
            html_inspector_core::Span::new(3, 4, 1, 3),
        );

        let first = Html5EverEventSource::pop_span_for_tag(&mut spans, "DIV").unwrap();
        assert_eq!(first, html_inspector_core::Span::new(1, 2, 1, 1));

        let second = Html5EverEventSource::pop_span_for_tag(&mut spans, "DIV").unwrap();
        assert_eq!(second, html_inspector_core::Span::new(3, 4, 1, 3));
        assert!(Html5EverEventSource::pop_span_for_tag(&mut spans, "DIV").is_none());
    }

    #[test]
    fn pop_span_for_tag_falls_back_when_exact_queue_is_empty() {
        let mut spans: HashMap<String, VecDeque<html_inspector_core::Span>> = HashMap::new();
        spans.insert("DIV".to_string(), VecDeque::new());
        spans.insert(
            "div".to_string(),
            VecDeque::from([html_inspector_core::Span::new(1, 2, 1, 1)]),
        );

        let span = Html5EverEventSource::pop_span_for_tag(&mut spans, "DIV").unwrap();
        assert_eq!(span, html_inspector_core::Span::new(1, 2, 1, 1));
    }

    #[test]
    fn pop_span_for_tag_does_not_fallback_when_tag_is_already_lowercase() {
        let mut spans = HashMap::new();
        spans.insert(
            "DIV".to_string(),
            VecDeque::from([html_inspector_core::Span::new(1, 2, 1, 1)]),
        );
        assert!(Html5EverEventSource::pop_span_for_tag(&mut spans, "div").is_none());
        assert_eq!(spans.get("DIV").unwrap().len(), 1);
    }

    #[test]
    fn pop_span_for_tag_returns_none_when_exact_queue_empty_and_no_fallback_key() {
        let mut spans: HashMap<String, VecDeque<html_inspector_core::Span>> = HashMap::new();
        spans.insert("DIV".to_string(), VecDeque::new());
        assert!(Html5EverEventSource::pop_span_for_tag(&mut spans, "DIV").is_none());
        assert!(spans.contains_key("DIV"));
        assert_eq!(spans.get("DIV").unwrap().len(), 0);
    }

    #[test]
    fn template_contents_are_walked() {
        let evs = collect(Html5EverEventSource::from_bytes(
            "t",
            b"<template><p>hi</p></template>".to_vec(),
        ));
        let template_start = evs
            .iter()
            .position(|e| matches!(e, ParseEvent::StartTag { name, .. } if name == "template"))
            .unwrap();
        let template_end = evs
            .iter()
            .position(|e| matches!(e, ParseEvent::EndTag { name, .. } if name == "template"))
            .unwrap();
        let p_start = evs
            .iter()
            .position(|e| matches!(e, ParseEvent::StartTag { name, .. } if name == "p"))
            .unwrap();
        let p_end = evs
            .iter()
            .position(|e| matches!(e, ParseEvent::EndTag { name, .. } if name == "p"))
            .unwrap();
        assert!(template_start < p_start);
        assert!(p_end < template_end);
    }

    #[test]
    fn doctype_nodes_and_misc_dom_nodes_are_ignored() {
        let evs = collect(Html5EverEventSource::from_bytes(
            "t",
            b"<!doctype html><p>hi</p>".to_vec(),
        ));
        assert!(evs
            .iter()
            .any(|e| matches!(e, ParseEvent::StartTag { name, .. } if name == "p")));
    }

    #[test]
    fn xmlns_double_prefix_is_parsed_as_attribute() {
        let evs = collect(Html5EverEventSource::from_bytes(
            "t",
            b"<svg xmlns:xmlns:foo=\"bar\"></svg>".to_vec(),
        ));
        let attrs: Vec<Attribute> = evs
            .iter()
            .filter_map(|e| match e {
                ParseEvent::StartTag { name, attrs, .. } if name == "svg" => Some(attrs.clone()),
                _ => None,
            })
            .flatten()
            .collect();
        let pairs: Vec<(String, Option<String>)> = attrs
            .iter()
            .map(|a| (a.name.clone(), a.value.clone()))
            .collect();
        assert!(
            pairs.iter().any(|(name, value)| {
                (name == "foo" || name == "xmlns:foo" || name == "xmlns:xmlns:foo")
                    && value.as_deref() == Some("bar")
            }),
            "attrs: {:?}",
            pairs
        );
    }

    #[test]
    fn normalize_double_xmlns_prefix_strips_repeated_prefixes() {
        let mut s = "xmlns:xmlns:foo".to_string();
        normalize_double_xmlns_prefix(&mut s);
        assert_eq!(s, "foo");
        normalize_double_xmlns_prefix(&mut s);
        assert_eq!(s, "foo");

        let mut s = "xmlns:xmlns:xmlns:bar".to_string();
        normalize_double_xmlns_prefix(&mut s);
        assert_eq!(s, "bar");

        let mut s = "xmlns:xmlns:xmlns:xmlns:baz".to_string();
        normalize_double_xmlns_prefix(&mut s);
        assert_eq!(s, "baz");
    }

    #[test]
    fn normalize_double_xmlns_prefix_preserves_utf8_after_prefix() {
        let mut s = "xmlns:xmlns:🦀a".to_string();
        normalize_double_xmlns_prefix(&mut s);
        assert_eq!(s, "🦀a");
    }

    #[test]
    fn normalize_double_xmlns_prefix_preserves_utf8_after_multiple_prefixes() {
        let mut s = "xmlns:xmlns:xmlns:🦀".to_string();
        normalize_double_xmlns_prefix(&mut s);
        assert_eq!(s, "🦀");
    }

    #[test]
    fn normalize_double_xmlns_prefix_can_strip_to_empty() {
        let mut s = "xmlns:xmlns:".to_string();
        normalize_double_xmlns_prefix(&mut s);
        assert_eq!(s, "");

        let mut s = "xmlns:xmlns:xmlns:".to_string();
        normalize_double_xmlns_prefix(&mut s);
        assert_eq!(s, "");
    }

    #[test]
    fn normalize_double_xmlns_prefix_does_not_strip_single_prefix() {
        let mut s = "xmlns:foo".to_string();
        normalize_double_xmlns_prefix(&mut s);
        assert_eq!(s, "xmlns:foo");

        let mut s = "xmlns:xmlnsfoo".to_string();
        normalize_double_xmlns_prefix(&mut s);
        assert_eq!(s, "xmlns:xmlnsfoo");

        let mut s = "xmlnS:xmlns:foo".to_string();
        normalize_double_xmlns_prefix(&mut s);
        assert_eq!(s, "xmlnS:xmlns:foo");
    }

    #[test]
    fn normalize_double_xmlns_prefix_does_not_strip_when_not_at_start() {
        let mut s = "foo xmlns:xmlns:bar".to_string();
        normalize_double_xmlns_prefix(&mut s);
        assert_eq!(s, "foo xmlns:xmlns:bar");
    }

    #[test]
    fn aria_role_token_lexing_rejects_empty_and_non_alpha_prefixes() {
        assert!(!is_potential_aria_role_token(""));
        assert!(!is_potential_aria_role_token("1button"));
        assert!(is_potential_aria_role_token("button"));
        assert!(is_potential_aria_role_token("tree-grid"));
        assert!(!is_potential_aria_role_token("böx"));
    }

    #[test]
    fn malformed_html_emits_parse_errors_in_prescan() {
        let evs = collect(Html5EverEventSource::from_bytes("t", b"<".to_vec()));
        assert!(evs
            .iter()
            .any(|e| matches!(e, ParseEvent::ParseError { .. })));
    }

    #[test]
    fn misplaced_table_text_emits_parse_error() {
        let evs = collect(Html5EverEventSource::from_bytes(
            "t",
            b"<table>hi</table>".to_vec(),
        ));
        let n = evs
            .iter()
            .filter(|e| matches!(
                e,
                ParseEvent::ParseError { code, .. } if code == "html.parser.misplaced_table_text"
            ))
            .count();
        assert_eq!(
            n, 1,
            "expected a single misplaced-table-text error, got: {evs:?}"
        );
    }

    #[test]
    fn select_in_table_emits_parse_error() {
        let evs = collect(Html5EverEventSource::from_bytes(
            "t",
            b"<table><select></select></table>".to_vec(),
        ));
        assert_eq!(
            count_parse_error_code(&evs, "html.parser.select_in_table"),
            1,
            "expected a single select-in-table error, got: {evs:?}"
        );
    }

    #[test]
    fn select_in_table_is_reported_once_across_multiple_occurrences() {
        let evs = collect(Html5EverEventSource::from_bytes(
            "t",
            b"<table><select></select></table><table><select></select></table>".to_vec(),
        ));
        assert_eq!(
            count_parse_error_code(&evs, "html.parser.select_in_table"),
            1
        );
    }

    #[test]
    fn input_in_table_emits_parse_error_when_not_in_table_cell() {
        let evs = collect(Html5EverEventSource::from_bytes(
            "t",
            b"<table><input></table>".to_vec(),
        ));
        assert_eq!(
            count_parse_error_code(&evs, "html.parser.input_in_table"),
            1,
            "expected a single input-in-table error, got: {evs:?}"
        );
    }

    #[test]
    fn input_in_table_is_not_emitted_inside_table_cell() {
        let evs = collect(Html5EverEventSource::from_bytes(
            "t",
            b"<table><tr><td><input></td></tr></table>".to_vec(),
        ));
        assert_eq!(
            count_parse_error_code(&evs, "html.parser.input_in_table"),
            0,
            "did not expect input-in-table error inside <td>, got: {evs:?}"
        );
    }

    #[test]
    fn input_in_table_is_reported_once_across_multiple_occurrences() {
        let evs = collect(Html5EverEventSource::from_bytes(
            "t",
            b"<table><input></table><table><input></table>".to_vec(),
        ));
        assert_eq!(
            count_parse_error_code(&evs, "html.parser.input_in_table"),
            1
        );
    }

    #[test]
    fn nested_table_emits_parse_error_when_not_in_table_cell() {
        let evs = collect(Html5EverEventSource::from_bytes(
            "t",
            b"<table><table></table></table>".to_vec(),
        ));
        assert_eq!(
            count_parse_error_code(&evs, "html.parser.nested_table"),
            1,
            "expected a single nested-table error, got: {evs:?}"
        );
    }

    #[test]
    fn nested_table_is_allowed_inside_table_cell() {
        let evs = collect(Html5EverEventSource::from_bytes(
            "t",
            b"<table><tr><td><table></table></td></tr></table>".to_vec(),
        ));
        assert_eq!(
            count_parse_error_code(&evs, "html.parser.nested_table"),
            0,
            "did not expect nested-table error inside <td>, got: {evs:?}"
        );
    }

    #[test]
    fn eof_open_elements_span_uses_picture_start_tag_span() {
        let input = "<picture>";
        let expected_span = input.find("<picture").unwrap();
        let evs = collect(Html5EverEventSource::from_bytes(
            "t",
            input.as_bytes().to_vec(),
        ));
        let span_start = evs.iter().find_map(|e| match e {
            ParseEvent::ParseError {
                code,
                span: Some(s),
                ..
            } if code == "html.parser.eof.open_elements" => Some(s.byte_start),
            _ => None,
        });
        assert_eq!(span_start, Some(expected_span));
    }

    #[test]
    fn eof_open_elements_span_uses_last_picture_start_tag_span_when_multiple_pictures_open() {
        let input = "<picture><picture>";
        let expected_span = input.rfind("<picture").unwrap();
        let evs = collect(Html5EverEventSource::from_bytes(
            "t",
            input.as_bytes().to_vec(),
        ));
        let span_start = evs.iter().find_map(|e| match e {
            ParseEvent::ParseError {
                code,
                span: Some(s),
                ..
            } if code == "html.parser.eof.open_elements" => Some(s.byte_start),
            _ => None,
        });
        assert_eq!(span_start, Some(expected_span));
    }

    #[test]
    fn syntax_errors_are_returned_in_byte_order() {
        let input = "<head><noscript>x</noscript></head><table>hi</table><picture>";
        let expected_x = input.find('x').unwrap();
        let expected_hi = input.find("hi").unwrap();
        let expected_picture = input.find("<picture").unwrap();

        let evs = collect(Html5EverEventSource::from_bytes(
            "t",
            input.as_bytes().to_vec(),
        ));
        let spans: Vec<(String, usize)> = evs
            .iter()
            .filter_map(|e| match e {
                ParseEvent::ParseError {
                    code,
                    span: Some(s),
                    ..
                } => Some((code.clone(), s.byte_start)),
                _ => None,
            })
            .collect();

        assert!(
            spans
                .iter()
                .any(|(code, _)| code == "html.parser.nonspace_in_noscript_head"),
            "missing nonspace_in_noscript_head: {spans:?}"
        );
        assert!(
            spans
                .iter()
                .any(|(code, _)| code == "html.parser.misplaced_table_text"),
            "missing misplaced_table_text: {spans:?}"
        );
        assert!(
            spans
                .iter()
                .any(|(code, _)| code == "html.parser.eof.open_elements"),
            "missing eof.open_elements: {spans:?}"
        );

        let only_starts: Vec<usize> = spans.iter().map(|(_, s)| *s).collect();
        assert!(
            only_starts.windows(2).all(|w| w[0] <= w[1]),
            "parse errors not ordered by byte_start: {spans:?}"
        );

        assert!(
            only_starts.contains(&expected_x),
            "expected x span missing: {spans:?}"
        );
        assert!(
            only_starts.contains(&expected_hi),
            "expected hi span missing: {spans:?}"
        );
        assert!(
            only_starts.contains(&expected_picture),
            "expected picture span missing: {spans:?}"
        );
    }

    #[test]
    fn invalid_utf8_bytes_are_lossily_decoded_for_html5ever() {
        let bytes = vec![b'<', b'p', b'>', 0xFF, b'<', b'/', b'p', b'>'];
        let evs = collect(Html5EverEventSource::from_bytes("t", bytes));
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::Text { text, .. } if text.contains('\u{FFFD}')
        )));
    }

    #[test]
    fn foreign_object_spans_match_start_tag_bytes() {
        let input = "<svg><foreignObject></foreignObject></svg>";
        let expected_start = input.find("<foreignObject>").unwrap();
        let evs = collect(Html5EverEventSource::from_bytes(
            "t",
            input.as_bytes().to_vec(),
        ));
        let span_start = evs.iter().find_map(|e| match e {
            ParseEvent::StartTag {
                name,
                span: Some(s),
                ..
            } if name.eq_ignore_ascii_case("foreignobject") => Some(s.byte_start),
            _ => None,
        });
        assert_eq!(span_start, Some(expected_start));
    }

    #[test]
    fn end_tag_spans_match_end_tag_bytes() {
        let input = "<div></div>";
        let expected_start = input.find("</div>").unwrap();
        let evs = collect(Html5EverEventSource::from_bytes(
            "t",
            input.as_bytes().to_vec(),
        ));
        let span_start = evs.iter().find_map(|e| match e {
            ParseEvent::EndTag {
                name,
                span: Some(s),
            } if name == "div" => Some(s.byte_start),
            _ => None,
        });
        assert_eq!(span_start, Some(expected_start));
    }

    #[test]
    fn prescan_parse_errors_are_emitted_before_later_tags() {
        let evs = collect(Html5EverEventSource::from_bytes(
            "t",
            b"<div></span><p>hi</p>".to_vec(),
        ));
        let first_error = evs
            .iter()
            .position(|e| matches!(e, ParseEvent::ParseError { .. }));
        let first_p = evs
            .iter()
            .position(|e| matches!(e, ParseEvent::StartTag { name, .. } if name == "p"));
        assert!(
            matches!((first_error, first_p), (Some(e), Some(p)) if e < p),
            "expected a prescan parse error before <p>, got: {evs:?}"
        );
    }

    #[test]
    fn aria_role_unrecognized_token_is_only_emitted_for_multi_token_values() {
        let evs = collect(Html5EverEventSource::from_bytes(
            "t",
            b"<div role=\"button frobnicate\"></div>".to_vec(),
        ));
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "aria.role.unrecognized_token"
        )));

        let evs = collect(Html5EverEventSource::from_bytes(
            "t",
            b"<div role=\"frobnicate\"></div>".to_vec(),
        ));
        assert!(!evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "aria.role.unrecognized_token"
        )));
    }

    #[test]
    fn aria_role_bad_value_suppresses_unrecognized_token_errors() {
        let evs = collect(Html5EverEventSource::from_bytes(
            "t",
            b"<div role=\"button 1\"></div>".to_vec(),
        ));
        assert!(evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "html.attr.role.bad_value"
        )));
        assert!(!evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "aria.role.unrecognized_token"
        )));
    }

    #[test]
    fn aria_role_defined_tokens_are_checked_case_insensitively() {
        let evs = collect(Html5EverEventSource::from_bytes(
            "t",
            b"<div role=\"BUTTON checkbox\"></div>".to_vec(),
        ));
        assert!(!evs.iter().any(|e| matches!(
            e,
            ParseEvent::ParseError { code, .. } if code == "aria.role.unrecognized_token"
        )));
    }

    #[test]
    fn pop_span_for_tag_falls_back_to_lowercase_when_needed() {
        let mut spans: HashMap<String, std::collections::VecDeque<html_inspector_core::Span>> =
            HashMap::new();
        let mut q = std::collections::VecDeque::new();
        q.push_back(html_inspector_core::Span::new(1, 2, 3, 4));
        spans.insert("div".to_string(), q);

        let got = Html5EverEventSource::pop_span_for_tag(&mut spans, "DIV");
        assert_eq!(got, Some(html_inspector_core::Span::new(1, 2, 3, 4)));
        assert!(spans.get("div").unwrap().is_empty());
    }

    #[test]
    fn pop_ready_syntax_error_respects_byte_start_ordering() {
        let mut src = Html5EverEventSource::from_bytes("t", Vec::new());
        src.syntax_errors.push_back(ParseEvent::ParseError {
            code: "e".to_string(),
            message: "m".to_string(),
            span: Some(html_inspector_core::Span::new(10, 11, 1, 1)),
        });

        assert!(src.pop_ready_syntax_error(9).is_none());
        assert!(matches!(
            src.syntax_errors.front(),
            Some(ParseEvent::ParseError { .. })
        ));
        assert!(matches!(
            src.pop_ready_syntax_error(10),
            Some(ParseEvent::ParseError { .. })
        ));
        assert!(src.syntax_errors.is_empty());
    }

    #[test]
    fn pop_ready_syntax_error_does_not_pop_unspanned_errors() {
        let mut src = Html5EverEventSource::from_bytes("t", Vec::new());
        src.syntax_errors.push_back(ParseEvent::ParseError {
            code: "e".to_string(),
            message: "m".to_string(),
            span: None,
        });

        assert!(src.pop_ready_syntax_error(usize::MAX).is_none());
        assert!(matches!(
            src.syntax_errors.front(),
            Some(ParseEvent::ParseError { span: None, .. })
        ));
    }

    #[test]
    fn pop_ready_syntax_error_does_not_pop_non_parse_errors() {
        let mut src = Html5EverEventSource::from_bytes("t", Vec::new());
        src.syntax_errors.push_back(ParseEvent::Comment {
            text: "x".to_string(),
            span: Some(html_inspector_core::Span::new(0, 1, 1, 1)),
        });

        assert!(src.pop_ready_syntax_error(usize::MAX).is_none());
        assert!(matches!(
            src.syntax_errors.front(),
            Some(ParseEvent::Comment { .. })
        ));
    }

    fn count_parse_error_code(evs: &[ParseEvent], expected: &str) -> usize {
        let mut n = 0usize;
        for e in evs {
            if let ParseEvent::ParseError { code, .. } = e {
                if code == expected {
                    n += 1;
                }
            }
        }
        n
    }

    #[test]
    fn nonspace_after_body_is_reported_once() {
        let evs = collect(Html5EverEventSource::from_bytes(
            "t",
            b"<!doctype html><html><head></head><body></body>x<!--c-->y</html>".to_vec(),
        ));
        assert_eq!(
            count_parse_error_code(&evs, "html.parser.nonspace_after_body"),
            1
        );
    }

    #[test]
    fn misplaced_table_text_is_reported_once_across_tables() {
        let evs = collect(Html5EverEventSource::from_bytes(
            "t",
            b"<table>a</table><table>b</table>".to_vec(),
        ));
        assert_eq!(
            count_parse_error_code(&evs, "html.parser.misplaced_table_text"),
            1
        );
    }

    #[test]
    fn nonspace_in_noscript_head_is_reported_once_across_noscripts() {
        let evs = collect(Html5EverEventSource::from_bytes(
            "t",
            b"<head><noscript>a</noscript><noscript>b</noscript></head>".to_vec(),
        ));
        assert_eq!(
            count_parse_error_code(&evs, "html.parser.nonspace_in_noscript_head"),
            1
        );
    }

    #[test]
    fn whitespace_only_table_text_does_not_trigger_misplaced_table_text_error() {
        let evs = collect(Html5EverEventSource::from_bytes(
            "t",
            b"<table>\n \t</table>".to_vec(),
        ));
        assert_eq!(
            count_parse_error_code(&evs, "html.parser.misplaced_table_text"),
            0
        );
    }
}
