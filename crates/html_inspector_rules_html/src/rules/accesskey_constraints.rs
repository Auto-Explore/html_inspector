use std::collections::HashSet;

use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct AccesskeyConstraints;

impl Rule for AccesskeyConstraints {
    fn id(&self) -> &'static str {
        "html.accesskey.constraints"
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

        let raw = attrs
            .iter()
            .find(|a| match ctx.format {
                html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case("accesskey"),
                html_inspector_core::InputFormat::Xhtml => a.name == "accesskey",
            })
            .and_then(|a| a.value.as_deref());

        let Some(raw) = raw else { return };

        let mut seen = HashSet::<&str>::new();
        let mut dup = false;
        let mut bad_len = false;
        for token in raw.split_ascii_whitespace() {
            if token.chars().count() != 1 {
                bad_len = true;
                break;
            }
            if !seen.insert(token) {
                dup = true;
                break;
            }
        }

        if dup || bad_len {
            out.push(Message::new(
                "html.accesskey.duplicate",
                Severity::Error,
                Category::Html,
                format!("Bad value “{raw}” for attribute “accesskey” on element “{name}”."),
                *span,
            ));
        }
    }
}
