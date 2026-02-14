use rustc_hash::FxHashSet;

use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, Span, ValidationContext,
};

#[derive(Default)]
pub struct InputListConstraints {
    datalist_ids: FxHashSet<String>,
    pending_lists: Vec<(String, Option<Span>)>,
}

impl Rule for InputListConstraints {
    fn id(&self) -> &'static str {
        "html.input.list_constraints"
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

        if is(ctx, name, "datalist") {
            if let Some(id) = attr_value(ctx, attrs, "id")
                && !id.is_empty() {
                    self.datalist_ids.insert(id.to_string());
                }
            return;
        }

        if !is(ctx, name, "input") {
            return;
        }

        let list = attr_value(ctx, attrs, "list");
        let Some(list) = list else { return };

        let input_type = attr_value(ctx, attrs, "type").unwrap_or("text");
        let input_type_lc = input_type.to_ascii_lowercase();

        let list_allowed = matches!(
            input_type_lc.as_str(),
            "color"
                | "date"
                | "datetime-local"
                | "email"
                | "month"
                | "number"
                | "range"
                | "search"
                | "tel"
                | "text"
                | "time"
                | "url"
                | "week"
        );
        if !list_allowed {
            out.push(Message::new(
                "html.input.list.disallowed_for_type",
                Severity::Error,
                Category::Html,
                "Attribute “list” is only allowed when the input type is “color”, “date”, “datetime-local”, “email”, “month”, “number”, “range”, “search”, “tel”, “text”, “time”, “url”, or “week”.",
                *span,
            ));
            return;
        }

        self.pending_lists.push((list.to_string(), *span));
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, out: &mut dyn MessageSink) {
        for (idref, span) in self.pending_lists.drain(..) {
            if !self.datalist_ids.contains(&idref) {
                out.push(Message::new(
                    "html.input.list.must_refer_to_datalist",
                    Severity::Error,
                    Category::Html,
                    "The “list” attribute of the “input” element must refer to a “datalist” element.",
                    span,
                ));
            }
        }
        self.datalist_ids.clear();
    }
}

fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    match ctx.format {
        html_inspector_core::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector_core::InputFormat::Xhtml => actual == expected,
    }
}

fn attr_value<'a>(
    ctx: &ValidationContext,
    attrs: &'a [html_inspector_core::Attribute],
    needle: &str,
) -> Option<&'a str> {
    attrs
        .iter()
        .find(|a| match ctx.format {
            html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case(needle),
            html_inspector_core::InputFormat::Xhtml => a.name == needle,
        })
        .and_then(|a| a.value.as_deref())
}
