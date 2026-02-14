use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

use super::foreign_content::{Namespace, namespace_for_next_start_tag};

#[derive(Default)]
pub struct HeadingSkipLevelError {
    last_heading: Option<HeadingInfo>,
    element_stack: Vec<ElementFrame>,
}

#[derive(Clone, Debug)]
struct HeadingInfo {
    name: String,
    computed_level: i32,
}

#[derive(Clone, Debug)]
struct ElementFrame {
    name: String,
    offset: i32,
    reset: bool,
}

impl Rule for HeadingSkipLevelError {
    fn id(&self) -> &'static str {
        "html.heading.skip_level"
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
                let is_html = namespace_for_next_start_tag(ctx, name) == Namespace::Html;
                let offset = if is_html {
                    parse_headingoffset(ctx, attrs)
                } else {
                    0
                };
                let reset = is_html && has_headingreset(ctx, attrs);
                self.element_stack.push(ElementFrame {
                    name: normalize_name(ctx, name),
                    offset,
                    reset,
                });

                if !is_html {
                    return;
                }

                let Some(base_level) = heading_level(ctx, name) else {
                    return;
                };
                let mut computed_level = base_level + current_headingoffset(&self.element_stack);
                if computed_level > 9 {
                    computed_level = 9;
                }
                let current_name = normalize_name(ctx, name);

                if let Some(prev) = &self.last_heading
                    && computed_level > prev.computed_level + 1
                {
                    let skipped = computed_level - prev.computed_level - 1;
                    let levels_word = if skipped == 1 { "level" } else { "levels" };
                    out.push(Message::new(
                            "html.heading.skip_level",
                            Severity::Error,
                            Category::Html,
                            format!(
                                "The heading “{current_name}” (with computed level {computed_level}) follows the heading “{}” (with computed level {}), skipping {skipped} heading {levels_word}.",
                                prev.name, prev.computed_level
                            ),
                            *span,
                        ));
                }

                self.last_heading = Some(HeadingInfo {
                    name: current_name,
                    computed_level,
                });
            }
            ParseEvent::EndTag { name, .. } => {
                let end_name = normalize_name(ctx, name);
                if let Some(pos) = self.element_stack.iter().rposition(|f| f.name == end_name) {
                    self.element_stack.truncate(pos);
                }
            }
            _ => {}
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, _out: &mut dyn MessageSink) {
        self.last_heading = None;
        self.element_stack.clear();
    }
}

fn current_headingoffset(stack: &[ElementFrame]) -> i32 {
    let mut total: i32 = 0;
    for frame in stack.iter().rev() {
        total = total.saturating_add(frame.offset);
        if frame.reset {
            break;
        }
    }
    total
}

fn parse_headingoffset(ctx: &ValidationContext, attrs: &[html_inspector::Attribute]) -> i32 {
    let raw = attrs
        .iter()
        .find(|a| match ctx.format {
            html_inspector::InputFormat::Html => a.name.eq_ignore_ascii_case("headingoffset"),
            html_inspector::InputFormat::Xhtml => a.name == "headingoffset",
        })
        .and_then(|a| a.value.as_deref());
    let Some(raw) = raw else { return 0 };
    let n = raw.trim().parse::<i32>().ok().unwrap_or(0);
    if n >= 0 { n } else { 0 }
}

fn has_headingreset(ctx: &ValidationContext, attrs: &[html_inspector::Attribute]) -> bool {
    attrs.iter().any(|a| match ctx.format {
        html_inspector::InputFormat::Html => a.name.eq_ignore_ascii_case("headingreset"),
        html_inspector::InputFormat::Xhtml => a.name == "headingreset",
    })
}

fn heading_level(ctx: &ValidationContext, name: &str) -> Option<i32> {
    for (h, level) in [
        ("h1", 1),
        ("h2", 2),
        ("h3", 3),
        ("h4", 4),
        ("h5", 5),
        ("h6", 6),
    ] {
        if is(ctx, name, h) {
            return Some(level);
        }
    }
    None
}

fn normalize_name(ctx: &ValidationContext, name: &str) -> String {
    match ctx.format {
        html_inspector::InputFormat::Html => name.to_ascii_lowercase(),
        html_inspector::InputFormat::Xhtml => name.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use html_inspector::{Config, InputFormat, RuleSet};
    use html_inspector_html::SimpleHtmlEventSource;

    #[test]
    fn headingreset_limits_offset_accumulation() {
        let html =
            "<h1>One</h1><div headingoffset=\"5\"><div headingreset><h2>Two</h2></div></div>";
        let src = SimpleHtmlEventSource::from_str("t", InputFormat::Html, html);
        let report = html_inspector::validate_events(
            src,
            RuleSet::new().push(HeadingSkipLevelError::default()),
            Config::default(),
        )
        .unwrap();
        assert!(
            !report
                .messages
                .iter()
                .any(|m| m.code == "html.heading.skip_level")
        );
    }

    #[test]
    fn computed_level_is_capped_at_nine() {
        let html = "<h1>One</h1><div headingoffset=\"8\"><h6>Six</h6></div>";
        let src = SimpleHtmlEventSource::from_str("t", InputFormat::Html, html);
        let report = html_inspector::validate_events(
            src,
            RuleSet::new().push(HeadingSkipLevelError::default()),
            Config::default(),
        )
        .unwrap();
        let msg = report
            .messages
            .iter()
            .find(|m| m.code == "html.heading.skip_level")
            .expect("expected skip-level error");
        assert!(msg.message.contains("computed level 9"));
    }
}

fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    match ctx.format {
        html_inspector::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector::InputFormat::Xhtml => actual == expected,
    }
}
