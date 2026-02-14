use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct TargetBrowsingContextConstraints;

impl Rule for TargetBrowsingContextConstraints {
    fn id(&self) -> &'static str {
        "html.target.browsing_context"
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

        let target = attrs
            .iter()
            .find(|a| match ctx.format {
                html_inspector::InputFormat::Html => a.name.eq_ignore_ascii_case("target"),
                html_inspector::InputFormat::Xhtml => a.name == "target",
            })
            .and_then(|a| a.value.as_deref());

        let Some(target) = target else { return };
        let v = target.trim();
        if v.is_empty() {
            out.push(Message::new(
                "html.target.empty",
                Severity::Error,
                Category::Html,
                "Bad value “” for attribute “target” on element “a”.",
                *span,
            ));
            return;
        }

        if let Some(rest) = v.strip_prefix('_') {
            let allowed = rest.eq_ignore_ascii_case("blank")
                || rest.eq_ignore_ascii_case("self")
                || rest.eq_ignore_ascii_case("parent")
                || rest.eq_ignore_ascii_case("top");
            if !allowed {
                out.push(Message::new(
                    "html.target.underscore_disallowed",
                    Severity::Error,
                    Category::Html,
                    format!("Bad value “{v}” for attribute “target” on element “{name}”."),
                    *span,
                ));
            }
        }
    }
}
