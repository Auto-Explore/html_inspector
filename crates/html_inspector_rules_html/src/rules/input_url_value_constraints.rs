use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, ValidationContext,
};

use super::a_href_constraints::{emit_forbidden_url_code_point, href_issue_severity};

#[derive(Default)]
pub struct InputUrlValueConstraints;

impl Rule for InputUrlValueConstraints {
    fn id(&self) -> &'static str {
        "html.input.url.value"
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
        if !ctx.name_is(name, "input") {
            return;
        }

        let input_type = ctx.attr_value(attrs, "type").unwrap_or("text");
        if !input_type.eq_ignore_ascii_case("url") {
            return;
        }

        let value = ctx.attr_value(attrs, "value");
        let Some(value) = value else { return };

        let v = value.trim();
        if v.is_empty() {
            return;
        }

        if emit_forbidden_url_code_point(v, *span, out) {
            return;
        }

        let mut sev = href_issue_severity(v);
        if sev.is_none() && !has_valid_scheme(v) {
            sev = Some(html_inspector::Severity::Error);
        }

        if let Some(sev) = sev {
            out.push(Message::new(
                "html.input.url.value.invalid",
                sev,
                Category::Html,
                format!("Bad value “{v}” for attribute “value” on element “input”."),
                *span,
            ));
        }
    }
}

fn has_valid_scheme(s: &str) -> bool {
    let Some((scheme, _)) = s.split_once(':') else {
        return false;
    };
    let mut chars = scheme.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !first.is_ascii_alphabetic() {
        return false;
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '-' || c == '.')
}
