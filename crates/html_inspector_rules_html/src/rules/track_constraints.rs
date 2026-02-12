use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct TrackConstraints {
    media_stack: Vec<MediaState>,
}

#[derive(Clone, Copy, Debug, Default)]
struct MediaState {
    saw_default_track: bool,
}

impl Rule for TrackConstraints {
    fn id(&self) -> &'static str {
        "html.track.constraints"
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
                attrs,
                span,
                self_closing,
            } => {
                if is_media_element(ctx, name) && !*self_closing {
                    self.media_stack.push(MediaState::default());
                    return;
                }

                if !is(ctx, name, "track") {
                    return;
                }

                let label = attrs
                    .iter()
                    .find(|a| attr_name_matches(ctx.format, &a.name, "label"))
                    .and_then(|a| a.value.as_deref());

                if let Some(label) = label {
                    if label.trim().is_empty() {
                        out.push(Message::new(
                            "html.track.label.non_empty",
                            Severity::Error,
                            Category::Html,
                            "Attribute “label” for element “track” must have non-empty value.",
                            *span,
                        ));
                    }
                }

                let has_default = attrs
                    .iter()
                    .any(|a| attr_name_matches(ctx.format, &a.name, "default"));
                if has_default {
                    if let Some(state) = self.media_stack.last_mut() {
                        if state.saw_default_track {
                            out.push(Message::new(
                                "html.track.default.multiple",
                                Severity::Error,
                                Category::Html,
                                "The “default” attribute must not occur on more than one “track” element within the same “audio” or “video” element.",
                                *span,
                            ));
                        } else {
                            state.saw_default_track = true;
                        }
                    }
                }
            }
            ParseEvent::EndTag { name, .. } => {
                if is_media_element(ctx, name) {
                    let _ = self.media_stack.pop();
                }
            }
            _ => {}
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, _out: &mut dyn MessageSink) {
        self.media_stack.clear();
    }
}

fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    match ctx.format {
        html_inspector_core::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector_core::InputFormat::Xhtml => actual == expected,
    }
}

#[inline]
fn is_media_element(ctx: &ValidationContext, name: &str) -> bool {
    is(ctx, name, "audio") || is(ctx, name, "video")
}

#[inline]
fn attr_name_matches(
    format: html_inspector_core::InputFormat,
    actual: &str,
    expected: &str,
) -> bool {
    match format {
        html_inspector_core::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector_core::InputFormat::Xhtml => actual == expected,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use html_inspector_core::{Config, InputFormat};

    struct Sink(Vec<html_inspector_core::Message>);
    impl html_inspector_core::MessageSink for Sink {
        fn push(&mut self, msg: html_inspector_core::Message) {
            self.0.push(msg);
        }
    }

    #[test]
    fn xhtml_label_and_default_branches_execute_inside_media_element() {
        let mut rule = TrackConstraints::default();
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        let mut sink = Sink(Vec::new());

        rule.on_event(
            &ParseEvent::StartTag {
                name: "audio".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            &mut ctx,
            &mut sink,
        );

        rule.on_event(
            &ParseEvent::StartTag {
                name: "track".to_string(),
                attrs: vec![
                    html_inspector_core::Attribute {
                        name: "label".to_string(),
                        value: Some("   ".to_string()),
                        span: None,
                    },
                    html_inspector_core::Attribute {
                        name: "default".to_string(),
                        value: None,
                        span: None,
                    },
                ],
                self_closing: true,
                span: None,
            },
            &mut ctx,
            &mut sink,
        );

        assert!(sink
            .0
            .iter()
            .any(|m| m.code == "html.track.label.non_empty"));

        rule.on_event(
            &ParseEvent::Text {
                text: "x".to_string(),
                span: None,
            },
            &mut ctx,
            &mut sink,
        );
    }

    #[test]
    fn html_branch_is_case_insensitive_for_element_and_attribute_names() {
        let mut rule = TrackConstraints::default();
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink(Vec::new());

        rule.on_event(
            &ParseEvent::StartTag {
                name: "AuDiO".to_string(),
                attrs: vec![],
                self_closing: false,
                span: None,
            },
            &mut ctx,
            &mut sink,
        );

        rule.on_event(
            &ParseEvent::StartTag {
                name: "TrAcK".to_string(),
                attrs: vec![
                    html_inspector_core::Attribute {
                        name: "LaBeL".to_string(),
                        value: Some("   ".to_string()),
                        span: None,
                    },
                    html_inspector_core::Attribute {
                        name: "DeFaUlT".to_string(),
                        value: None,
                        span: None,
                    },
                ],
                self_closing: true,
                span: None,
            },
            &mut ctx,
            &mut sink,
        );

        rule.on_event(
            &ParseEvent::StartTag {
                name: "track".to_string(),
                attrs: vec![html_inspector_core::Attribute {
                    name: "default".to_string(),
                    value: None,
                    span: None,
                }],
                self_closing: true,
                span: None,
            },
            &mut ctx,
            &mut sink,
        );

        assert!(sink
            .0
            .iter()
            .any(|m| m.code == "html.track.label.non_empty"));
        assert!(sink
            .0
            .iter()
            .any(|m| m.code == "html.track.default.multiple"));
    }
}
