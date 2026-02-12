use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct TextareaConstraints;

impl Rule for TextareaConstraints {
    fn id(&self) -> &'static str {
        "html.textarea.constraints"
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
        if !is(ctx, name, "textarea") {
            return;
        }

        let rows = attrs
            .iter()
            .find(|a| match ctx.format {
                html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case("rows"),
                html_inspector_core::InputFormat::Xhtml => a.name == "rows",
            })
            .and_then(|a| a.value.as_deref());
        if let Some(rows) = rows {
            let v = rows.trim();
            let parsed = v.parse::<u32>().ok();
            let ok = parsed.is_some_and(|n| n > 0);
            if !ok {
                out.push(Message::new(
                    "html.textarea.rows.positive",
                    Severity::Error,
                    Category::Html,
                    format!("Bad value “{rows}” for attribute “rows” on element “textarea”."),
                    *span,
                ));
            }
        }

        let cols = attrs
            .iter()
            .find(|a| match ctx.format {
                html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case("cols"),
                html_inspector_core::InputFormat::Xhtml => a.name == "cols",
            })
            .and_then(|a| a.value.as_deref());
        if let Some(cols) = cols {
            let v = cols.trim();
            let parsed = v.parse::<u32>().ok();
            let ok = parsed.is_some_and(|n| n > 0);
            if !ok {
                out.push(Message::new(
                    "html.textarea.cols.positive",
                    Severity::Error,
                    Category::Html,
                    format!("Bad value “{cols}” for attribute “cols” on element “textarea”."),
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
