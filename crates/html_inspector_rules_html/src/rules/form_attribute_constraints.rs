use std::collections::HashSet;

use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct FormAttributeConstraints {
    form_ids: HashSet<String>,
    form_refs: Vec<(String, Option<html_inspector_core::Span>)>,
}

impl Rule for FormAttributeConstraints {
    fn id(&self) -> &'static str {
        "html.form_attribute.constraints"
    }

    fn interest(&self) -> Interest {
        Interest::START_TAG | Interest::FINISH
    }

    fn init(&mut self, _ctx: &ValidationContext) {
        self.form_ids.clear();
        self.form_refs.clear();
    }

    fn on_event(
        &mut self,
        event: &ParseEvent,
        ctx: &mut ValidationContext,
        _out: &mut dyn MessageSink,
    ) {
        let ParseEvent::StartTag {
            name, attrs, span, ..
        } = event
        else {
            return;
        };

        if is(ctx, name, "form") {
            if let Some(id) = attrs
                .iter()
                .find(|a| match ctx.format {
                    html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case("id"),
                    html_inspector_core::InputFormat::Xhtml => a.name == "id",
                })
                .and_then(|a| a.value.as_deref())
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
            {
                self.form_ids.insert(id.to_string());
            }
        }

        if let Some(form) = attrs
            .iter()
            .find(|a| match ctx.format {
                html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case("form"),
                html_inspector_core::InputFormat::Xhtml => a.name == "form",
            })
            .and_then(|a| a.value.as_deref())
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
        {
            self.form_refs.push((form.to_string(), *span));
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, out: &mut dyn MessageSink) {
        for (idref, span) in &self.form_refs {
            if !self.form_ids.contains(idref) {
                out.push(Message::new(
                    "html.form_attribute.not_form",
                    Severity::Error,
                    Category::Html,
                    "The “form” attribute must refer to a form element.",
                    *span,
                ));
            }
        }
    }
}

fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    match ctx.format {
        html_inspector_core::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector_core::InputFormat::Xhtml => actual == expected,
    }
}
