use std::borrow::Cow;
use std::cmp::{Ordering, Reverse};
use std::fmt;

use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidatorError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("invalid input: {0}")]
    InvalidInput(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum InputFormat {
    Html,
    Xhtml,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Span {
    pub byte_start: usize,
    pub byte_end: usize,
    pub line: u32,
    pub col: u32,
}

impl Span {
    pub fn new(byte_start: usize, byte_end: usize, line: u32, col: u32) -> Self {
        Self {
            byte_start,
            byte_end,
            line,
            col,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Default)]
pub enum Severity {
    Error,
    Warning,
    #[default]
    Info,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SeverityProfile {
    /// Use severities exactly as emitted by rule packs.
    Conformance,
    /// Normalize severities to reflect runtime risk (DOM/interaction instability > spec purity).
    #[default]
    Risk,
}

impl Severity {
    fn rank(self) -> u8 {
        match self {
            Severity::Error => 3,
            Severity::Warning => 2,
            Severity::Info => 1,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub enum Category {
    Html,
    Aria,
    I18n,
    Internal,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct MessageNote {
    pub message: String,
    pub span: Option<Span>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Message {
    pub code: String,
    pub severity: Severity,
    pub category: Category,
    pub message: String,
    pub span: Option<Span>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub notes: Vec<MessageNote>,
    #[serde(skip)]
    order: u64,
}

impl Message {
    pub fn new(
        code: impl Into<String>,
        severity: Severity,
        category: Category,
        message: impl Into<String>,
        span: Option<Span>,
    ) -> Self {
        Self {
            code: code.into(),
            severity,
            category,
            message: message.into(),
            span,
            notes: Vec::new(),
            order: 0,
        }
    }
}

pub trait MessageSink {
    fn push(&mut self, msg: Message);
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Report {
    pub source_name: String,
    pub messages: Vec<Message>,
}

impl Report {
    pub fn counts(&self) -> (usize, usize, usize) {
        let (mut errors, mut warnings, mut infos) = (0usize, 0usize, 0usize);
        for msg in &self.messages {
            match msg.severity {
                Severity::Error => errors += 1,
                Severity::Warning => warnings += 1,
                Severity::Info => infos += 1,
            }
        }
        (errors, warnings, infos)
    }
}

#[derive(Clone, Debug, Default)]
pub struct Config {
    pub ignore_missing_lang: bool,
    pub message_order: MessageOrder,
    pub also_check_css: bool,
    /// Controls how severities emitted by rule packs are interpreted.
    pub severity_profile: SeverityProfile,
    /// Minimum message severity to include in output.
    ///
    /// Rules that can never emit at/above this level are skipped entirely.
    pub min_severity: Severity,
    pub base_uri: Option<String>,
    pub css_profile: Option<String>,
    pub css_medium: Option<String>,
    pub css_warning: Option<String>,
    /// Content-Security-Policy header value (best-effort enforcement/warnings).
    pub csp_header: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum MessageOrder {
    /// Preserve the order messages are emitted by rules.
    #[default]
    Emit,
    /// Sort by source span (start position) and severity.
    BySpan,
    /// Group by severity (errors, then warnings, then infos), preserving emission order within each group.
    Vnu,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Attribute {
    pub name: String,
    pub value: Option<String>,
    pub span: Option<Span>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseEvent {
    StartTag {
        name: String,
        attrs: Vec<Attribute>,
        self_closing: bool,
        span: Option<Span>,
    },
    EndTag {
        name: String,
        span: Option<Span>,
    },
    Text {
        text: String,
        span: Option<Span>,
    },
    ProcessingInstruction {
        target: String,
        data: String,
        span: Option<Span>,
    },
    Comment {
        text: String,
        span: Option<Span>,
    },
    Doctype {
        name: Option<String>,
        public_id: Option<String>,
        system_id: Option<String>,
        span: Option<Span>,
    },
    ParseError {
        code: String,
        message: String,
        span: Option<Span>,
    },
}

pub trait EventSource {
    fn source_name(&self) -> &str;
    fn format(&self) -> InputFormat;
    fn next_event(&mut self) -> Result<Option<ParseEvent>, ValidatorError>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Interest(u8);

impl Interest {
    pub const NONE: Interest = Interest(0);
    pub const START_TAG: Interest = Interest(1 << 0);
    pub const END_TAG: Interest = Interest(1 << 1);
    pub const TEXT: Interest = Interest(1 << 2);
    pub const COMMENT: Interest = Interest(1 << 3);
    pub const DOCTYPE: Interest = Interest(1 << 4);
    pub const PARSE_ERROR: Interest = Interest(1 << 5);
    pub const FINISH: Interest = Interest(1 << 6);
    pub const PROCESSING_INSTRUCTION: Interest = Interest(1 << 7);
    pub const ALL: Interest = Interest(0xFF);

    pub fn contains(self, other: Interest) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl std::ops::BitOr for Interest {
    type Output = Interest;
    fn bitor(self, rhs: Interest) -> Self::Output {
        Interest(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for Interest {
    fn bitor_assign(&mut self, rhs: Interest) {
        self.0 |= rhs.0;
    }
}

impl std::ops::BitAnd for Interest {
    type Output = Interest;
    fn bitand(self, rhs: Interest) -> Self::Output {
        Interest(self.0 & rhs.0)
    }
}

impl std::ops::BitAndAssign for Interest {
    fn bitand_assign(&mut self, rhs: Interest) {
        self.0 &= rhs.0;
    }
}

impl fmt::Display for Interest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Interest({:#04x})", self.0)
    }
}

pub trait Rule: Send {
    fn id(&self) -> &'static str;
    fn max_severity(&self) -> Severity {
        Severity::Error
    }
    fn interest(&self) -> Interest {
        Interest::ALL
    }
    fn init(&mut self, _ctx: &ValidationContext) {}
    fn on_event(
        &mut self,
        _event: &ParseEvent,
        _ctx: &mut ValidationContext,
        _out: &mut dyn MessageSink,
    ) {
    }
    fn on_finish(&mut self, _ctx: &mut ValidationContext, _out: &mut dyn MessageSink) {}
}

#[derive(Default)]
pub struct RuleSet {
    rules: Vec<Box<dyn Rule>>,
}

impl RuleSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push<R: Rule + 'static>(mut self, rule: R) -> Self {
        self.rules.push(Box::new(rule));
        self
    }

    pub fn extend(mut self, rules: Vec<Box<dyn Rule>>) -> Self {
        self.rules.extend(rules);
        self
    }

    pub fn merge(mut self, mut other: RuleSet) -> Self {
        self.rules.append(&mut other.rules);
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct DocumentFlags {
    pub has_doctype: bool,
    pub section: DocumentSection,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum DocumentSection {
    #[default]
    Head,
    Body,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ForeignContentNamespace {
    #[default]
    Html,
    Svg,
    Math,
}

#[derive(Clone, Debug)]
pub struct ValidationContext {
    pub config: Config,
    pub format: InputFormat,
    pub document: DocumentFlags,
    open_elements: Vec<String>,
    foreign_insertion_stack: Vec<ForeignContentNamespace>,
    had_fatal_parse_error: bool,
}

impl ValidationContext {
    pub fn new(config: Config, format: InputFormat) -> Self {
        Self {
            config,
            format,
            document: DocumentFlags::default(),
            open_elements: Vec::new(),
            foreign_insertion_stack: Vec::new(),
            had_fatal_parse_error: false,
        }
    }

    pub fn had_fatal_parse_error(&self) -> bool {
        self.had_fatal_parse_error
    }

    #[inline]
    pub fn name_is(&self, actual: &str, expected: &str) -> bool {
        match self.format {
            InputFormat::Html => actual.eq_ignore_ascii_case(expected),
            InputFormat::Xhtml => actual == expected,
        }
    }

    #[inline]
    pub fn attr_value<'a>(&self, attrs: &'a [Attribute], needle: &str) -> Option<&'a str> {
        attrs.iter().find_map(|attr| {
            if self.name_is(&attr.name, needle) {
                attr.value.as_deref()
            } else {
                None
            }
        })
    }

    #[inline]
    pub fn has_attr(&self, attrs: &[Attribute], needle: &str) -> bool {
        attrs.iter().any(|a| self.name_is(&a.name, needle))
    }

    pub fn foreign_insertion_namespace(&self) -> ForeignContentNamespace {
        self.foreign_insertion_stack
            .last()
            .copied()
            .unwrap_or_default()
    }

    pub fn open_elements(&self) -> &[String] {
        &self.open_elements
    }

    pub fn current_parent(&self) -> Option<&str> {
        self.open_elements.last().map(String::as_str)
    }

    pub fn has_ancestor(&self, name: &str) -> bool {
        self.open_elements.iter().any(|n| self.name_is(n, name))
    }

    fn foreign_insertion_after(
        &self,
        prev: ForeignContentNamespace,
        element_name: &str,
        attrs: &[Attribute],
    ) -> ForeignContentNamespace {
        match prev {
            ForeignContentNamespace::Html if self.name_is(element_name, "svg") => {
                ForeignContentNamespace::Svg
            }
            ForeignContentNamespace::Html if self.name_is(element_name, "math") => {
                ForeignContentNamespace::Math
            }
            ForeignContentNamespace::Html => ForeignContentNamespace::Html,
            ForeignContentNamespace::Svg
                if self.name_is(element_name, "foreignobject")
                    || self.name_is(element_name, "desc")
                    || self.name_is(element_name, "title") =>
            {
                ForeignContentNamespace::Html
            }
            ForeignContentNamespace::Svg => ForeignContentNamespace::Svg,
            ForeignContentNamespace::Math
                if self.name_is(element_name, "annotation-xml")
                    && is_mathml_annotation_xml_html_integration_point(self, attrs) =>
            {
                ForeignContentNamespace::Html
            }
            other => other,
        }
    }

    fn push_open_element_owned(&mut self, name: String, attrs: &[Attribute]) {
        let prev = self.foreign_insertion_namespace();
        let next = self.foreign_insertion_after(prev, &name, attrs);
        self.open_elements.push(name);
        self.foreign_insertion_stack.push(next);
    }

    #[inline]
    fn should_push_open_element(&self, name: &str, self_closing: bool) -> bool {
        // HTML void elements never push onto the open-elements stack.
        // In HTML, `/>` on non-void elements is a parse error and the slash is ignored.
        match self.format {
            InputFormat::Html => !is_void_html_element(name),
            InputFormat::Xhtml => !self_closing,
        }
    }

    #[inline]
    fn on_start_tag_common(&mut self, name: &str, self_closing: bool) -> bool {
        self.update_section(name);
        self.should_push_open_element(name, self_closing)
    }

    #[cfg(test)]
    fn on_start_tag(&mut self, name: &str, self_closing: bool) {
        if self.on_start_tag_common(name, self_closing) {
            self.push_open_element_owned(name.to_string(), &[]);
        }
    }

    fn on_start_tag_owned(&mut self, name: String, attrs: &[Attribute], self_closing: bool) {
        if self.on_start_tag_common(&name, self_closing) {
            self.push_open_element_owned(name, attrs);
        }
    }

    fn on_end_tag(&mut self, name: &str) {
        let Some(pos) = self
            .open_elements
            .iter()
            .rposition(|n| self.name_is(n, name))
        else {
            return;
        };
        self.open_elements.truncate(pos);
        self.foreign_insertion_stack.truncate(pos);
    }

    fn update_section(&mut self, name: &str) {
        if self.name_is(name, "body") {
            self.document.section = DocumentSection::Body;
        } else if self.name_is(name, "head") {
            self.document.section = DocumentSection::Head;
        } else if self.document.section == DocumentSection::Head
            && !is_metadata_element(self.format, name)
        {
            self.document.section = DocumentSection::Body;
        }
    }
}

fn is_metadata_element(format: InputFormat, name: &str) -> bool {
    const META: [&str; 10] = [
        "base", "link", "meta", "style", "title", "noscript", "script", "template", "head", "html",
    ];

    match format {
        InputFormat::Html => ascii_case_insensitive_contains(&META, name),
        InputFormat::Xhtml => META.contains(&name),
    }
}

fn is_mathml_annotation_xml_html_integration_point(
    ctx: &ValidationContext,
    attrs: &[Attribute],
) -> bool {
    let Some(encoding) = attrs.iter().find_map(|attr| {
        if ctx.name_is(&attr.name, "encoding") {
            attr.value.as_deref()
        } else {
            None
        }
    }) else {
        return false;
    };

    encoding.eq_ignore_ascii_case("text/html")
        || encoding.eq_ignore_ascii_case("application/xhtml+xml")
}

#[inline]
fn ascii_case_insensitive_contains(list: &[&str], s: &str) -> bool {
    if s.bytes().any(|b| b.is_ascii_uppercase()) {
        list.iter().any(|&el| el.eq_ignore_ascii_case(s))
    } else {
        list.contains(&s)
    }
}

#[inline]
pub fn ascii_lowercase_cow_if_needed(s: &str) -> Cow<'_, str> {
    let Some(first_upper) = s.bytes().position(|b| b.is_ascii_uppercase()) else {
        return Cow::Borrowed(s);
    };

    // `first_upper` is always a UTF-8 boundary: ASCII uppercase bytes cannot appear as UTF-8
    // continuation bytes (0x80..=0xBF).
    let mut out = s.to_owned();
    out[first_upper..].make_ascii_lowercase();
    Cow::Owned(out)
}

#[inline]
pub fn starts_with_ascii_ci(s: &str, prefix: &str) -> bool {
    let sb = s.as_bytes();
    let pb = prefix.as_bytes();
    sb.len() >= pb.len() && sb[..pb.len()].eq_ignore_ascii_case(pb)
}

#[inline]
pub fn ends_with_ascii_ci(s: &str, suffix: &str) -> bool {
    let sb = s.as_bytes();
    let suf = suffix.as_bytes();
    sb.len() >= suf.len() && sb[sb.len() - suf.len()..].eq_ignore_ascii_case(suf)
}

pub fn is_void_html_element(name: &str) -> bool {
    const VOID: [&str; 14] = [
        "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param",
        "source", "track", "wbr",
    ];

    ascii_case_insensitive_contains(&VOID, name)
}
#[derive(Default)]
struct CollectingSink {
    order: u64,
    messages: Vec<Message>,
}

#[inline]
fn is_extra_html_attribute_message(message: &str) -> bool {
    // Intentionally diverge from vnu.jar: treat extra/disallowed HTML attributes as warnings.
    //
    // Matches common vnu-style diagnostics:
    // - Attribute “x” not allowed on element “y” at this point.
    // - Attribute “xmlns:foo” not allowed here.
    message.starts_with("Attribute “")
        && (message.contains("” not allowed on element “")
            || message.contains("” not allowed here."))
}

impl MessageSink for CollectingSink {
    fn push(&mut self, mut msg: Message) {
        if msg.category == Category::Html
            && msg.severity == Severity::Error
            && is_extra_html_attribute_message(&msg.message)
        {
            msg.severity = Severity::Warning;
        }
        msg.order = self.order;
        self.order += 1;
        self.messages.push(msg);
    }
}

pub fn validate_events(
    mut source: impl EventSource,
    mut rules: RuleSet,
    config: Config,
) -> Result<Report, ValidatorError> {
    let format = source.format();
    let message_order = config.message_order;
    let mut ctx = ValidationContext::new(config, format);

    let min_sev_rank = ctx.config.min_severity.rank();
    rules
        .rules
        .retain(|r| r.max_severity().rank() >= min_sev_rank);

    for rule in &mut rules.rules {
        // Ensure rule IDs are exercised in normal validation runs (useful for coverage and
        // for catching accidental panics in rule metadata).
        let _ = rule.id();
        rule.init(&ctx);
    }

    let mut sink = CollectingSink::default();

    let mut fatal_parse_error_pos: Option<usize> = None;

    while let Some(event) = source.next_event()? {
        // Once a fatal parse error with a source position has occurred, skip processing any
        // subsequent events that are positioned after it. This prevents later events (e.g. a
        // duplicate-ID) from triggering additional messages that point back to earlier spans.
        if let Some(fatal_pos) = fatal_parse_error_pos.filter(|&p| p != usize::MAX) {
            if parse_event_span_start(&event).is_some_and(|pos| pos > fatal_pos) {
                continue;
            }
        }

        let kind = match &event {
            ParseEvent::ParseError { code, span, .. } => {
                if code == "html.parser.cannot_recover" {
                    ctx.had_fatal_parse_error = true;
                    fatal_parse_error_pos
                        .get_or_insert_with(|| span.as_ref().map_or(usize::MAX, |s| s.byte_start));
                }
                Interest::PARSE_ERROR
            }
            ParseEvent::Doctype { .. } => {
                ctx.document.has_doctype = true;
                Interest::DOCTYPE
            }
            _ => event_interest(&event),
        };

        for rule in &mut rules.rules {
            let interest = rule.interest();
            if interest.contains(kind) {
                rule.on_event(&event, &mut ctx, &mut sink);
            }
        }

        match event {
            ParseEvent::StartTag {
                name,
                attrs,
                self_closing,
                ..
            } => ctx.on_start_tag_owned(name, &attrs, self_closing),
            ParseEvent::EndTag { name, .. } => ctx.on_end_tag(&name),
            _ => (),
        }
    }

    for rule in &mut rules.rules {
        rule.on_finish(&mut ctx, &mut sink);
    }

    if ctx.config.severity_profile == SeverityProfile::Risk {
        apply_risk_severity_profile(&mut sink.messages);
    }

    sink.messages.retain(|m| m.severity.rank() >= min_sev_rank);

    match message_order {
        MessageOrder::Emit => {}
        // The comparison functions include `Message.order`, which is unique per emitted message.
        // That makes the sort order a strict total order, so stability isn't needed.
        MessageOrder::BySpan => sink.messages.sort_unstable_by(compare_messages),
        MessageOrder::Vnu => sink.messages.sort_unstable_by(compare_messages_vnu),
    }

    // Mimic vnu.jar’s “Cannot recover…” behavior by dropping any messages that occur after the
    // fatal parse-error position in the original input. We filter by spans (not emission order)
    // to avoid losing messages from parsers that can yield events slightly out of source order.
    if let Some(fatal_pos) = fatal_parse_error_pos {
        sink.messages
            .retain(|m| m.span.is_some_and(|s| s.byte_start <= fatal_pos));
    }

    Ok(Report {
        source_name: source.source_name().to_string(),
        messages: sink.messages,
    })
}

fn apply_risk_severity_profile(messages: &mut [Message]) {
    for m in messages {
        m.severity = risk_severity_for_message(m);
    }
}

fn risk_severity_for_message(m: &Message) -> Severity {
    match m.category {
        Category::Internal => m.severity,
        Category::Aria => m.severity,
        Category::I18n => m.severity,
        Category::Html => risk_severity_for_html_message(m),
    }
}

fn risk_severity_for_html_message(m: &Message) -> Severity {
    let code = m.code.as_str();

    // Keep CSP diagnostics as authored: they intentionally mix warnings and errors.
    if code.starts_with("html.meta.csp.") {
        return m.severity;
    }

    // DOM tree corruption / parser recovery.
    if code.starts_with("html.parser.") || code.starts_with("html.parse.") {
        return Severity::Error;
    }
    if code.starts_with("html.doctype.") {
        return Severity::Error;
    }

    // Determinism hazards.
    if code == "html.id.duplicate" {
        return Severity::Error;
    }
    if code == "html.id.duplicate.first" {
        return Severity::Info;
    }

    // Broken interactive semantics.
    if code == "html.a.href.button_descendant"
        || code == "html.css.error"
        || code == "html.iframe.sandbox.invalid_token"
        || code == "html.svg.a.nested_in_a"
        || code == "html.svg.xmlns.default.bad_value"
        || code == "html.svg.xmlns.xlink.bad_value"
        || code.starts_with("html.label.")
        || code.starts_with("html.role_button.")
        || code.starts_with("html.form_attribute.")
        || code.starts_with("html.input.checkbox.role_button.")
        || code.starts_with("html.img.role.")
        || code.starts_with("html.td.role.")
        || code.starts_with("html.area.map_ancestor.")
        || code.starts_with("html.img.usemap.")
        || code.starts_with("html.object.usemap.")
        || code.starts_with("html.img.ismap.")
    {
        return Severity::Error;
    }

    if code == "html.iframe.sandbox.duplicate_token" {
        return Severity::Info;
    }

    // Framework-generated markers and custom element noise.
    if is_framework_noise_html_message(m) {
        return Severity::Info;
    }

    // Spec-purity checks: keep as low-signal by default.
    if is_spec_purism_html_code(code) {
        return Severity::Info;
    }

    // Default: downgrade conformance errors to warnings; preserve warning/info.
    match m.severity {
        Severity::Error => Severity::Warning,
        other => other,
    }
}

fn is_spec_purism_html_code(code: &str) -> bool {
    code.contains("microdata") || code.contains("rdfa") || code.contains("rdfa_lite")
}

fn is_framework_noise_html_message(m: &Message) -> bool {
    if m.code.starts_with("html.unknown_element.") {
        if let Some(name) = first_curly_quoted_token(&m.message) {
            if name.contains('-') {
                return true;
            }
        }
    }

    // Attribute diagnostics include the attribute name as the first quoted token.
    if m.message.starts_with("Attribute “") {
        if let Some(attr) = first_curly_quoted_token(&m.message) {
            return is_framework_attribute_name(attr);
        }
    }

    false
}

fn is_framework_attribute_name(attr: &str) -> bool {
    // Angular
    attr.starts_with("_ngcontent-")
        || attr.starts_with("_nghost-")
        || attr.starts_with("ng-")
        || attr.starts_with("ng-reflect-")
        // Vue / Alpine / HTMX (common non-data-* markers)
        || attr.starts_with("v-")
        || attr.starts_with("x-")
        || attr.starts_with("hx-")
}

fn first_curly_quoted_token(message: &str) -> Option<&str> {
    let start = message.find('“')? + '“'.len_utf8();
    let end = message[start..].find('”')? + start;
    Some(&message[start..end])
}

#[inline]
fn parse_event_span_start(event: &ParseEvent) -> Option<usize> {
    match event {
        ParseEvent::StartTag { span, .. }
        | ParseEvent::EndTag { span, .. }
        | ParseEvent::Text { span, .. }
        | ParseEvent::ProcessingInstruction { span, .. }
        | ParseEvent::Comment { span, .. }
        | ParseEvent::Doctype { span, .. }
        | ParseEvent::ParseError { span, .. } => span.as_ref().map(|s| s.byte_start),
    }
}

#[inline]
fn event_interest(event: &ParseEvent) -> Interest {
    match event {
        ParseEvent::StartTag { .. } => Interest::START_TAG,
        ParseEvent::EndTag { .. } => Interest::END_TAG,
        ParseEvent::Text { .. } => Interest::TEXT,
        ParseEvent::ProcessingInstruction { .. } => Interest::PROCESSING_INSTRUCTION,
        ParseEvent::Comment { .. } => Interest::COMMENT,
        ParseEvent::Doctype { .. } => Interest::DOCTYPE,
        ParseEvent::ParseError { .. } => Interest::PARSE_ERROR,
    }
}

fn compare_messages(a: &Message, b: &Message) -> Ordering {
    let (a_pos, a_rank) = span_sort_key(a);
    let (b_pos, b_rank) = span_sort_key(b);

    let a_key = (a_pos, a_rank, a.order, &a.code);
    let b_key = (b_pos, b_rank, b.order, &b.code);
    a_key.cmp(&b_key)
}

fn compare_messages_vnu(a: &Message, b: &Message) -> Ordering {
    let a_key = (severity_rank_desc(a.severity), a.order, &a.code);
    let b_key = (severity_rank_desc(b.severity), b.order, &b.code);
    a_key.cmp(&b_key)
}

#[inline]
fn severity_rank_desc(sev: Severity) -> Reverse<u8> {
    Reverse(sev.rank())
}

fn span_sort_key(m: &Message) -> (usize, Reverse<u8>) {
    let start = m.span.map_or(usize::MAX, |s| s.byte_start);
    (start, severity_rank_desc(m.severity))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_default_values_are_stable() {
        let c = Config::default();
        assert!(!c.ignore_missing_lang);
        assert_eq!(c.message_order, MessageOrder::Emit);
        assert!(!c.also_check_css);
        assert_eq!(c.severity_profile, SeverityProfile::Risk);
        assert_eq!(c.min_severity, Severity::Info);
        assert!(c.base_uri.is_none());
        assert!(c.css_profile.is_none());
        assert!(c.css_medium.is_none());
        assert!(c.css_warning.is_none());
        assert!(c.csp_header.is_none());
    }

    #[test]
    fn ascii_case_insensitive_contains_is_case_insensitive_only_when_needed() {
        const LIST: [&str; 3] = ["a", "b", "cat"];

        assert!(ascii_case_insensitive_contains(&LIST, "a"));
        assert!(ascii_case_insensitive_contains(&LIST, "b"));
        assert!(ascii_case_insensitive_contains(&LIST, "cat"));

        assert!(ascii_case_insensitive_contains(&LIST, "A"));
        assert!(ascii_case_insensitive_contains(&LIST, "B"));
        assert!(ascii_case_insensitive_contains(&LIST, "CAT"));

        assert!(!ascii_case_insensitive_contains(&LIST, "dog"));
        assert!(!ascii_case_insensitive_contains(&LIST, "DOG"));

        // ASCII-only case folding still works with non-ASCII content.
        const LIST2: [&str; 1] = ["bé"];
        assert!(ascii_case_insensitive_contains(&LIST2, "bé"));
        assert!(ascii_case_insensitive_contains(&LIST2, "Bé"));
        assert!(!ascii_case_insensitive_contains(&LIST2, "BE"));
    }

    #[test]
    fn starts_with_ascii_ci_is_byte_based_and_non_panicking() {
        assert!(starts_with_ascii_ci("Hello", ""));
        assert!(starts_with_ascii_ci("Hello", "he"));
        assert!(!starts_with_ascii_ci("Hi", "hello"));
        assert!(!starts_with_ascii_ci("", "a"));
        assert!(starts_with_ascii_ci("", ""));
        assert!(!starts_with_ascii_ci("❤", "h"));
        assert!(!starts_with_ascii_ci("❤", "❤H"));
        assert!(starts_with_ascii_ci("❤H", "❤h"));
        assert!(starts_with_ascii_ci("🦀A", "🦀a"));
        assert!(!starts_with_ascii_ci("🦀", "🦀a"));
    }

    #[test]
    fn ends_with_ascii_ci_is_byte_based_and_non_panicking() {
        assert!(ends_with_ascii_ci("Hello", ""));
        assert!(ends_with_ascii_ci("Hello", "LO"));
        assert!(!ends_with_ascii_ci("Hi", "hello"));
        assert!(!ends_with_ascii_ci("abc", "xabc"));
        assert!(!ends_with_ascii_ci("", "a"));
        assert!(ends_with_ascii_ci("", ""));
        assert!(ends_with_ascii_ci("☃.xhtml", ".xhtml"));
        assert!(!ends_with_ascii_ci("é", "e"));
        assert!(!ends_with_ascii_ci("❤", "h"));
        assert!(ends_with_ascii_ci("❤H", "h"));
        assert!(!ends_with_ascii_ci("❤", "❤H"));
        assert!(ends_with_ascii_ci("x🦀A", "🦀a"));
        assert!(!ends_with_ascii_ci("🦀", "x🦀a"));
    }

    #[test]
    fn ascii_lowercase_cow_if_needed_borrows_when_no_ascii_uppercase() {
        assert!(matches!(
            ascii_lowercase_cow_if_needed("already-lower"),
            Cow::Borrowed(_)
        ));
        assert!(matches!(
            ascii_lowercase_cow_if_needed("no-Äöü"),
            Cow::Borrowed(_)
        ));
    }

    #[test]
    fn ascii_lowercase_cow_if_needed_owns_and_lowercases_ascii_only() {
        let out = ascii_lowercase_cow_if_needed("AbC-Ä");
        assert!(matches!(out, Cow::Owned(_)));
        assert_eq!(out.as_ref(), "abc-Ä");
    }

    #[test]
    fn ascii_lowercase_cow_if_needed_handles_multibyte_prefix_before_ascii_uppercase() {
        let out = ascii_lowercase_cow_if_needed("🦀A");
        assert!(matches!(out, Cow::Owned(_)));
        assert_eq!(out.as_ref(), "🦀a");
    }

    #[test]
    fn is_metadata_element_respects_input_format_case_rules() {
        assert!(is_metadata_element(InputFormat::Html, "LiNk"));
        assert!(!is_metadata_element(InputFormat::Xhtml, "LiNk"));
        assert!(is_metadata_element(InputFormat::Xhtml, "link"));
    }

    #[test]
    fn attr_value_skips_missing_values_and_respects_input_format_case_rules() {
        let attrs = vec![
            Attribute {
                name: "HREF".to_string(),
                value: None,
                span: None,
            },
            Attribute {
                name: "href".to_string(),
                value: Some("a".to_string()),
                span: None,
            },
        ];

        let html = ValidationContext::new(Config::default(), InputFormat::Html);
        assert_eq!(html.attr_value(&attrs, "href"), Some("a"));
        assert_eq!(html.attr_value(&attrs, "HREF"), Some("a"));

        let xhtml = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        assert_eq!(xhtml.attr_value(&attrs, "href"), Some("a"));
        assert_eq!(xhtml.attr_value(&attrs, "HREF"), None);
    }

    #[test]
    fn attr_value_returns_none_when_attribute_has_no_value() {
        let attrs = vec![Attribute {
            name: "href".to_string(),
            value: None,
            span: None,
        }];

        let html = ValidationContext::new(Config::default(), InputFormat::Html);
        assert_eq!(html.attr_value(&attrs, "href"), None);
        assert!(html.has_attr(&attrs, "HREF"));

        let xhtml = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        assert_eq!(xhtml.attr_value(&attrs, "href"), None);
        assert!(xhtml.has_attr(&attrs, "href"));
    }

    #[test]
    fn attr_value_returns_empty_string_when_attribute_value_is_empty() {
        let attrs = vec![Attribute {
            name: "href".to_string(),
            value: Some(String::new()),
            span: None,
        }];

        let html = ValidationContext::new(Config::default(), InputFormat::Html);
        assert_eq!(html.attr_value(&attrs, "href"), Some(""));

        let xhtml = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        assert_eq!(xhtml.attr_value(&attrs, "href"), Some(""));
    }

    #[test]
    fn report_counts_messages_by_severity() {
        let report = Report {
            source_name: "t".to_string(),
            messages: vec![
                Message::new("e1", Severity::Error, Category::Internal, "m", None),
                Message::new("w1", Severity::Warning, Category::Internal, "m", None),
                Message::new("w2", Severity::Warning, Category::Internal, "m", None),
                Message::new("i1", Severity::Info, Category::Internal, "m", None),
            ],
        };
        assert_eq!(report.counts(), (1, 2, 1));
    }

    #[test]
    fn validation_context_name_is_matches_html_case_insensitively() {
        let ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        assert!(ctx.name_is("DiV", "div"));
        assert!(ctx.name_is("div", "DIV"));
    }

    #[test]
    fn validation_context_name_is_matches_xhtml_case_sensitively() {
        let ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        assert!(ctx.name_is("div", "div"));
        assert!(!ctx.name_is("DiV", "div"));
    }

    #[test]
    fn validation_context_attr_helpers_follow_format_case_rules() {
        let attrs = vec![
            Attribute {
                name: "HREF".to_string(),
                value: Some("x".to_string()),
                span: None,
            },
            Attribute {
                name: "rel".to_string(),
                value: None,
                span: None,
            },
        ];

        let html_ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        assert_eq!(html_ctx.attr_value(&attrs, "href"), Some("x"));
        assert!(html_ctx.has_attr(&attrs, "href"));
        assert_eq!(html_ctx.attr_value(&attrs, "rel"), None);
        assert!(html_ctx.has_attr(&attrs, "rel"));

        let xhtml_ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        assert_eq!(xhtml_ctx.attr_value(&attrs, "href"), None);
        assert!(!xhtml_ctx.has_attr(&attrs, "href"));
        assert_eq!(xhtml_ctx.attr_value(&attrs, "rel"), None);
        assert!(xhtml_ctx.has_attr(&attrs, "rel"));
    }

    #[test]
    fn validation_context_attr_value_skips_valueless_duplicates() {
        let attrs = vec![
            Attribute {
                name: "class".to_string(),
                value: None,
                span: None,
            },
            Attribute {
                name: "class".to_string(),
                value: Some("x".to_string()),
                span: None,
            },
        ];

        let ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        assert_eq!(ctx.attr_value(&attrs, "class"), Some("x"));
    }

    #[test]
    fn validation_context_attr_value_skips_valueless_duplicates_case_insensitively_in_html() {
        let attrs = vec![
            Attribute {
                name: "CLASS".to_string(),
                value: None,
                span: None,
            },
            Attribute {
                name: "CLASS".to_string(),
                value: Some("x".to_string()),
                span: None,
            },
        ];

        let html_ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        assert_eq!(html_ctx.attr_value(&attrs, "class"), Some("x"));

        let xhtml_ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        assert_eq!(xhtml_ctx.attr_value(&attrs, "class"), None);
    }

    #[test]
    fn validation_context_attr_value_skips_valueless_duplicates_even_when_case_differs() {
        let attrs = vec![
            Attribute {
                name: "CLASS".to_string(),
                value: None,
                span: None,
            },
            Attribute {
                name: "class".to_string(),
                value: Some("x".to_string()),
                span: None,
            },
        ];

        let html_ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        assert_eq!(html_ctx.attr_value(&attrs, "class"), Some("x"));

        let xhtml_ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        assert_eq!(xhtml_ctx.attr_value(&attrs, "class"), Some("x"));
    }

    #[test]
    fn validation_context_attr_value_prefers_first_valued_duplicate() {
        let attrs = vec![
            Attribute {
                name: "class".to_string(),
                value: Some("a".to_string()),
                span: None,
            },
            Attribute {
                name: "class".to_string(),
                value: Some("b".to_string()),
                span: None,
            },
        ];

        let ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        assert_eq!(ctx.attr_value(&attrs, "class"), Some("a"));
    }

    #[test]
    fn validation_context_attr_value_returns_empty_string_when_present() {
        let attrs = vec![Attribute {
            name: "href".to_string(),
            value: Some("".to_string()),
            span: None,
        }];

        let ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        assert_eq!(ctx.attr_value(&attrs, "href"), Some(""));
        assert!(ctx.has_attr(&attrs, "href"));
    }

    #[test]
    fn foreign_insertion_after_respects_format_case_rules() {
        let html_ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        assert_eq!(
            html_ctx.foreign_insertion_after(ForeignContentNamespace::Html, "SVG", &[]),
            ForeignContentNamespace::Svg
        );
        assert_eq!(
            html_ctx.foreign_insertion_after(ForeignContentNamespace::Html, "math", &[]),
            ForeignContentNamespace::Math
        );
        assert_eq!(
            html_ctx.foreign_insertion_after(ForeignContentNamespace::Html, "div", &[]),
            ForeignContentNamespace::Html
        );
        assert_eq!(
            html_ctx.foreign_insertion_after(ForeignContentNamespace::Svg, "foreignObject", &[]),
            ForeignContentNamespace::Html
        );
        assert_eq!(
            html_ctx.foreign_insertion_after(ForeignContentNamespace::Svg, "DeSc", &[]),
            ForeignContentNamespace::Html
        );
        assert_eq!(
            html_ctx.foreign_insertion_after(ForeignContentNamespace::Svg, "TITLE", &[]),
            ForeignContentNamespace::Html
        );
        assert_eq!(
            html_ctx.foreign_insertion_after(ForeignContentNamespace::Svg, "rect", &[]),
            ForeignContentNamespace::Svg
        );
        assert_eq!(
            html_ctx.foreign_insertion_after(ForeignContentNamespace::Math, "svg", &[]),
            ForeignContentNamespace::Math
        );

        let xhtml_ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        assert_eq!(
            xhtml_ctx.foreign_insertion_after(ForeignContentNamespace::Html, "SVG", &[]),
            ForeignContentNamespace::Html
        );
        assert_eq!(
            xhtml_ctx.foreign_insertion_after(ForeignContentNamespace::Html, "math", &[]),
            ForeignContentNamespace::Math
        );
        assert_eq!(
            xhtml_ctx.foreign_insertion_after(ForeignContentNamespace::Svg, "foreignObject", &[]),
            ForeignContentNamespace::Svg
        );
        assert_eq!(
            xhtml_ctx.foreign_insertion_after(ForeignContentNamespace::Svg, "foreignobject", &[]),
            ForeignContentNamespace::Html
        );
        assert_eq!(
            xhtml_ctx.foreign_insertion_after(ForeignContentNamespace::Math, "svg", &[]),
            ForeignContentNamespace::Math
        );
    }

    #[test]
    fn compare_messages_orders_by_span_severity_then_emit_order() {
        let mut m1 = Message::new(
            "a",
            Severity::Warning,
            Category::Internal,
            "m1",
            Some(Span::new(5, 6, 1, 1)),
        );
        m1.order = 1;

        let mut m2 = Message::new(
            "b",
            Severity::Error,
            Category::Internal,
            "m2",
            Some(Span::new(5, 6, 1, 1)),
        );
        m2.order = 0;

        let mut m3 = Message::new(
            "c",
            Severity::Error,
            Category::Internal,
            "m3",
            Some(Span::new(3, 4, 1, 1)),
        );
        m3.order = 2;

        let mut m4 = Message::new("d", Severity::Error, Category::Internal, "m4", None);
        m4.order = 3;

        let mut msgs = [m1, m2, m3, m4];
        msgs.sort_unstable_by(compare_messages);

        let codes: Vec<&str> = msgs.iter().map(|m| m.code.as_str()).collect();
        assert_eq!(codes, vec!["c", "b", "a", "d"]);
    }

    #[test]
    fn compare_messages_breaks_ties_by_emit_order_even_when_code_matches() {
        let mut early = Message::new(
            "same",
            Severity::Error,
            Category::Internal,
            "early",
            Some(Span::new(1, 2, 1, 2)),
        );
        early.order = 0;

        let mut late = Message::new(
            "same",
            Severity::Error,
            Category::Internal,
            "late",
            Some(Span::new(1, 2, 1, 2)),
        );
        late.order = 1;

        let mut msgs = [late, early];
        msgs.sort_unstable_by(compare_messages);
        let orders: Vec<u64> = msgs.iter().map(|m| m.order).collect();
        assert_eq!(orders, vec![0, 1]);
    }

    #[test]
    fn compare_messages_breaks_ties_by_code_when_other_keys_match() {
        let mut b = Message::new(
            "b",
            Severity::Error,
            Category::Internal,
            "b",
            Some(Span::new(1, 2, 1, 1)),
        );
        b.order = 0;

        let mut a = Message::new(
            "a",
            Severity::Error,
            Category::Internal,
            "a",
            Some(Span::new(1, 2, 1, 1)),
        );
        a.order = 0;

        let mut msgs = [b, a];
        msgs.sort_unstable_by(compare_messages);
        let codes: Vec<&str> = msgs.iter().map(|m| m.code.as_str()).collect();
        assert_eq!(codes, vec!["a", "b"]);
    }

    #[test]
    fn compare_messages_vnu_groups_by_severity_preserving_emit_order() {
        let mut e0 = Message::new("e0", Severity::Error, Category::Internal, "e0", None);
        e0.order = 0;
        let mut e1 = Message::new("e1", Severity::Error, Category::Internal, "e1", None);
        e1.order = 2;
        let mut w0 = Message::new("w0", Severity::Warning, Category::Internal, "w0", None);
        w0.order = 1;
        let mut i0 = Message::new("i0", Severity::Info, Category::Internal, "i0", None);
        i0.order = 3;

        let mut msgs = [w0, e1, i0, e0];
        msgs.sort_unstable_by(compare_messages_vnu);

        let codes: Vec<&str> = msgs.iter().map(|m| m.code.as_str()).collect();
        assert_eq!(codes, vec!["e0", "e1", "w0", "i0"]);
    }

    #[test]
    fn compare_messages_vnu_breaks_ties_by_emit_order_even_when_code_matches() {
        let mut early = Message::new("same", Severity::Warning, Category::Internal, "early", None);
        early.order = 0;
        let mut late = Message::new("same", Severity::Warning, Category::Internal, "late", None);
        late.order = 1;

        let mut msgs = [late, early];
        msgs.sort_unstable_by(compare_messages_vnu);
        let orders: Vec<u64> = msgs.iter().map(|m| m.order).collect();
        assert_eq!(orders, vec![0, 1]);
    }

    #[test]
    fn compare_messages_vnu_breaks_ties_by_code_when_other_keys_match() {
        let mut b = Message::new("b", Severity::Warning, Category::Internal, "b", None);
        b.order = 0;
        let mut a = Message::new("a", Severity::Warning, Category::Internal, "a", None);
        a.order = 0;

        let mut msgs = [b, a];
        msgs.sort_unstable_by(compare_messages_vnu);
        let codes: Vec<&str> = msgs.iter().map(|m| m.code.as_str()).collect();
        assert_eq!(codes, vec!["a", "b"]);
    }

    #[test]
    fn is_interested_matches_event_kinds_to_bits() {
        let cases = [
            (
                ParseEvent::StartTag {
                    name: "div".to_string(),
                    attrs: vec![],
                    self_closing: false,
                    span: None,
                },
                Interest::START_TAG,
            ),
            (
                ParseEvent::EndTag {
                    name: "div".to_string(),
                    span: None,
                },
                Interest::END_TAG,
            ),
            (
                ParseEvent::Text {
                    text: "t".to_string(),
                    span: None,
                },
                Interest::TEXT,
            ),
            (
                ParseEvent::ProcessingInstruction {
                    target: "t".to_string(),
                    data: "d".to_string(),
                    span: None,
                },
                Interest::PROCESSING_INSTRUCTION,
            ),
            (
                ParseEvent::Comment {
                    text: "c".to_string(),
                    span: None,
                },
                Interest::COMMENT,
            ),
            (
                ParseEvent::Doctype {
                    name: None,
                    public_id: None,
                    system_id: None,
                    span: None,
                },
                Interest::DOCTYPE,
            ),
            (
                ParseEvent::ParseError {
                    code: "err".to_string(),
                    message: "m".to_string(),
                    span: None,
                },
                Interest::PARSE_ERROR,
            ),
        ];

        for (event, bit) in cases {
            assert_eq!(event_interest(&event), bit);
            assert!(Interest::ALL.contains(bit));
            assert!((bit | Interest::FINISH).contains(bit));
            assert!(!Interest::NONE.contains(bit));
        }
    }

    #[test]
    fn validate_events_filters_by_rule_interest() {
        struct PanicOnNonStartTag;
        impl Rule for PanicOnNonStartTag {
            fn id(&self) -> &'static str {
                "test.panic_on_non_start_tag"
            }
            fn interest(&self) -> Interest {
                Interest::START_TAG
            }
            fn on_event(
                &mut self,
                event: &ParseEvent,
                _ctx: &mut ValidationContext,
                out: &mut dyn MessageSink,
            ) {
                let ParseEvent::StartTag { .. } = event else {
                    panic!("rule should only receive StartTag events");
                };
                out.push(Message::new(
                    "test.start",
                    Severity::Info,
                    Category::Internal,
                    "start",
                    None,
                ));
            }
        }

        struct PanicOnNonText;
        impl Rule for PanicOnNonText {
            fn id(&self) -> &'static str {
                "test.panic_on_non_text"
            }
            fn interest(&self) -> Interest {
                Interest::TEXT
            }
            fn on_event(
                &mut self,
                event: &ParseEvent,
                _ctx: &mut ValidationContext,
                out: &mut dyn MessageSink,
            ) {
                let ParseEvent::Text { .. } = event else {
                    panic!("rule should only receive Text events");
                };
                out.push(Message::new(
                    "test.text",
                    Severity::Info,
                    Category::Internal,
                    "text",
                    None,
                ));
            }
        }

        let report = validate_events(
            VecSource::new(
                InputFormat::Html,
                vec![
                    ParseEvent::StartTag {
                        name: "div".to_string(),
                        attrs: vec![],
                        self_closing: false,
                        span: None,
                    },
                    ParseEvent::Comment {
                        text: "c".to_string(),
                        span: None,
                    },
                    ParseEvent::Text {
                        text: "t".to_string(),
                        span: None,
                    },
                ],
            ),
            RuleSet::new().push(PanicOnNonStartTag).push(PanicOnNonText),
            Config::default(),
        )
        .unwrap();

        let codes: Vec<&str> = report.messages.iter().map(|m| m.code.as_str()).collect();
        assert_eq!(codes, vec!["test.start", "test.text"]);
    }

    #[test]
    fn validate_events_skips_rules_below_min_severity() {
        struct ShouldBeSkipped;
        impl Rule for ShouldBeSkipped {
            fn id(&self) -> &'static str {
                panic!("rule should have been skipped")
            }
            fn max_severity(&self) -> Severity {
                Severity::Warning
            }
            fn interest(&self) -> Interest {
                panic!("rule should have been skipped")
            }
        }

        let _ = validate_events(
            VecSource::new(
                InputFormat::Html,
                vec![ParseEvent::StartTag {
                    name: "div".to_string(),
                    attrs: vec![],
                    self_closing: false,
                    span: None,
                }],
            ),
            RuleSet::new().push(ShouldBeSkipped),
            Config {
                min_severity: Severity::Error,
                ..Config::default()
            },
        )
        .unwrap();
    }

    #[test]
    fn validate_events_filters_messages_below_min_severity() {
        struct EmitInfoWarnError;
        impl Rule for EmitInfoWarnError {
            fn id(&self) -> &'static str {
                "test.emit_info_warn_error"
            }
            fn max_severity(&self) -> Severity {
                Severity::Error
            }
            fn interest(&self) -> Interest {
                Interest::START_TAG
            }
            fn on_event(
                &mut self,
                _event: &ParseEvent,
                _ctx: &mut ValidationContext,
                out: &mut dyn MessageSink,
            ) {
                out.push(Message::new(
                    "i",
                    Severity::Info,
                    Category::Internal,
                    "i",
                    None,
                ));
                out.push(Message::new(
                    "w",
                    Severity::Warning,
                    Category::Internal,
                    "w",
                    None,
                ));
                out.push(Message::new(
                    "e",
                    Severity::Error,
                    Category::Internal,
                    "e",
                    None,
                ));
            }
        }

        let src = VecSource::new(
            InputFormat::Html,
            vec![ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            }],
        );

        let report = validate_events(
            src,
            RuleSet::new().push(EmitInfoWarnError),
            Config {
                min_severity: Severity::Error,
                ..Config::default()
            },
        )
        .unwrap();
        let codes: Vec<&str> = report.messages.iter().map(|m| m.code.as_str()).collect();
        assert_eq!(codes, vec!["e"]);

        let src = VecSource::new(
            InputFormat::Html,
            vec![ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            }],
        );
        let report = validate_events(
            src,
            RuleSet::new().push(EmitInfoWarnError),
            Config {
                min_severity: Severity::Warning,
                ..Config::default()
            },
        )
        .unwrap();
        let codes: Vec<&str> = report.messages.iter().map(|m| m.code.as_str()).collect();
        assert_eq!(codes, vec!["w", "e"]);
    }

    #[test]
    fn validate_events_drops_messages_after_fatal_parse_error_pos() {
        struct EmitSpannedMessages;
        impl Rule for EmitSpannedMessages {
            fn id(&self) -> &'static str {
                "test.emit_spanned_messages"
            }
            fn on_event(
                &mut self,
                event: &ParseEvent,
                _ctx: &mut ValidationContext,
                out: &mut dyn MessageSink,
            ) {
                match event {
                    ParseEvent::StartTag { name, span, .. } => out.push(Message::new(
                        name.clone(),
                        Severity::Info,
                        Category::Internal,
                        "t",
                        *span,
                    )),
                    ParseEvent::ParseError { code, span, .. }
                        if code == "html.parser.cannot_recover" =>
                    {
                        out.push(Message::new(
                            "fatal",
                            Severity::Error,
                            Category::Internal,
                            "cannot recover",
                            *span,
                        ));
                    }
                    _ => {}
                }
            }
            fn on_finish(&mut self, _ctx: &mut ValidationContext, out: &mut dyn MessageSink) {
                out.push(Message::new(
                    "finish",
                    Severity::Info,
                    Category::Internal,
                    "finish",
                    None,
                ));
            }
        }

        let report = validate_events(
            VecSource::new(
                InputFormat::Html,
                vec![
                    ParseEvent::StartTag {
                        name: "before".to_string(),
                        attrs: vec![],
                        self_closing: false,
                        span: Some(Span::new(0, 1, 1, 1)),
                    },
                    ParseEvent::ParseError {
                        code: "html.parser.cannot_recover".to_string(),
                        message:
                            "Cannot recover after last error. Any further errors will be ignored."
                                .to_string(),
                        span: Some(Span::new(10, 11, 1, 11)),
                    },
                    ParseEvent::StartTag {
                        name: "after".to_string(),
                        attrs: vec![],
                        self_closing: false,
                        span: Some(Span::new(20, 21, 2, 1)),
                    },
                ],
            ),
            RuleSet::new().push(EmitSpannedMessages),
            Config::default(),
        )
        .unwrap();

        let codes: Vec<&str> = report.messages.iter().map(|m| m.code.as_str()).collect();
        assert_eq!(codes, vec!["before", "fatal"]);
    }

    #[test]
    fn validate_events_uses_first_fatal_parse_error_pos_as_cutoff() {
        struct EmitSpannedMessages;
        impl Rule for EmitSpannedMessages {
            fn id(&self) -> &'static str {
                "test.emit_spanned_messages"
            }
            fn interest(&self) -> Interest {
                Interest::START_TAG | Interest::PARSE_ERROR
            }
            fn on_event(
                &mut self,
                event: &ParseEvent,
                _ctx: &mut ValidationContext,
                out: &mut dyn MessageSink,
            ) {
                match event {
                    ParseEvent::StartTag { name, span, .. } => out.push(Message::new(
                        name.clone(),
                        Severity::Info,
                        Category::Internal,
                        "t",
                        *span,
                    )),
                    ParseEvent::ParseError { code, span, .. }
                        if code == "html.parser.cannot_recover" =>
                    {
                        out.push(Message::new(
                            "fatal",
                            Severity::Error,
                            Category::Internal,
                            "cannot recover",
                            *span,
                        ));
                    }
                    _ => {}
                }
            }
        }

        let report = validate_events(
            VecSource::new(
                InputFormat::Html,
                vec![
                    ParseEvent::ParseError {
                        code: "html.parser.cannot_recover".to_string(),
                        message: "fatal".to_string(),
                        span: Some(Span::new(10, 11, 1, 11)),
                    },
                    ParseEvent::ParseError {
                        code: "html.parser.cannot_recover".to_string(),
                        message: "fatal2".to_string(),
                        span: Some(Span::new(5, 6, 1, 6)),
                    },
                    ParseEvent::StartTag {
                        name: "kept".to_string(),
                        attrs: vec![],
                        self_closing: false,
                        span: Some(Span::new(7, 8, 1, 8)),
                    },
                    ParseEvent::StartTag {
                        name: "dropped".to_string(),
                        attrs: vec![],
                        self_closing: false,
                        span: Some(Span::new(12, 13, 1, 13)),
                    },
                ],
            ),
            RuleSet::new().push(EmitSpannedMessages),
            Config::default(),
        )
        .unwrap();

        let codes: Vec<&str> = report.messages.iter().map(|m| m.code.as_str()).collect();
        assert_eq!(codes, vec!["fatal", "fatal", "kept"]);
    }

    #[test]
    fn validate_events_retains_spanned_messages_when_fatal_parse_error_has_no_span() {
        struct EmitStartTags;
        impl Rule for EmitStartTags {
            fn id(&self) -> &'static str {
                "test.emit_start_tags"
            }
            fn interest(&self) -> Interest {
                Interest::START_TAG | Interest::PARSE_ERROR
            }
            fn on_event(
                &mut self,
                event: &ParseEvent,
                _ctx: &mut ValidationContext,
                out: &mut dyn MessageSink,
            ) {
                let ParseEvent::StartTag { name, span, .. } = event else {
                    return;
                };
                out.push(Message::new(
                    name.as_str(),
                    Severity::Info,
                    Category::Internal,
                    "t",
                    *span,
                ));
            }
        }

        let report = validate_events(
            VecSource::new(
                InputFormat::Html,
                vec![
                    ParseEvent::StartTag {
                        name: "before".to_string(),
                        attrs: vec![],
                        self_closing: false,
                        span: Some(Span::new(0, 1, 1, 1)),
                    },
                    ParseEvent::ParseError {
                        code: "html.parser.cannot_recover".to_string(),
                        message: "Cannot recover".to_string(),
                        span: None,
                    },
                    ParseEvent::StartTag {
                        name: "after".to_string(),
                        attrs: vec![],
                        self_closing: false,
                        span: Some(Span::new(10, 11, 1, 11)),
                    },
                ],
            ),
            RuleSet::new().push(EmitStartTags),
            Config::default(),
        )
        .unwrap();

        let codes: Vec<&str> = report.messages.iter().map(|m| m.code.as_str()).collect();
        assert_eq!(codes, vec!["before", "after"]);
    }

    #[test]
    fn validate_events_uses_first_fatal_parse_error_pos() {
        struct EmitSpannedCodes;
        impl Rule for EmitSpannedCodes {
            fn id(&self) -> &'static str {
                "test.emit_spanned_codes_first_fatal_pos"
            }
            fn interest(&self) -> Interest {
                Interest::START_TAG | Interest::PARSE_ERROR
            }
            fn on_event(
                &mut self,
                event: &ParseEvent,
                _ctx: &mut ValidationContext,
                out: &mut dyn MessageSink,
            ) {
                match event {
                    ParseEvent::StartTag { name, span, .. } => out.push(Message::new(
                        name.as_str(),
                        Severity::Info,
                        Category::Internal,
                        "t",
                        *span,
                    )),
                    ParseEvent::ParseError { code, span, .. }
                        if code == "html.parser.cannot_recover" =>
                    {
                        out.push(Message::new(
                            "fatal",
                            Severity::Error,
                            Category::Internal,
                            "cannot recover",
                            *span,
                        ));
                    }
                    _ => {}
                }
            }
        }

        let report = validate_events(
            VecSource::new(
                InputFormat::Html,
                vec![
                    ParseEvent::StartTag {
                        name: "before".to_string(),
                        attrs: vec![],
                        self_closing: false,
                        span: Some(Span::new(5, 6, 1, 6)),
                    },
                    ParseEvent::ParseError {
                        code: "html.parser.cannot_recover".to_string(),
                        message: "Cannot recover".to_string(),
                        span: Some(Span::new(10, 11, 1, 11)),
                    },
                    ParseEvent::StartTag {
                        name: "after".to_string(),
                        attrs: vec![],
                        self_closing: false,
                        span: Some(Span::new(15, 16, 1, 16)),
                    },
                    ParseEvent::ParseError {
                        code: "html.parser.cannot_recover".to_string(),
                        message: "Cannot recover (again)".to_string(),
                        span: Some(Span::new(20, 21, 1, 21)),
                    },
                ],
            ),
            RuleSet::new().push(EmitSpannedCodes),
            Config {
                message_order: MessageOrder::Emit,
                ..Config::default()
            },
        )
        .unwrap();

        let codes: Vec<&str> = report.messages.iter().map(|m| m.code.as_str()).collect();
        assert_eq!(codes, vec!["before", "fatal"]);
    }

    #[test]
    fn validate_events_keeps_first_fatal_parse_error_pos_even_if_later_fatal_is_earlier_in_source()
    {
        struct EmitSpannedCodes;
        impl Rule for EmitSpannedCodes {
            fn id(&self) -> &'static str {
                "test.emit_spanned_codes_first_fatal_pos_out_of_order"
            }
            fn interest(&self) -> Interest {
                Interest::START_TAG | Interest::PARSE_ERROR
            }
            fn on_event(
                &mut self,
                event: &ParseEvent,
                _ctx: &mut ValidationContext,
                out: &mut dyn MessageSink,
            ) {
                match event {
                    ParseEvent::StartTag { name, span, .. } => out.push(Message::new(
                        name.as_str(),
                        Severity::Info,
                        Category::Internal,
                        "t",
                        *span,
                    )),
                    ParseEvent::ParseError { code, span, .. }
                        if code == "html.parser.cannot_recover" =>
                    {
                        let msg_code = match span.as_ref().map(|s| s.byte_start) {
                            Some(20) => "fatal1",
                            Some(10) => "fatal2",
                            _ => "fatal",
                        };
                        out.push(Message::new(
                            msg_code,
                            Severity::Error,
                            Category::Internal,
                            "cannot recover",
                            *span,
                        ));
                    }
                    _ => {}
                }
            }
        }

        let report = validate_events(
            VecSource::new(
                InputFormat::Html,
                vec![
                    ParseEvent::StartTag {
                        name: "before".to_string(),
                        attrs: vec![],
                        self_closing: false,
                        span: Some(Span::new(0, 1, 1, 1)),
                    },
                    ParseEvent::ParseError {
                        code: "html.parser.cannot_recover".to_string(),
                        message: "Cannot recover".to_string(),
                        span: Some(Span::new(20, 21, 1, 21)),
                    },
                    // Out-of-order: this event is after the fatal error, but earlier in the source.
                    ParseEvent::StartTag {
                        name: "mid".to_string(),
                        attrs: vec![],
                        self_closing: false,
                        span: Some(Span::new(15, 16, 1, 16)),
                    },
                    ParseEvent::ParseError {
                        code: "html.parser.cannot_recover".to_string(),
                        message: "Cannot recover (again)".to_string(),
                        span: Some(Span::new(10, 11, 1, 11)),
                    },
                    ParseEvent::StartTag {
                        name: "after".to_string(),
                        attrs: vec![],
                        self_closing: false,
                        span: Some(Span::new(25, 26, 1, 26)),
                    },
                ],
            ),
            RuleSet::new().push(EmitSpannedCodes),
            Config::default(),
        )
        .unwrap();

        let codes: Vec<&str> = report.messages.iter().map(|m| m.code.as_str()).collect();
        assert_eq!(codes, vec!["before", "fatal1", "mid", "fatal2"]);
    }

    #[test]
    fn validate_events_fatal_parse_error_without_span_does_not_drop_later_messages() {
        struct EmitSpannedCodes;
        impl Rule for EmitSpannedCodes {
            fn id(&self) -> &'static str {
                "test.emit_spanned_codes_fatal_span_none"
            }
            fn interest(&self) -> Interest {
                Interest::START_TAG | Interest::PARSE_ERROR
            }
            fn on_event(
                &mut self,
                event: &ParseEvent,
                _ctx: &mut ValidationContext,
                out: &mut dyn MessageSink,
            ) {
                match event {
                    ParseEvent::StartTag { name, span, .. } => out.push(Message::new(
                        name.as_str(),
                        Severity::Info,
                        Category::Internal,
                        "t",
                        *span,
                    )),
                    ParseEvent::ParseError { code, span, .. }
                        if code == "html.parser.cannot_recover" =>
                    {
                        let msg_code = if span.is_some() {
                            "fatal_some"
                        } else {
                            "fatal_none"
                        };
                        out.push(Message::new(
                            msg_code,
                            Severity::Error,
                            Category::Internal,
                            "cannot recover",
                            *span,
                        ));
                    }
                    _ => {}
                }
            }
        }

        let report = validate_events(
            VecSource::new(
                InputFormat::Html,
                vec![
                    ParseEvent::ParseError {
                        code: "html.parser.cannot_recover".to_string(),
                        message: "Cannot recover".to_string(),
                        span: None,
                    },
                    ParseEvent::StartTag {
                        name: "after_first".to_string(),
                        attrs: vec![],
                        self_closing: false,
                        span: Some(Span::new(100, 101, 1, 101)),
                    },
                    ParseEvent::ParseError {
                        code: "html.parser.cannot_recover".to_string(),
                        message: "Cannot recover (again)".to_string(),
                        span: Some(Span::new(10, 11, 1, 11)),
                    },
                    ParseEvent::StartTag {
                        name: "after_second".to_string(),
                        attrs: vec![],
                        self_closing: false,
                        span: Some(Span::new(200, 201, 2, 1)),
                    },
                ],
            ),
            RuleSet::new().push(EmitSpannedCodes),
            Config {
                message_order: MessageOrder::Emit,
                ..Config::default()
            },
        )
        .unwrap();

        let codes: Vec<&str> = report.messages.iter().map(|m| m.code.as_str()).collect();
        assert_eq!(codes, vec!["after_first", "fatal_some", "after_second"]);
    }

    #[test]
    fn validate_events_does_not_override_first_fatal_pos_with_spanless_fatal_error() {
        struct EmitSpannedCodes;
        impl Rule for EmitSpannedCodes {
            fn id(&self) -> &'static str {
                "test.emit_spanned_codes_fatal_span_some_then_none"
            }
            fn interest(&self) -> Interest {
                Interest::START_TAG | Interest::PARSE_ERROR
            }
            fn on_event(
                &mut self,
                event: &ParseEvent,
                _ctx: &mut ValidationContext,
                out: &mut dyn MessageSink,
            ) {
                match event {
                    ParseEvent::StartTag { name, span, .. } => out.push(Message::new(
                        name.as_str(),
                        Severity::Info,
                        Category::Internal,
                        "t",
                        *span,
                    )),
                    ParseEvent::ParseError { code, span, .. }
                        if code == "html.parser.cannot_recover" =>
                    {
                        let msg_code = if span.is_some() {
                            "fatal_some"
                        } else {
                            "fatal_none"
                        };
                        out.push(Message::new(
                            msg_code,
                            Severity::Error,
                            Category::Internal,
                            "cannot recover",
                            *span,
                        ));
                    }
                    _ => {}
                }
            }
        }

        let report = validate_events(
            VecSource::new(
                InputFormat::Html,
                vec![
                    ParseEvent::StartTag {
                        name: "before".to_string(),
                        attrs: vec![],
                        self_closing: false,
                        span: Some(Span::new(0, 1, 1, 1)),
                    },
                    ParseEvent::ParseError {
                        code: "html.parser.cannot_recover".to_string(),
                        message: "Cannot recover".to_string(),
                        span: Some(Span::new(10, 11, 1, 11)),
                    },
                    ParseEvent::StartTag {
                        name: "after".to_string(),
                        attrs: vec![],
                        self_closing: false,
                        span: Some(Span::new(20, 21, 1, 21)),
                    },
                    ParseEvent::ParseError {
                        code: "html.parser.cannot_recover".to_string(),
                        message: "Cannot recover (again)".to_string(),
                        span: None,
                    },
                    // Out-of-order: this event is after the fatal error, but earlier in the source.
                    ParseEvent::StartTag {
                        name: "mid".to_string(),
                        attrs: vec![],
                        self_closing: false,
                        span: Some(Span::new(5, 6, 1, 6)),
                    },
                ],
            ),
            RuleSet::new().push(EmitSpannedCodes),
            Config {
                message_order: MessageOrder::Emit,
                ..Config::default()
            },
        )
        .unwrap();

        let codes: Vec<&str> = report.messages.iter().map(|m| m.code.as_str()).collect();
        assert_eq!(codes, vec!["before", "fatal_some", "mid"]);
    }

    #[test]
    fn validate_events_sets_had_fatal_parse_error() {
        struct AssertFatalOnFinish;
        impl Rule for AssertFatalOnFinish {
            fn id(&self) -> &'static str {
                "test.assert_fatal_parse_error_flag"
            }
            fn interest(&self) -> Interest {
                Interest::PARSE_ERROR
            }
            fn on_finish(&mut self, ctx: &mut ValidationContext, _out: &mut dyn MessageSink) {
                assert!(ctx.had_fatal_parse_error());
            }
        }

        validate_events(
            VecSource::new(
                InputFormat::Html,
                vec![ParseEvent::ParseError {
                    code: "html.parser.cannot_recover".to_string(),
                    message: "Cannot recover".to_string(),
                    span: Some(Span::new(0, 1, 1, 1)),
                }],
            ),
            RuleSet::new().push(AssertFatalOnFinish),
            Config::default(),
        )
        .unwrap();
    }

    #[test]
    fn foreign_insertion_namespace_defaults_to_html_and_tracks_stack() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        assert_eq!(
            ctx.foreign_insertion_namespace(),
            ForeignContentNamespace::Html
        );

        ctx.foreign_insertion_stack
            .push(ForeignContentNamespace::Svg);
        assert_eq!(
            ctx.foreign_insertion_namespace(),
            ForeignContentNamespace::Svg
        );
    }

    #[test]
    fn validation_context_open_elements_and_foreign_namespace_track_start_end_tags() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);

        ctx.on_start_tag("br", false);
        assert!(ctx.open_elements().is_empty());

        ctx.on_start_tag("svg", false);
        assert!(ctx.open_elements().iter().map(String::as_str).eq(["svg"]));
        assert_eq!(
            ctx.foreign_insertion_namespace(),
            ForeignContentNamespace::Svg
        );

        ctx.on_start_tag("foreignobject", false);
        assert!(ctx
            .open_elements()
            .iter()
            .map(String::as_str)
            .eq(["svg", "foreignobject"]));
        assert_eq!(
            ctx.foreign_insertion_namespace(),
            ForeignContentNamespace::Html
        );

        ctx.on_end_tag("foreignobject");
        assert!(ctx.open_elements().iter().map(String::as_str).eq(["svg"]));
        assert_eq!(
            ctx.foreign_insertion_namespace(),
            ForeignContentNamespace::Svg
        );

        ctx.on_end_tag("svg");
        assert!(ctx.open_elements().is_empty());
        assert_eq!(
            ctx.foreign_insertion_namespace(),
            ForeignContentNamespace::Html
        );
    }

    #[test]
    fn validation_context_foreign_insertion_svg_integration_points_switch_to_html() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);

        ctx.on_start_tag("svg", false);
        assert_eq!(
            ctx.foreign_insertion_namespace(),
            ForeignContentNamespace::Svg
        );

        ctx.on_start_tag("desc", false);
        assert_eq!(
            ctx.foreign_insertion_namespace(),
            ForeignContentNamespace::Html
        );
        ctx.on_end_tag("desc");
        assert_eq!(
            ctx.foreign_insertion_namespace(),
            ForeignContentNamespace::Svg
        );

        ctx.on_start_tag("title", false);
        assert_eq!(
            ctx.foreign_insertion_namespace(),
            ForeignContentNamespace::Html
        );
        ctx.on_end_tag("title");
        assert_eq!(
            ctx.foreign_insertion_namespace(),
            ForeignContentNamespace::Svg
        );
    }

    #[test]
    fn validation_context_foreign_insertion_respects_xhtml_case_sensitivity() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);

        ctx.on_start_tag("SVG", false);
        assert_eq!(
            ctx.foreign_insertion_namespace(),
            ForeignContentNamespace::Html
        );
        ctx.on_end_tag("SVG");
        assert_eq!(
            ctx.foreign_insertion_namespace(),
            ForeignContentNamespace::Html
        );

        ctx.on_start_tag("svg", false);
        assert_eq!(
            ctx.foreign_insertion_namespace(),
            ForeignContentNamespace::Svg
        );
    }

    #[test]
    fn validation_context_foreign_insertion_math_switches_to_math_and_persists() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);

        ctx.on_start_tag("math", false);
        assert!(ctx.open_elements().iter().map(String::as_str).eq(["math"]));
        assert_eq!(
            ctx.foreign_insertion_namespace(),
            ForeignContentNamespace::Math
        );

        // Minimal behavior: we only switch namespaces from HTML->SVG/Math, and SVG integration
        // points switch back to HTML. While in Math, other tags keep the Math namespace.
        ctx.on_start_tag("svg", false);
        assert!(ctx
            .open_elements()
            .iter()
            .map(String::as_str)
            .eq(["math", "svg"]));
        assert_eq!(
            ctx.foreign_insertion_namespace(),
            ForeignContentNamespace::Math
        );

        ctx.on_end_tag("svg");
        assert!(ctx.open_elements().iter().map(String::as_str).eq(["math"]));
        assert_eq!(
            ctx.foreign_insertion_namespace(),
            ForeignContentNamespace::Math
        );

        ctx.on_end_tag("math");
        assert!(ctx.open_elements().is_empty());
        assert_eq!(
            ctx.foreign_insertion_namespace(),
            ForeignContentNamespace::Html
        );
    }

    #[test]
    fn validation_context_xhtml_self_closing_does_not_push_open_elements() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);

        ctx.on_start_tag("p", true);
        assert!(ctx.open_elements().is_empty());

        ctx.on_start_tag("p", false);
        assert!(ctx.open_elements().iter().map(String::as_str).eq(["p"]));
    }

    #[test]
    fn validation_context_xhtml_non_self_closing_void_elements_still_push_open_elements() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        ctx.on_start_tag("br", false);
        assert!(ctx.open_elements().iter().map(String::as_str).eq(["br"]));
        ctx.on_end_tag("br");
        assert!(ctx.open_elements().is_empty());
    }

    #[test]
    fn validation_context_html_self_closing_still_pushes_non_void_elements() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        ctx.on_start_tag("p", true);
        assert!(ctx.open_elements().iter().map(String::as_str).eq(["p"]));
    }

    #[test]
    fn interest_bit_ops_and_display() {
        let mut i = Interest::START_TAG | Interest::TEXT;
        assert!(i.contains(Interest::START_TAG));
        assert!(!i.contains(Interest::END_TAG));

        let both = Interest::START_TAG & Interest::TEXT;
        assert!(!both.contains(Interest::START_TAG));
        assert!(!both.contains(Interest::TEXT));

        i |= Interest::END_TAG;
        assert!(i.contains(Interest::END_TAG));

        i &= Interest::START_TAG;
        assert!(i.contains(Interest::START_TAG));
        assert!(!i.contains(Interest::END_TAG));

        let rendered = format!("{i}");
        assert!(rendered.starts_with("Interest("));
    }

    #[test]
    fn void_html_elements_match_case_insensitively() {
        assert!(is_void_html_element("br"));
        assert!(is_void_html_element("BR"));
        assert!(is_void_html_element("Img"));
        assert!(!is_void_html_element("div"));
    }

    #[test]
    fn metadata_elements_match_by_input_format() {
        assert!(is_metadata_element(InputFormat::Html, "META"));
        assert!(is_metadata_element(InputFormat::Html, "Head"));
        assert!(is_metadata_element(InputFormat::Html, "Html"));
        assert!(is_metadata_element(InputFormat::Html, "head"));
        assert!(!is_metadata_element(InputFormat::Html, "div"));

        assert!(is_metadata_element(InputFormat::Xhtml, "meta"));
        assert!(!is_metadata_element(InputFormat::Xhtml, "META"));
    }

    #[test]
    fn validation_context_section_updates_follow_metadata_rules() {
        let mut html_ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        assert_eq!(html_ctx.document.section, DocumentSection::Head);
        html_ctx.on_start_tag("meta", false);
        assert_eq!(html_ctx.document.section, DocumentSection::Head);
        html_ctx.on_start_tag("div", false);
        assert_eq!(html_ctx.document.section, DocumentSection::Body);
        html_ctx.on_start_tag("head", false);
        assert_eq!(html_ctx.document.section, DocumentSection::Head);
        html_ctx.on_start_tag("body", false);
        assert_eq!(html_ctx.document.section, DocumentSection::Body);

        let mut xhtml_ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        assert_eq!(xhtml_ctx.document.section, DocumentSection::Head);
        xhtml_ctx.on_start_tag("meta", false);
        assert_eq!(xhtml_ctx.document.section, DocumentSection::Head);
        xhtml_ctx.on_start_tag("META", false);
        assert_eq!(xhtml_ctx.document.section, DocumentSection::Body);
    }

    #[test]
    fn ascii_case_insensitive_contains_is_case_insensitive_when_needed() {
        const ITEMS: [&str; 3] = ["a", "br", "meta"];
        assert!(ascii_case_insensitive_contains(&ITEMS, "br"));
        assert!(ascii_case_insensitive_contains(&ITEMS, "BR"));
        assert!(!ascii_case_insensitive_contains(&ITEMS, "BrX"));
    }

    struct VecSource {
        name: String,
        format: InputFormat,
        events: std::vec::IntoIter<ParseEvent>,
    }

    impl VecSource {
        fn new(format: InputFormat, events: Vec<ParseEvent>) -> Self {
            Self {
                name: "vec".to_string(),
                format,
                events: events.into_iter(),
            }
        }
    }

    impl EventSource for VecSource {
        fn source_name(&self) -> &str {
            &self.name
        }
        fn format(&self) -> InputFormat {
            self.format
        }
        fn next_event(&mut self) -> Result<Option<ParseEvent>, ValidatorError> {
            Ok(self.events.next())
        }
    }

    struct CountEventsRule {
        pub start_tags: usize,
        pub finish: bool,
    }

    impl Rule for CountEventsRule {
        fn id(&self) -> &'static str {
            "test.count_events"
        }
        fn on_event(
            &mut self,
            event: &ParseEvent,
            _ctx: &mut ValidationContext,
            _out: &mut dyn MessageSink,
        ) {
            if matches!(event, ParseEvent::StartTag { .. }) {
                self.start_tags += 1;
            }
        }
        fn on_finish(&mut self, _ctx: &mut ValidationContext, _out: &mut dyn MessageSink) {
            self.finish = true;
        }
    }

    #[test]
    fn dispatches_events_and_finish() {
        let src = VecSource::new(
            InputFormat::Html,
            vec![ParseEvent::StartTag {
                name: "html".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            }],
        );
        let rule = CountEventsRule {
            start_tags: 0,
            finish: false,
        };
        let _ = rule.id();
        let rules = RuleSet::new().push(rule);
        let report = validate_events(src, rules, Config::default()).unwrap();
        assert_eq!(report.messages.len(), 0);
    }

    #[test]
    fn validate_events_moves_start_tag_name_into_open_elements_without_cloning() {
        struct AssertStartTagMoved {
            expected_ptr: usize,
        }

        impl Rule for AssertStartTagMoved {
            fn id(&self) -> &'static str {
                "test.assert_start_tag_moved"
            }

            fn on_finish(&mut self, ctx: &mut ValidationContext, _out: &mut dyn MessageSink) {
                assert_eq!(ctx.open_elements().len(), 1);
                let s = &ctx.open_elements()[0];
                assert_eq!(s, "div");
                assert_eq!(s.as_ptr() as usize, self.expected_ptr);
            }
        }

        let mut name = String::with_capacity(32);
        name.push_str("div");
        let expected_ptr = name.as_ptr() as usize;

        let src = VecSource::new(
            InputFormat::Xhtml,
            vec![ParseEvent::StartTag {
                name,
                attrs: vec![],
                self_closing: false,
                span: None,
            }],
        );
        let rules = RuleSet::new().push(AssertStartTagMoved { expected_ptr });
        let report = validate_events(src, rules, Config::default()).unwrap();
        assert_eq!(report.messages.len(), 0);
    }

    #[test]
    fn count_events_rule_counts_start_tags_and_finish() {
        let mut rule = CountEventsRule {
            start_tags: 0,
            finish: false,
        };
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = CollectingSink::default();
        rule.on_event(
            &ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            &mut ctx,
            &mut sink,
        );
        rule.on_finish(&mut ctx, &mut sink);
        assert_eq!(rule.start_tags, 1);
        assert!(rule.finish);
    }

    #[test]
    fn collecting_sink_assigns_sequential_order() {
        let mut sink = CollectingSink::default();
        sink.push(Message::new(
            "t.a",
            Severity::Info,
            Category::Internal,
            "a",
            None,
        ));
        sink.push(Message::new(
            "t.b",
            Severity::Info,
            Category::Internal,
            "b",
            None,
        ));
        assert_eq!(sink.messages.len(), 2);
        assert_eq!(sink.messages[0].order, 0);
        assert_eq!(sink.messages[1].order, 1);
    }

    #[test]
    fn ruleset_merge_preserves_rule_order() {
        struct EmitOnFinish(&'static str);
        impl Rule for EmitOnFinish {
            fn id(&self) -> &'static str {
                "test.emit_on_finish"
            }
            fn on_finish(&mut self, _ctx: &mut ValidationContext, out: &mut dyn MessageSink) {
                out.push(Message::new(
                    self.0,
                    Severity::Info,
                    Category::Internal,
                    self.0,
                    None,
                ));
            }
        }

        let rules = RuleSet::new()
            .push(EmitOnFinish("a"))
            .merge(RuleSet::new().push(EmitOnFinish("b")));
        let report = validate_events(
            VecSource::new(InputFormat::Html, vec![]),
            rules,
            Config::default(),
        )
        .unwrap();
        let codes: Vec<&str> = report.messages.iter().map(|m| m.code.as_str()).collect();
        assert_eq!(codes, vec!["a", "b"]);
    }

    #[test]
    fn vnu_message_order_groups_errors_before_warnings() {
        struct EmitWarnThenError;
        impl Rule for EmitWarnThenError {
            fn id(&self) -> &'static str {
                "test.emit_warn_then_error"
            }
            fn on_event(
                &mut self,
                _event: &ParseEvent,
                _ctx: &mut ValidationContext,
                out: &mut dyn MessageSink,
            ) {
                out.push(Message::new(
                    "test.warning",
                    Severity::Warning,
                    Category::Internal,
                    "w".to_string(),
                    None,
                ));
                out.push(Message::new(
                    "test.error",
                    Severity::Error,
                    Category::Internal,
                    "e".to_string(),
                    None,
                ));
            }
        }

        let src = VecSource::new(
            InputFormat::Html,
            vec![ParseEvent::StartTag {
                name: "html".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            }],
        );
        let rule = EmitWarnThenError;
        let _ = rule.id();
        let rules = RuleSet::new().push(rule);
        let report = validate_events(
            src,
            rules,
            Config {
                ignore_missing_lang: true,
                message_order: MessageOrder::Vnu,
                ..Config::default()
            },
        )
        .unwrap();
        assert_eq!(
            report.messages.first().map(|m| m.code.as_str()),
            Some("test.error")
        );
    }

    #[test]
    fn vnu_message_order_includes_info_severity_rank() {
        struct EmitInfoThenWarnThenError;
        impl Rule for EmitInfoThenWarnThenError {
            fn id(&self) -> &'static str {
                "test.emit_info_warn_error"
            }
            fn on_event(
                &mut self,
                _event: &ParseEvent,
                _ctx: &mut ValidationContext,
                out: &mut dyn MessageSink,
            ) {
                out.push(Message::new(
                    "test.info",
                    Severity::Info,
                    Category::Internal,
                    "i".to_string(),
                    None,
                ));
                out.push(Message::new(
                    "test.warning",
                    Severity::Warning,
                    Category::Internal,
                    "w".to_string(),
                    None,
                ));
                out.push(Message::new(
                    "test.error",
                    Severity::Error,
                    Category::Internal,
                    "e".to_string(),
                    None,
                ));
            }
        }

        let src = VecSource::new(
            InputFormat::Html,
            vec![ParseEvent::StartTag {
                name: "html".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            }],
        );
        let report = validate_events(
            src,
            RuleSet::new().push(EmitInfoThenWarnThenError),
            Config {
                ignore_missing_lang: true,
                message_order: MessageOrder::Vnu,
                ..Config::default()
            },
        )
        .unwrap();

        let codes: Vec<&str> = report.messages.iter().map(|m| m.code.as_str()).collect();
        assert_eq!(codes, vec!["test.error", "test.warning", "test.info"]);
    }

    #[test]
    fn by_span_message_order_sorts_by_position_then_severity() {
        struct EmitOutOfOrder;
        impl Rule for EmitOutOfOrder {
            fn id(&self) -> &'static str {
                "test.emit_out_of_order"
            }
            fn on_event(
                &mut self,
                _event: &ParseEvent,
                _ctx: &mut ValidationContext,
                out: &mut dyn MessageSink,
            ) {
                out.push(Message::new(
                    "test.warn_late",
                    Severity::Warning,
                    Category::Internal,
                    "w2",
                    Some(Span::new(10, 11, 1, 11)),
                ));
                out.push(Message::new(
                    "test.warn_early",
                    Severity::Warning,
                    Category::Internal,
                    "w1",
                    Some(Span::new(5, 6, 1, 6)),
                ));
                out.push(Message::new(
                    "test.err_early",
                    Severity::Error,
                    Category::Internal,
                    "e1",
                    Some(Span::new(5, 6, 1, 6)),
                ));
            }
        }

        let src = VecSource::new(
            InputFormat::Html,
            vec![ParseEvent::StartTag {
                name: "html".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            }],
        );
        let rule = EmitOutOfOrder;
        let _ = rule.id();
        let rules = RuleSet::new().push(rule);
        let report = validate_events(
            src,
            rules,
            Config {
                ignore_missing_lang: true,
                message_order: MessageOrder::BySpan,
                ..Config::default()
            },
        )
        .unwrap();

        let codes: Vec<&str> = report.messages.iter().map(|m| m.code.as_str()).collect();
        assert_eq!(
            codes,
            vec!["test.err_early", "test.warn_early", "test.warn_late"]
        );
    }

    #[test]
    fn by_span_message_order_breaks_ties_by_emit_order_then_code() {
        let mut m_span1_info = Message::new(
            "span1-info",
            Severity::Info,
            Category::Internal,
            "i",
            Some(Span::new(1, 2, 1, 2)),
        );
        m_span1_info.order = 0;

        let mut m_span1_error = Message::new(
            "span1-error",
            Severity::Error,
            Category::Internal,
            "e",
            Some(Span::new(1, 2, 1, 2)),
        );
        m_span1_error.order = 1;

        let mut m_span5_error_2 = Message::new(
            "span5-error-2",
            Severity::Error,
            Category::Internal,
            "e2",
            Some(Span::new(5, 6, 1, 6)),
        );
        m_span5_error_2.order = 2;

        let mut m_span5_error_1 = Message::new(
            "span5-error-1",
            Severity::Error,
            Category::Internal,
            "e1",
            Some(Span::new(5, 6, 1, 6)),
        );
        m_span5_error_1.order = 1;

        let mut m_span5_warn_b = Message::new(
            "span5-warn-b",
            Severity::Warning,
            Category::Internal,
            "wb",
            Some(Span::new(5, 6, 1, 6)),
        );
        m_span5_warn_b.order = 99;

        let mut m_span5_warn_a = Message::new(
            "span5-warn-a",
            Severity::Warning,
            Category::Internal,
            "wa",
            Some(Span::new(5, 6, 1, 6)),
        );
        m_span5_warn_a.order = 99;

        let mut m_nospan = Message::new("nospan", Severity::Error, Category::Internal, "n", None);
        m_nospan.order = 0;

        let mut messages = [
            m_span5_warn_b,
            m_nospan,
            m_span1_info,
            m_span5_error_2,
            m_span5_warn_a,
            m_span1_error,
            m_span5_error_1,
        ];
        messages.sort_unstable_by(compare_messages);

        let codes: Vec<&str> = messages.iter().map(|m| m.code.as_str()).collect();
        assert_eq!(
            codes,
            vec![
                "span1-error",
                "span1-info",
                "span5-error-1",
                "span5-error-2",
                "span5-warn-a",
                "span5-warn-b",
                "nospan",
            ]
        );
    }

    #[test]
    fn filters_messages_after_fatal_parse_error_by_span_position() {
        struct EmitCodeAtSpan;
        impl Rule for EmitCodeAtSpan {
            fn id(&self) -> &'static str {
                "test.emit_code_at_span"
            }
            fn interest(&self) -> Interest {
                Interest::ALL
            }
            fn on_event(
                &mut self,
                event: &ParseEvent,
                _ctx: &mut ValidationContext,
                out: &mut dyn MessageSink,
            ) {
                let (code, span) = match event {
                    ParseEvent::StartTag { span, .. } => ("test.start", *span),
                    ParseEvent::ParseError { span, .. } => ("test.parse_error", *span),
                    _ => return,
                };
                out.push(Message::new(
                    code,
                    Severity::Error,
                    Category::Internal,
                    code,
                    span,
                ));
            }
        }

        let report = validate_events(
            VecSource::new(
                InputFormat::Html,
                vec![
                    ParseEvent::ParseError {
                        code: "html.parser.cannot_recover".to_string(),
                        message: "Cannot recover".to_string(),
                        span: Some(Span::new(5, 6, 1, 6)),
                    },
                    // Deliberately out-of-source-order: should still be kept because its span is
                    // before the fatal position.
                    ParseEvent::StartTag {
                        name: "a".to_string(),
                        attrs: vec![],
                        self_closing: false,
                        span: Some(Span::new(1, 2, 1, 2)),
                    },
                    // Should be dropped.
                    ParseEvent::StartTag {
                        name: "b".to_string(),
                        attrs: vec![],
                        self_closing: false,
                        span: Some(Span::new(10, 11, 1, 11)),
                    },
                ],
            ),
            RuleSet::new().push(EmitCodeAtSpan),
            Config {
                ignore_missing_lang: true,
                message_order: MessageOrder::Emit,
                ..Config::default()
            },
        )
        .unwrap();

        let codes: Vec<&str> = report.messages.iter().map(|m| m.code.as_str()).collect();
        assert_eq!(codes, vec!["test.parse_error", "test.start"]);
    }

    #[test]
    fn filters_messages_at_fatal_parse_error_position_are_retained() {
        struct EmitSpannedCodes;
        impl Rule for EmitSpannedCodes {
            fn id(&self) -> &'static str {
                "test.emit_spanned_codes"
            }
            fn interest(&self) -> Interest {
                Interest::ALL
            }
            fn on_event(
                &mut self,
                event: &ParseEvent,
                _ctx: &mut ValidationContext,
                out: &mut dyn MessageSink,
            ) {
                match event {
                    ParseEvent::StartTag { name, span, .. } => out.push(Message::new(
                        name.clone(),
                        Severity::Info,
                        Category::Internal,
                        "t",
                        *span,
                    )),
                    ParseEvent::ParseError {
                        code,
                        message,
                        span,
                    } if code == "html.parser.cannot_recover" => out.push(Message::new(
                        "fatal",
                        Severity::Error,
                        Category::Internal,
                        message.clone(),
                        *span,
                    )),
                    _ => {}
                }
            }
        }

        let report = validate_events(
            VecSource::new(
                InputFormat::Html,
                vec![
                    ParseEvent::ParseError {
                        code: "html.parser.cannot_recover".to_string(),
                        message: "Cannot recover".to_string(),
                        span: Some(Span::new(5, 6, 1, 6)),
                    },
                    // Same position as the fatal parse error: should be retained (<= fatal_pos).
                    ParseEvent::StartTag {
                        name: "equal".to_string(),
                        attrs: vec![],
                        self_closing: false,
                        span: Some(Span::new(5, 6, 1, 6)),
                    },
                    // After the fatal position: should be dropped.
                    ParseEvent::StartTag {
                        name: "after".to_string(),
                        attrs: vec![],
                        self_closing: false,
                        span: Some(Span::new(6, 7, 1, 7)),
                    },
                ],
            ),
            RuleSet::new().push(EmitSpannedCodes),
            Config {
                message_order: MessageOrder::Emit,
                ..Config::default()
            },
        )
        .unwrap();

        let codes: Vec<&str> = report.messages.iter().map(|m| m.code.as_str()).collect();
        assert_eq!(codes, vec!["fatal", "equal"]);
    }

    #[test]
    fn validate_events_when_fatal_parse_error_has_no_span_retains_all_spanned_messages() {
        struct EmitSpannedStartTagsAndUnspannedFinish;
        impl Rule for EmitSpannedStartTagsAndUnspannedFinish {
            fn id(&self) -> &'static str {
                "test.emit_spanned_start_tags_and_unspanned_finish"
            }
            fn interest(&self) -> Interest {
                Interest::START_TAG | Interest::PARSE_ERROR
            }
            fn on_event(
                &mut self,
                event: &ParseEvent,
                _ctx: &mut ValidationContext,
                out: &mut dyn MessageSink,
            ) {
                match event {
                    ParseEvent::StartTag { name, span, .. } => out.push(Message::new(
                        name.clone(),
                        Severity::Info,
                        Category::Internal,
                        "t",
                        *span,
                    )),
                    ParseEvent::ParseError { code, span, .. }
                        if code == "html.parser.cannot_recover" =>
                    {
                        out.push(Message::new(
                            "fatal",
                            Severity::Error,
                            Category::Internal,
                            "cannot recover",
                            *span,
                        ));
                    }
                    _ => {}
                }
            }
            fn on_finish(&mut self, _ctx: &mut ValidationContext, out: &mut dyn MessageSink) {
                out.push(Message::new(
                    "finish",
                    Severity::Info,
                    Category::Internal,
                    "finish",
                    None,
                ));
            }
        }

        let report = validate_events(
            VecSource::new(
                InputFormat::Html,
                vec![
                    ParseEvent::StartTag {
                        name: "before".to_string(),
                        attrs: vec![],
                        self_closing: false,
                        span: Some(Span::new(1, 2, 1, 2)),
                    },
                    ParseEvent::ParseError {
                        code: "html.parser.cannot_recover".to_string(),
                        message: "Cannot recover".to_string(),
                        span: None,
                    },
                    ParseEvent::StartTag {
                        name: "after".to_string(),
                        attrs: vec![],
                        self_closing: false,
                        span: Some(Span::new(10, 11, 1, 11)),
                    },
                ],
            ),
            RuleSet::new().push(EmitSpannedStartTagsAndUnspannedFinish),
            Config::default(),
        )
        .unwrap();

        let codes: Vec<&str> = report.messages.iter().map(|m| m.code.as_str()).collect();
        assert_eq!(codes, vec!["before", "after"]);
    }

    #[test]
    fn open_elements_stack_respects_void_elements_and_self_closing() {
        struct EmitStackAt {
            tag: &'static str,
        }
        impl Rule for EmitStackAt {
            fn id(&self) -> &'static str {
                "test.emit_stack_at"
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
                let ParseEvent::StartTag { name, .. } = event else {
                    return;
                };
                if name != self.tag {
                    return;
                }
                out.push(Message::new(
                    "test.stack",
                    Severity::Info,
                    Category::Internal,
                    ctx.open_elements().join("/"),
                    None,
                ));
            }
        }

        // In HTML, void elements never push onto the open-elements stack.
        let src = VecSource::new(
            InputFormat::Html,
            vec![
                ParseEvent::StartTag {
                    name: "map".to_string(),
                    attrs: vec![],
                    self_closing: false,
                    span: None,
                },
                ParseEvent::StartTag {
                    name: "area".to_string(),
                    attrs: vec![],
                    self_closing: false,
                    span: None,
                },
                ParseEvent::StartTag {
                    name: "div".to_string(),
                    attrs: vec![],
                    self_closing: false,
                    span: None,
                },
            ],
        );
        let report = validate_events(
            src,
            RuleSet::new().push(EmitStackAt { tag: "div" }),
            Config::default(),
        )
        .unwrap();
        assert_eq!(report.messages.len(), 1);
        assert_eq!(report.messages[0].message, "map");

        // In XHTML, self-closing tags never push onto the open-elements stack.
        let src = VecSource::new(
            InputFormat::Xhtml,
            vec![
                ParseEvent::StartTag {
                    name: "div".to_string(),
                    attrs: vec![],
                    self_closing: true,
                    span: None,
                },
                ParseEvent::StartTag {
                    name: "span".to_string(),
                    attrs: vec![],
                    self_closing: false,
                    span: None,
                },
            ],
        );
        let report = validate_events(
            src,
            RuleSet::new().push(EmitStackAt { tag: "span" }),
            Config::default(),
        )
        .unwrap();
        assert_eq!(report.messages.len(), 1);
        assert_eq!(report.messages[0].message, "");
    }

    #[test]
    fn emit_stack_at_ignores_non_start_tag_events() {
        struct EmitStackAt {
            tag: &'static str,
        }
        impl Rule for EmitStackAt {
            fn id(&self) -> &'static str {
                "test.emit_stack_at_direct"
            }
            fn interest(&self) -> Interest {
                Interest::START_TAG
            }
            fn on_event(
                &mut self,
                event: &ParseEvent,
                _ctx: &mut ValidationContext,
                _out: &mut dyn MessageSink,
            ) {
                let ParseEvent::StartTag { name, .. } = event else {
                    return;
                };
                let _ = name == self.tag;
            }
        }

        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = CollectingSink::default();
        let mut rule = EmitStackAt { tag: "x" };
        let _ = rule.id();
        let _ = rule.interest();
        rule.on_event(
            &ParseEvent::Text {
                text: "t".to_string(),
                span: None,
            },
            &mut ctx,
            &mut sink,
        );
    }

    #[test]
    fn xhtml_end_tag_truncates_open_elements_stack() {
        struct RecordStackOnFinish;
        impl Rule for RecordStackOnFinish {
            fn id(&self) -> &'static str {
                "test.record_stack_on_finish_xhtml"
            }
            fn on_finish(&mut self, ctx: &mut ValidationContext, out: &mut dyn MessageSink) {
                out.push(Message::new(
                    "test.stack",
                    Severity::Info,
                    Category::Internal,
                    ctx.open_elements().join("/"),
                    None,
                ));
            }
        }

        let src = VecSource::new(
            InputFormat::Xhtml,
            vec![
                ParseEvent::StartTag {
                    name: "div".to_string(),
                    attrs: vec![],
                    self_closing: false,
                    span: None,
                },
                ParseEvent::StartTag {
                    name: "span".to_string(),
                    attrs: vec![],
                    self_closing: false,
                    span: None,
                },
                ParseEvent::EndTag {
                    name: "div".to_string(),
                    span: None,
                },
            ],
        );
        let report = validate_events(
            src,
            RuleSet::new().push(RecordStackOnFinish),
            Config::default(),
        )
        .unwrap();
        assert_eq!(report.messages.len(), 1);
        assert_eq!(report.messages[0].message, "");
    }

    #[test]
    fn validation_context_on_end_tag_truncates_in_xhtml_directly() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        ctx.on_start_tag("div", false);
        ctx.on_start_tag("span", false);
        ctx.on_end_tag("div");
        assert!(ctx.open_elements().is_empty());
    }

    #[test]
    fn validation_context_html_self_closing_non_void_element_still_pushes() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        ctx.on_start_tag("div", true);
        assert_eq!(ctx.current_parent(), Some("div"));
    }

    #[test]
    fn validation_context_xhtml_self_closing_element_does_not_push() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        ctx.on_start_tag("div", true);
        assert!(ctx.open_elements().is_empty());
    }

    #[test]
    fn validation_context_tracks_foreign_insertion_namespace_in_html() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        assert_eq!(
            ctx.foreign_insertion_namespace(),
            ForeignContentNamespace::Html
        );

        ctx.on_start_tag("svg", false);
        assert_eq!(
            ctx.foreign_insertion_namespace(),
            ForeignContentNamespace::Svg
        );

        ctx.on_start_tag("foreignObject", false);
        assert_eq!(
            ctx.foreign_insertion_namespace(),
            ForeignContentNamespace::Html
        );

        ctx.on_end_tag("foreignObject");
        assert_eq!(
            ctx.foreign_insertion_namespace(),
            ForeignContentNamespace::Svg
        );

        ctx.on_end_tag("svg");
        assert_eq!(
            ctx.foreign_insertion_namespace(),
            ForeignContentNamespace::Html
        );
    }

    #[test]
    fn validation_context_svg_integration_points_switch_to_html_insertion_mode() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);

        ctx.on_start_tag("svg", false);
        assert_eq!(
            ctx.foreign_insertion_namespace(),
            ForeignContentNamespace::Svg
        );

        ctx.on_start_tag("desc", false);
        assert_eq!(
            ctx.foreign_insertion_namespace(),
            ForeignContentNamespace::Html
        );
        ctx.on_end_tag("desc");
        assert_eq!(
            ctx.foreign_insertion_namespace(),
            ForeignContentNamespace::Svg
        );

        ctx.on_start_tag("TITLE", false);
        assert_eq!(
            ctx.foreign_insertion_namespace(),
            ForeignContentNamespace::Html
        );
        ctx.on_end_tag("TiTlE");
        assert_eq!(
            ctx.foreign_insertion_namespace(),
            ForeignContentNamespace::Svg
        );

        ctx.on_end_tag("svg");
        assert_eq!(
            ctx.foreign_insertion_namespace(),
            ForeignContentNamespace::Html
        );
    }

    #[test]
    fn document_section_switches_from_head_to_body_after_non_metadata_element() {
        struct RecordSectionAt {
            tag: &'static str,
        }
        impl Rule for RecordSectionAt {
            fn id(&self) -> &'static str {
                "test.record_section_at"
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
                let ParseEvent::StartTag { name, .. } = event else {
                    return;
                };
                if name != self.tag {
                    return;
                }
                let section = match ctx.document.section {
                    DocumentSection::Head => "head",
                    DocumentSection::Body => "body",
                };
                out.push(Message::new(
                    "test.section",
                    Severity::Info,
                    Category::Internal,
                    section,
                    None,
                ));
            }
        }

        let src = VecSource::new(
            InputFormat::Html,
            vec![
                ParseEvent::StartTag {
                    name: "head".to_string(),
                    attrs: vec![],
                    self_closing: false,
                    span: None,
                },
                ParseEvent::StartTag {
                    name: "title".to_string(),
                    attrs: vec![],
                    self_closing: false,
                    span: None,
                },
                // Not a metadata element => switches to body.
                ParseEvent::StartTag {
                    name: "div".to_string(),
                    attrs: vec![],
                    self_closing: false,
                    span: None,
                },
                // The section should now be body.
                ParseEvent::StartTag {
                    name: "base".to_string(),
                    attrs: vec![],
                    self_closing: false,
                    span: None,
                },
            ],
        );
        let report = validate_events(
            src,
            RuleSet::new().push(RecordSectionAt { tag: "base" }),
            Config::default(),
        )
        .unwrap();
        assert_eq!(report.messages.len(), 1);
        assert_eq!(report.messages[0].message, "body");

        // Also cover the head branch directly.
        let src = VecSource::new(
            InputFormat::Html,
            vec![ParseEvent::StartTag {
                name: "head".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            }],
        );
        let report = validate_events(
            src,
            RuleSet::new().push(RecordSectionAt { tag: "head" }),
            Config::default(),
        )
        .unwrap();
        assert_eq!(report.messages.len(), 1);
        assert_eq!(report.messages[0].message, "head");

        // And the non-start-tag branch in RecordSectionAt::on_event.
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = CollectingSink::default();
        let mut rule = RecordSectionAt { tag: "x" };
        rule.on_event(
            &ParseEvent::Text {
                text: "x".to_string(),
                span: None,
            },
            &mut ctx,
            &mut sink,
        );
    }

    #[test]
    fn rules_handle_unexpected_events_in_on_event() {
        struct OnlyStartTags;
        impl Rule for OnlyStartTags {
            fn id(&self) -> &'static str {
                "test.only_start_tags"
            }
            fn interest(&self) -> Interest {
                Interest::START_TAG
            }
            fn on_event(
                &mut self,
                event: &ParseEvent,
                _ctx: &mut ValidationContext,
                _out: &mut dyn MessageSink,
            ) {
                let ParseEvent::StartTag { .. } = event else {
                    return;
                };
            }
        }

        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = CollectingSink::default();
        let mut rule = OnlyStartTags;
        let _ = rule.id();
        let _ = rule.interest();
        rule.on_event(
            &ParseEvent::Text {
                text: "x".to_string(),
                span: None,
            },
            &mut ctx,
            &mut sink,
        );
    }

    #[test]
    fn by_span_sorts_messages_without_spans_last() {
        struct EmitMixedSpans;
        impl Rule for EmitMixedSpans {
            fn id(&self) -> &'static str {
                "test.emit_mixed_spans"
            }
            fn on_event(
                &mut self,
                _event: &ParseEvent,
                _ctx: &mut ValidationContext,
                out: &mut dyn MessageSink,
            ) {
                out.push(Message::new(
                    "test.none_span",
                    Severity::Error,
                    Category::Internal,
                    "n",
                    None,
                ));
                out.push(Message::new(
                    "test.with_span",
                    Severity::Error,
                    Category::Internal,
                    "s",
                    Some(Span::new(1, 2, 1, 2)),
                ));
            }
        }

        let src = VecSource::new(
            InputFormat::Html,
            vec![ParseEvent::StartTag {
                name: "html".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            }],
        );
        let report = validate_events(
            src,
            RuleSet::new().push(EmitMixedSpans),
            Config {
                ignore_missing_lang: true,
                message_order: MessageOrder::BySpan,
                ..Config::default()
            },
        )
        .unwrap();

        let codes: Vec<&str> = report.messages.iter().map(|m| m.code.as_str()).collect();
        assert_eq!(codes, vec!["test.with_span", "test.none_span"]);
    }

    #[test]
    fn html_end_tag_closes_to_the_matching_open_element() {
        struct RecordStackOnFinish;
        impl Rule for RecordStackOnFinish {
            fn id(&self) -> &'static str {
                "test.record_stack_on_finish"
            }
            fn on_finish(&mut self, ctx: &mut ValidationContext, out: &mut dyn MessageSink) {
                let msg = ctx.open_elements().join("/");
                out.push(Message::new(
                    "test.stack",
                    Severity::Info,
                    Category::Internal,
                    msg,
                    None,
                ));
            }
        }

        let src = VecSource::new(
            InputFormat::Html,
            vec![
                ParseEvent::StartTag {
                    name: "DiV".to_string(),
                    attrs: vec![],
                    self_closing: false,
                    span: None,
                },
                ParseEvent::StartTag {
                    name: "SpAn".to_string(),
                    attrs: vec![],
                    self_closing: false,
                    span: None,
                },
                // Closing <div> should implicitly close the <span> as well in this simplified stack model.
                ParseEvent::EndTag {
                    name: "dIv".to_string(),
                    span: None,
                },
            ],
        );
        let report = validate_events(
            src,
            RuleSet::new().push(RecordStackOnFinish),
            Config::default(),
        )
        .unwrap();
        assert_eq!(report.messages.len(), 1);
        assert_eq!(report.messages[0].message, "");
    }

    #[test]
    fn has_ancestor_is_case_insensitive_only_in_html() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        ctx.on_start_tag("DiV", false);
        assert!(ctx.has_ancestor("div"));

        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        ctx.on_start_tag("DiV", false);
        assert!(!ctx.has_ancestor("div"));
        assert!(ctx.has_ancestor("DiV"));
    }

    #[test]
    fn end_tag_that_is_not_open_is_a_noop() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        ctx.on_start_tag("html", false);
        ctx.on_start_tag("div", false);

        ctx.on_end_tag("span");

        assert_eq!(
            ctx.open_elements,
            vec!["html".to_string(), "div".to_string()]
        );
        assert_eq!(
            ctx.foreign_insertion_stack,
            vec![ForeignContentNamespace::Html, ForeignContentNamespace::Html]
        );
    }

    #[test]
    fn end_tag_truncates_open_elements_and_foreign_stack() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        ctx.on_start_tag("html", false);
        ctx.on_start_tag("svg", false);
        ctx.on_start_tag("title", false);

        assert_eq!(
            ctx.open_elements,
            vec!["html".to_string(), "svg".to_string(), "title".to_string()]
        );
        assert_eq!(
            ctx.foreign_insertion_stack,
            vec![
                ForeignContentNamespace::Html,
                ForeignContentNamespace::Svg,
                ForeignContentNamespace::Html
            ]
        );

        ctx.on_end_tag("SVG");
        assert_eq!(ctx.open_elements, vec!["html".to_string()]);
        assert_eq!(
            ctx.foreign_insertion_stack,
            vec![ForeignContentNamespace::Html]
        );

        ctx.on_end_tag("HTML");
        assert!(ctx.open_elements.is_empty());
        assert!(ctx.foreign_insertion_stack.is_empty());
    }

    #[derive(Default)]
    struct EmitOnFinish;

    impl Rule for EmitOnFinish {
        fn id(&self) -> &'static str {
            "test.emit_on_finish"
        }

        fn on_finish(&mut self, _ctx: &mut ValidationContext, out: &mut dyn MessageSink) {
            out.push(Message::new(
                "before",
                Severity::Error,
                Category::Internal,
                "before",
                Some(Span::new(5, 6, 1, 1)),
            ));
            out.push(Message::new(
                "at_fatal",
                Severity::Error,
                Category::Internal,
                "at_fatal",
                Some(Span::new(10, 11, 1, 1)),
            ));
            out.push(Message::new(
                "after",
                Severity::Error,
                Category::Internal,
                "after",
                Some(Span::new(11, 12, 1, 1)),
            ));
            out.push(Message::new(
                "no_span",
                Severity::Error,
                Category::Internal,
                "no_span",
                None,
            ));
        }
    }

    #[test]
    fn fatal_parse_error_filters_messages_by_span_start() -> Result<(), ValidatorError> {
        let events = vec![
            ParseEvent::StartTag {
                name: "p".to_string(),
                attrs: Vec::new(),
                self_closing: false,
                span: Some(Span::new(0, 3, 1, 1)),
            },
            ParseEvent::ParseError {
                code: "html.parser.cannot_recover".to_string(),
                message: "Cannot recover after last error.".to_string(),
                span: Some(Span::new(10, 10, 1, 11)),
            },
            ParseEvent::StartTag {
                name: "div".to_string(),
                attrs: Vec::new(),
                self_closing: false,
                span: Some(Span::new(20, 25, 1, 21)),
            },
        ];

        let report = validate_events(
            VecSource::new(InputFormat::Html, events),
            RuleSet::new().push(EmitOnFinish),
            Config::default(),
        )?;

        let codes: Vec<&str> = report.messages.iter().map(|m| m.code.as_str()).collect();
        assert_eq!(codes, vec!["before", "at_fatal"]);
        Ok(())
    }

    #[test]
    fn report_counts_match_severities() {
        let mut report = Report::default();
        report.messages.push(Message::new(
            "e",
            Severity::Error,
            Category::Html,
            "err",
            None,
        ));
        report.messages.push(Message::new(
            "w",
            Severity::Warning,
            Category::Html,
            "warn",
            None,
        ));
        report.messages.push(Message::new(
            "i",
            Severity::Info,
            Category::Html,
            "info",
            None,
        ));
        report.messages.push(Message::new(
            "e2",
            Severity::Error,
            Category::Html,
            "err2",
            None,
        ));

        assert_eq!(report.counts(), (2, 1, 1));
    }

    #[test]
    fn attr_value_respects_case_sensitivity_and_skips_valueless_attrs() {
        let attrs = vec![
            Attribute {
                name: "CLass".to_string(),
                value: None,
                span: None,
            },
            Attribute {
                name: "class".to_string(),
                value: Some("ok".to_string()),
                span: None,
            },
        ];

        let html_ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        assert_eq!(html_ctx.attr_value(&attrs, "class"), Some("ok"));

        let xhtml_ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        assert_eq!(xhtml_ctx.attr_value(&attrs, "class"), Some("ok"));
        assert_eq!(xhtml_ctx.attr_value(&attrs, "CLass"), None);
    }
}
