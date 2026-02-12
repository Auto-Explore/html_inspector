use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct RelTypoConstraints;

impl Rule for RelTypoConstraints {
    fn id(&self) -> &'static str {
        "html.rel.typo"
    }

    fn max_severity(&self) -> Severity {
        Severity::Info
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
        let rel = attrs
            .iter()
            .find(|a| match ctx.format {
                html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case("rel"),
                html_inspector_core::InputFormat::Xhtml => a.name == "rel",
            })
            .and_then(|a| a.value.as_deref())
            .map(|s| s.trim())
            .filter(|s| !s.is_empty());

        let Some(rel) = rel else { return };

        let suggestion = match rel {
            "alternat" => Some("alternate"),
            "authr" => Some("author"),
            "canonicl" => Some("canonical"),
            "styleshet" => Some("stylesheet"),
            _ => None,
        };

        let Some(suggestion) = suggestion else { return };

        out.push(Message::new(
            "html.rel.typo",
            Severity::Info,
            Category::Html,
            format!(
                "Bad value “{rel}” for attribute “rel” on element “{name}”: Bad list of link-type keywords: Typo for “{suggestion}”?"
            ),
            *span,
        ));
    }
}
