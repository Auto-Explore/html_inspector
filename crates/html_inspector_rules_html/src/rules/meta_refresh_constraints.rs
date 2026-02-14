use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

use super::shared::starts_with_ascii_ci;

#[derive(Default)]
pub struct MetaRefreshConstraints;

impl Rule for MetaRefreshConstraints {
    fn id(&self) -> &'static str {
        "html.meta.refresh"
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
        if !is(ctx, name, "meta") {
            return;
        }

        let http_equiv = attrs
            .iter()
            .find(|a| match ctx.format {
                html_inspector::InputFormat::Html => a.name.eq_ignore_ascii_case("http-equiv"),
                html_inspector::InputFormat::Xhtml => a.name == "http-equiv",
            })
            .and_then(|a| a.value.as_deref())
            .unwrap_or("");
        if !http_equiv.eq_ignore_ascii_case("refresh") {
            return;
        }

        let content = attrs
            .iter()
            .find(|a| match ctx.format {
                html_inspector::InputFormat::Html => a.name.eq_ignore_ascii_case("content"),
                html_inspector::InputFormat::Xhtml => a.name == "content",
            })
            .and_then(|a| a.value.as_deref())
            .unwrap_or("");

        let push_invalid = |out: &mut dyn MessageSink| {
            out.push(Message::new(
                "html.meta.refresh.invalid",
                Severity::Error,
                Category::Html,
                format!("Bad value “{content}” for attribute “content” on element “meta”."),
                *span,
            ));
        };

        let v = content.trim();
        if v.is_empty() {
            out.push(Message::new(
                "html.meta.refresh.empty",
                Severity::Error,
                Category::Html,
                "Bad value “” for attribute “content” on element “meta”.",
                *span,
            ));
            return;
        }
        let digit_len = v
            .as_bytes()
            .iter()
            .take_while(|b| b.is_ascii_digit())
            .count();
        if digit_len == 0 {
            push_invalid(out);
            return;
        }

        let rest = &v[digit_len..];
        if rest.trim().is_empty() {
            return;
        }

        let rest = rest.trim_start();
        if !rest.starts_with(';') {
            push_invalid(out);
            return;
        }

        let after_sc = &rest[1..];
        if !after_sc
            .as_bytes()
            .first()
            .is_some_and(|b| b.is_ascii_whitespace())
        {
            push_invalid(out);
            return;
        }

        let after_ws = after_sc.trim_start();
        if !starts_with_ascii_ci(after_ws, "url=") {
            push_invalid(out);
            return;
        }

        let url_val = after_ws[4..].trim_start();
        if url_val.starts_with('\'') || url_val.starts_with('\"') {
            push_invalid(out);
        }
    }
}

fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    match ctx.format {
        html_inspector::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector::InputFormat::Xhtml => actual == expected,
    }
}
