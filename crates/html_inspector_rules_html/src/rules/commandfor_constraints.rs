use rustc_hash::FxHashSet;

use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct CommandforConstraints {
    ids: FxHashSet<String>,
    refs: Vec<(String, Option<html_inspector_core::Span>)>,
}

impl Rule for CommandforConstraints {
    fn id(&self) -> &'static str {
        "html.commandfor.constraints"
    }

    fn interest(&self) -> Interest {
        Interest::START_TAG | Interest::FINISH
    }

    fn init(&mut self, _ctx: &ValidationContext) {
        self.ids.clear();
        self.refs.clear();
    }

    fn on_event(
        &mut self,
        event: &ParseEvent,
        ctx: &mut ValidationContext,
        _out: &mut dyn MessageSink,
    ) {
        let ParseEvent::StartTag { attrs, span, .. } = event else {
            return;
        };

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
            self.ids.insert(id.to_string());
        }

        if let Some(cmdfor) = attrs
            .iter()
            .find(|a| match ctx.format {
                html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case("commandfor"),
                html_inspector_core::InputFormat::Xhtml => a.name == "commandfor",
            })
            .and_then(|a| a.value.as_deref())
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
        {
            self.refs.push((cmdfor.to_string(), *span));
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, out: &mut dyn MessageSink) {
        for (idref, span) in &self.refs {
            if !self.ids.contains(idref) {
                out.push(Message::new(
                    "html.commandfor.idref.missing",
                    Severity::Error,
                    Category::Html,
                    "The value of the “commandfor” attribute of the “button” element must be the ID of an element in the same tree as the “button” with the “commandfor” attribute.",
                    *span,
                ));
                break;
            }
        }
    }
}
