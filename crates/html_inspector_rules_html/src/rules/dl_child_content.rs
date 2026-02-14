use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct DlChildContent;

impl Rule for DlChildContent {
    fn id(&self) -> &'static str {
        "html.dl.child_content"
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
        let ParseEvent::StartTag { name, span, .. } = event else {
            return;
        };

        let Some(parent) = ctx.current_parent() else {
            return;
        };

        let in_dl = match ctx.format {
            html_inspector::InputFormat::Html => parent.eq_ignore_ascii_case("dl"),
            html_inspector::InputFormat::Xhtml => parent == "dl",
        };
        if !in_dl {
            return;
        }

        let allowed = match ctx.format {
            html_inspector::InputFormat::Html => {
                name.eq_ignore_ascii_case("dt")
                    || name.eq_ignore_ascii_case("dd")
                    || name.eq_ignore_ascii_case("div")
                    || name.eq_ignore_ascii_case("script")
                    || name.eq_ignore_ascii_case("template")
            }
            html_inspector::InputFormat::Xhtml => {
                name == "dt"
                    || name == "dd"
                    || name == "div"
                    || name == "script"
                    || name == "template"
            }
        };
        if allowed {
            return;
        }

        out.push(Message::new(
            "html.dl.child.invalid",
            Severity::Error,
            Category::Html,
            format!("Element “{name}” not allowed as child of “dl” in this context."),
            *span,
        ));
    }
}
