use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

use super::shared::ascii_lowercase_cow_if_needed;

#[derive(Default)]
pub struct IsAttributeConstraints;

impl Rule for IsAttributeConstraints {
    fn id(&self) -> &'static str {
        "html.is_attribute.constraints"
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

        let is_value = attrs
            .iter()
            .find(|a| match ctx.format {
                html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case("is"),
                html_inspector_core::InputFormat::Xhtml => a.name == "is",
            })
            .and_then(|a| a.value.as_deref());

        let Some(is_value) = is_value else { return };

        if is_autonomous_custom_element_name(ctx, name) {
            out.push(Message::new(
                "html.is_attribute.autonomous_custom_element_forbidden",
                Severity::Error,
                Category::Html,
                "Autonomous custom elements must not specify the “is” attribute.",
                *span,
            ));
            return;
        }

        let v = is_value.trim();

        if !is_valid_custom_element_name(v) {
            out.push(Message::new(
                "html.is_attribute.invalid",
                Severity::Error,
                Category::Html,
                format!("Bad value “{v}” for attribute “is” on element “{name}”."),
                *span,
            ));
        }
    }
}

fn is_autonomous_custom_element_name(ctx: &ValidationContext, name: &str) -> bool {
    match ctx.format {
        html_inspector_core::InputFormat::Html => {
            is_valid_custom_element_name(ascii_lowercase_cow_if_needed(name).as_ref())
        }
        html_inspector_core::InputFormat::Xhtml => is_valid_custom_element_name(name),
    }
}

fn is_valid_custom_element_name(v: &str) -> bool {
    if v.is_empty() {
        return false;
    }
    if !v.as_bytes().iter().all(|b| {
        b.is_ascii_lowercase() || b.is_ascii_digit() || *b == b'-' || *b == b'.' || *b == b'_'
    }) {
        return false;
    }
    if !v.contains('-') {
        return false;
    }
    if !v.as_bytes()[0].is_ascii_lowercase() {
        return false;
    }
    // Reserved names (subset used in the suite).
    !matches!(
        v,
        "annotation-xml"
            | "color-profile"
            | "font-face"
            | "font-face-src"
            | "font-face-uri"
            | "font-face-format"
            | "font-face-name"
            | "missing-glyph"
    )
}
