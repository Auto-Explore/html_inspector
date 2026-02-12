use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct MetaHttpEquivCharset;

fn find_ascii_ci(haystack: &str, needle: &str) -> Option<usize> {
    let hb = haystack.as_bytes();
    let nb = needle.as_bytes();
    if nb.is_empty() {
        return Some(0);
    }
    if hb.len() < nb.len() {
        return None;
    }
    hb.windows(nb.len())
        .position(|w| w.eq_ignore_ascii_case(nb))
}

impl Rule for MetaHttpEquivCharset {
    fn id(&self) -> &'static str {
        "i18n.meta.http_equiv_charset"
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

        let http_equiv = ctx.attr_value(attrs, "http-equiv").unwrap_or("");
        if !http_equiv.eq_ignore_ascii_case("content-type") {
            return;
        }

        let content = ctx.attr_value(attrs, "content").unwrap_or("");

        let Some(idx) = find_ascii_ci(content, "charset=") else {
            return;
        };
        let charset_part = &content[idx + "charset=".len()..];
        let charset = charset_part
            .split_once(';')
            .map_or(charset_part, |(before, _)| before)
            .trim();

        if charset.is_empty() {
            out.push(Message::new(
                "i18n.meta.http_equiv_charset.empty",
                Severity::Error,
                Category::I18n,
                format!("Bad value “{content}” for attribute “content” on element “meta”."),
                *span,
            ));
            return;
        }

        if !is_supported_charset(charset) {
            out.push(Message::new(
                "i18n.charset.unsupported",
                Severity::Error,
                Category::I18n,
                format!(
                    "Internal encoding declaration named an unsupported chararacter encoding “{charset}”."
                ),
                *span,
            ));
        }
    }
}

fn is_supported_charset(name: &str) -> bool {
    let n = name.trim();
    n.eq_ignore_ascii_case("utf-8")
        || n.eq_ignore_ascii_case("utf8")
        || n.eq_ignore_ascii_case("iso-8859-1")
        || n.eq_ignore_ascii_case("windows-1252")
        || n.eq_ignore_ascii_case("us-ascii")
}
