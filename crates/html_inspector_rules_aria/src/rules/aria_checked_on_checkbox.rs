use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct AriaCheckedOnCheckbox;

impl Rule for AriaCheckedOnCheckbox {
    fn id(&self) -> &'static str {
        "aria.aria_checked.disallowed_on_input_checkbox"
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

        let is_input = match ctx.format {
            html_inspector_core::InputFormat::Html => name.eq_ignore_ascii_case("input"),
            html_inspector_core::InputFormat::Xhtml => name == "input",
        };
        if !is_input {
            return;
        }

        let has_aria_checked = attrs.iter().any(|a| match ctx.format {
            html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case("aria-checked"),
            html_inspector_core::InputFormat::Xhtml => a.name == "aria-checked",
        });
        if !has_aria_checked {
            return;
        }

        let type_value = attrs
            .iter()
            .find(|a| match ctx.format {
                html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case("type"),
                html_inspector_core::InputFormat::Xhtml => a.name == "type",
            })
            .and_then(|a| a.value.as_deref())
            .unwrap_or("");

        if type_value.eq_ignore_ascii_case("checkbox") {
            out.push(Message::new(
                "aria.aria_checked.disallowed_on_input_checkbox",
                Severity::Error,
                Category::Aria,
                "The “aria-checked” attribute must not be used on an “input” element which has a “type” attribute whose value is “checkbox”.",
                *span,
            ));
        } else if type_value.eq_ignore_ascii_case("radio") {
            out.push(Message::new(
                "aria.aria_checked.disallowed_on_input_radio",
                Severity::Error,
                Category::Aria,
                "The “aria-checked” attribute must not be used on an “input” element which has a “type” attribute whose value is “radio”.",
                *span,
            ));
        }
    }
}
