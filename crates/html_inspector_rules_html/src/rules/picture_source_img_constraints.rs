use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, Span, ValidationContext,
};

use super::shared::starts_with_ascii_ci;
use super::srcset_microsyntax::{self, SrcsetMode};

#[derive(Default)]
pub struct PictureSourceImgConstraints {
    stack: Vec<PictureState>,
}

#[derive(Default)]
struct PictureState {
    pending_width_srcset_without_sizes: Vec<Option<Span>>,
    pending_auto_sizes_sources: Vec<Option<Span>>,
}

impl Rule for PictureSourceImgConstraints {
    fn id(&self) -> &'static str {
        "html.picture.source_img_constraints"
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
                name, attrs, span, ..
            } => {
                if is(ctx, name, "picture") {
                    self.stack.push(PictureState::default());
                    return;
                }

                let Some(parent) = ctx.current_parent() else {
                    return;
                };
                if !is(ctx, parent, "picture") {
                    return;
                }
                let Some(state) = self.stack.last_mut() else {
                    return;
                };

                if is(ctx, name, "source") {
                    if !has_attr(ctx, attrs, "srcset") {
                        out.push(Message::new(
                            "html.source.missing_srcset",
                            Severity::Error,
                            Category::Html,
                            "Element “source” is missing required attribute “srcset”.",
                            *span,
                        ));
                        return;
                    }

                    let srcset = attr_value_or_empty(ctx, attrs, "srcset");
                    let sizes_present = has_attr(ctx, attrs, "sizes");
                    let srcset_valid =
                        !srcset_microsyntax::is_invalid_srcset(srcset, sizes_present);
                    if !srcset_valid {
                        out.push(Message::new(
                            "html.source.srcset.invalid",
                            Severity::Error,
                            Category::Html,
                            format!(
                                "Bad value “{srcset}” for attribute “srcset” on element “source”."
                            ),
                            *span,
                        ));
                    }

                    if srcset_valid
                        && !sizes_present
                        && matches!(
                            srcset_microsyntax::srcset_mode(srcset),
                            Some(SrcsetMode::Width)
                        )
                    {
                        state.pending_width_srcset_without_sizes.push(*span);
                    }

                    if sizes_present
                        && starts_with_ascii_ci(
                            attr_value_or_empty(ctx, attrs, "sizes").trim_start(),
                            "auto",
                        )
                    {
                        state.pending_auto_sizes_sources.push(*span);
                    }
                    return;
                }

                if is(ctx, name, "img") {
                    let img_loading_lazy =
                        attr_value_or_empty(ctx, attrs, "loading").eq_ignore_ascii_case("lazy");
                    let img_lazy_with_dims = img_loading_lazy
                        && has_attr(ctx, attrs, "width")
                        && has_attr(ctx, attrs, "height");

                    if !state.pending_width_srcset_without_sizes.is_empty() && !img_lazy_with_dims {
                        for pending_span in state.pending_width_srcset_without_sizes.drain(..) {
                            out.push(Message::new(
                                "html.source.srcset.width_descriptor_requires_sizes",
                                Severity::Error,
                                Category::Html,
                                "When the “srcset” attribute has any image candidate string with a width descriptor, the “sizes” attribute must also be specified.",
                                pending_span,
                            ));
                        }
                    } else {
                        state.pending_width_srcset_without_sizes.clear();
                    }

                    if !state.pending_auto_sizes_sources.is_empty() && !img_loading_lazy {
                        for pending_span in state.pending_auto_sizes_sources.drain(..) {
                            out.push(Message::new(
                                "html.source.sizes.auto_requires_img_lazy",
                                Severity::Error,
                                Category::Html,
                                "The “sizes” attribute value starting with “auto” is only valid for lazy-loaded images. The “img” element must have a “loading” attribute set to “lazy”.",
                                pending_span,
                            ));
                        }
                    } else {
                        state.pending_auto_sizes_sources.clear();
                    }
                }
            }
            ParseEvent::EndTag { name, .. } => {
                if is(ctx, name, "picture") {
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

fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    match ctx.format {
        html_inspector_core::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector_core::InputFormat::Xhtml => actual == expected,
    }
}

fn has_attr(
    ctx: &ValidationContext,
    attrs: &[html_inspector_core::Attribute],
    needle: &str,
) -> bool {
    attrs.iter().any(|a| match ctx.format {
        html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case(needle),
        html_inspector_core::InputFormat::Xhtml => a.name == needle,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use html_inspector_core::{
        Attribute, Config, EventSource, InputFormat, ParseEvent, RuleSet, ValidatorError,
    };

    struct VecSource {
        name: String,
        format: InputFormat,
        events: std::vec::IntoIter<ParseEvent>,
    }

    impl VecSource {
        fn new(format: InputFormat, events: Vec<ParseEvent>) -> Self {
            Self {
                name: "vec".to_string(),
                format,
                events: events.into_iter(),
            }
        }
    }

    impl EventSource for VecSource {
        fn source_name(&self) -> &str {
            &self.name
        }
        fn format(&self) -> InputFormat {
            self.format
        }
        fn next_event(&mut self) -> Result<Option<ParseEvent>, ValidatorError> {
            Ok(self.events.next())
        }
    }

    fn attr(name: &str, value: Option<&str>) -> Attribute {
        Attribute {
            name: name.to_string(),
            value: value.map(|v| v.to_string()),
            span: None,
        }
    }

    #[test]
    fn auto_sizes_requires_img_loading_lazy_case_insensitively() {
        let src = VecSource::new(
            InputFormat::Html,
            vec![
                ParseEvent::StartTag {
                    name: "picture".to_string(),
                    attrs: vec![],
                    self_closing: false,
                    span: None,
                },
                ParseEvent::StartTag {
                    name: "source".to_string(),
                    attrs: vec![
                        attr("srcset", Some("a 1w")),
                        attr("sizes", Some("AUTO 100vw")),
                    ],
                    self_closing: true,
                    span: None,
                },
                ParseEvent::StartTag {
                    name: "img".to_string(),
                    attrs: vec![],
                    self_closing: true,
                    span: None,
                },
            ],
        );

        let rules = RuleSet::new().push(PictureSourceImgConstraints::default());
        let report = html_inspector_core::validate_events(src, rules, Config::default()).unwrap();
        assert!(report
            .messages
            .iter()
            .any(|m| m.code == "html.source.sizes.auto_requires_img_lazy"));
    }
}

fn attr_value_or_empty<'a>(
    ctx: &ValidationContext,
    attrs: &'a [html_inspector_core::Attribute],
    needle: &str,
) -> &'a str {
    attrs
        .iter()
        .find(|a| match ctx.format {
            html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case(needle),
            html_inspector_core::InputFormat::Xhtml => a.name == needle,
        })
        .and_then(|a| a.value.as_deref())
        .unwrap_or("")
}
