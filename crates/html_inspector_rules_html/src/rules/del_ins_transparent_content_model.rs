use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

use super::shared::{is_phrasing_element, normalize_name};

#[derive(Default)]
pub struct DelInsTransparentContentModel {
    stack: Vec<TransparentCtx>,
}

#[derive(Clone, Copy, Debug)]
struct TransparentCtx {
    element: &'static str,
    phrasing_context: bool,
}

impl Rule for DelInsTransparentContentModel {
    fn id(&self) -> &'static str {
        "html.del_ins.transparent_content_model"
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
                if ctx.name_is(name, "del") || ctx.name_is(name, "ins") {
                    if *self_closing {
                        return;
                    }
                    let element = if ctx.name_is(name, "del") {
                        "del"
                    } else {
                        "ins"
                    };
                    let parent = ctx.current_parent();
                    let phrasing_context = parent.is_some_and(|p| is_phrasing_element(ctx, p));
                    self.stack.push(TransparentCtx {
                        element,
                        phrasing_context,
                    });
                    return;
                }

                if let Some(top) = self.stack.last().copied()
                    && top.phrasing_context
                    && !is_phrasing_element(ctx, name)
                {
                    let child = normalize_name(ctx, name);
                    out.push(Message::new(
                            "html.del_ins.transparent.disallowed_child_in_phrasing",
                            Severity::Error,
                            Category::Html,
                            format!(
                                "Element “{child}” not allowed as child of “{}” in this context. Note: The “{}” element has a transparent content model; its allowed content is inherited from its parent element.",
                                top.element, top.element,
                            ),
                            *span,
                        ));
                }
            }
            ParseEvent::EndTag { name, .. } => {
                if ctx.name_is(name, "del") || ctx.name_is(name, "ins") {
                    self.stack.pop();
                }
            }
            _ => {}
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, _out: &mut dyn MessageSink) {
        self.stack.clear();
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
            RuleSet::new().push(DelInsTransparentContentModel::default()),
            Config::default(),
        )
        .unwrap();
        report.messages.into_iter().map(|m| m.code).collect()
    }

    #[test]
    fn del_ins_transparent_model_only_restricts_children_in_phrasing_context() {
        assert!(
            codes(r#"<span><del><div></div></del></span>"#)
                .iter()
                .any(|c| c == "html.del_ins.transparent.disallowed_child_in_phrasing")
        );
        assert!(
            !codes(r#"<div><del><div></div></del></div>"#)
                .iter()
                .any(|c| c == "html.del_ins.transparent.disallowed_child_in_phrasing")
        );
    }
}
