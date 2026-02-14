use rustc_hash::FxHashMap;

use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct DuplicateId {
    first: FxHashMap<String, Option<html_inspector_core::Span>>,
}

impl Rule for DuplicateId {
    fn id(&self) -> &'static str {
        "html.id.duplicate"
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
        let ParseEvent::StartTag { attrs, span, .. } = event else {
            return;
        };
        let Some(id) = ctx
            .attr_value(attrs, "id")
            .map(str::trim)
            .filter(|s| !s.is_empty())
        else {
            return;
        };

        // Match vnu.jar behavior: the htmlparser skips ID-uniqueness checks inside <template>
        // contents (TreeBuilder.isTemplateContents()).
        if ctx.has_ancestor("template") {
            return;
        }

        if let Some(first_span) = self.first.get(id).copied() {
            // Emit messages immediately, matching TreeBuilder behavior.
            out.push(Message::new(
                "html.id.duplicate",
                Severity::Error,
                Category::Html,
                format!("Duplicate ID “{id}”."),
                *span,
            ));
            out.push(Message::new(
                "html.id.duplicate.first",
                Severity::Warning,
                Category::Html,
                format!("The first occurrence of ID “{id}” was here."),
                first_span,
            ));
        } else {
            self.first.insert(id.to_string(), *span);
        }
    }

    fn on_finish(&mut self, ctx: &mut ValidationContext, out: &mut dyn MessageSink) {
        let _ = (ctx, out);
        self.first.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use html_inspector_core::{Attribute, Config, InputFormat, Span};

    #[derive(Default)]
    struct Sink(Vec<html_inspector_core::Message>);

    impl html_inspector_core::MessageSink for Sink {
        fn push(&mut self, msg: html_inspector_core::Message) {
            self.0.push(msg);
        }
    }

    fn start_tag_with_id(id: &str, span: Option<Span>) -> ParseEvent {
        ParseEvent::StartTag {
            name: "div".to_string(),
            attrs: vec![Attribute {
                name: "id".to_string(),
                value: Some(id.to_string()),
                span: None,
            }],
            self_closing: false,
            span,
        }
    }

    #[test]
    fn emits_duplicate_error_and_first_occurrence_immediately() {
        let mut rule = DuplicateId::default();
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink::default();

        let first_span = Some(Span::new(1, 2, 1, 2));
        rule.on_event(&start_tag_with_id("a", first_span), &mut ctx, &mut sink);
        assert!(sink.0.is_empty());

        let dup_span = Some(Span::new(5, 6, 1, 6));
        rule.on_event(&start_tag_with_id("a", dup_span), &mut ctx, &mut sink);
        assert_eq!(sink.0.len(), 2);
        assert_eq!(sink.0[0].code, "html.id.duplicate");
        assert_eq!(sink.0[0].span, dup_span);
        assert_eq!(sink.0[1].code, "html.id.duplicate.first");
        assert_eq!(sink.0[1].span, first_span);

        // Ensure we don't emit any additional messages on finish.
        rule.on_finish(&mut ctx, &mut sink);
        assert_eq!(sink.0.len(), 2);
    }

    #[test]
    fn emits_first_occurrence_warning_without_span_when_first_span_is_missing() {
        let mut rule = DuplicateId::default();
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink::default();

        rule.on_event(&start_tag_with_id("a", None), &mut ctx, &mut sink);
        let dup_span = Some(Span::new(5, 6, 1, 6));
        rule.on_event(&start_tag_with_id("a", dup_span), &mut ctx, &mut sink);
        assert_eq!(sink.0.len(), 2);
        assert_eq!(sink.0[0].code, "html.id.duplicate");
        assert_eq!(sink.0[1].code, "html.id.duplicate.first");
        assert_eq!(sink.0[1].span, None);

        rule.on_finish(&mut ctx, &mut sink);
        assert_eq!(sink.0.len(), 2);
    }
}
