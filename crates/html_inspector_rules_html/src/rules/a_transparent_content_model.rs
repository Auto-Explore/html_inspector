use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

use super::shared::{is_phrasing_element, normalize_name};

#[derive(Default)]
pub struct ATransparentContentModel {
    a_phrasing_context_stack: Vec<bool>,
}

impl Rule for ATransparentContentModel {
    fn id(&self) -> &'static str {
        "html.a.transparent_content_model"
    }

    fn interest(&self) -> Interest {
        Interest::START_TAG | Interest::END_TAG
    }

    fn on_event(
        &mut self,
        event: &ParseEvent,
        ctx: &mut ValidationContext,
        out: &mut dyn MessageSink,
    ) {
        match event {
            ParseEvent::StartTag {
                name,
                self_closing,
                span,
                ..
            } => {
                if ctx.name_is(name, "a") {
                    if *self_closing {
                        return;
                    }
                    let parent = ctx.current_parent();
                    let is_phrasing_ctx = parent.is_some_and(|p| is_phrasing_element(ctx, p));
                    self.a_phrasing_context_stack.push(is_phrasing_ctx);
                    return;
                }

                if self
                    .a_phrasing_context_stack
                    .last()
                    .copied()
                    .unwrap_or_default()
                    && !is_phrasing_element(ctx, name)
                {
                    let child = normalize_name(ctx, name);
                    out.push(Message::new(
                        "html.a.transparent.disallowed_child_in_phrasing",
                        Severity::Error,
                        Category::Html,
                        format!("Element “{child}” not allowed as child of “a” in this context. Note: The “a” element has a transparent content model; its allowed content is inherited from its parent element."),
                        *span,
                    ));
                }
            }
            ParseEvent::EndTag { name, .. } => {
                if ctx.name_is(name, "a") {
                    self.a_phrasing_context_stack.pop();
                }
            }
            _ => {}
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, _out: &mut dyn MessageSink) {
        self.a_phrasing_context_stack.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use html_inspector::{Config, InputFormat, RuleSet};
    use html_inspector_html::HtmlEventSource;

    fn codes(html: &str) -> Vec<String> {
        let src = HtmlEventSource::from_str("t", InputFormat::Html, html).unwrap();
        let report = html_inspector::validate_events(
            src,
            RuleSet::new().push(ATransparentContentModel::default()),
            Config::default(),
        )
        .unwrap();
        report.messages.into_iter().map(|m| m.code).collect()
    }

    #[test]
    fn a_transparent_model_only_restricts_children_in_phrasing_context() {
        assert!(
            codes(r#"<span><a><div></div></a></span>"#)
                .iter()
                .any(|c| c == "html.a.transparent.disallowed_child_in_phrasing")
        );
        assert!(
            !codes(r#"<div><a><div></div></a></div>"#)
                .iter()
                .any(|c| c == "html.a.transparent.disallowed_child_in_phrasing")
        );
    }
}
