use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, Span, ValidationContext,
};

use super::foreign_content::{Namespace, namespace_for_next_start_tag};

const H1_WARNING_MESSAGE: &str = "Consider using the “h1” element as a top-level heading only — or else use the “headingoffset” attribute (otherwise, all “h1” elements are treated as top-level headings by many screen readers and other tools).";

#[derive(Default)]
pub struct H1TopLevelHeadingWarning {
    has_headingoffset: bool,
    has_top_level_h1: bool,
    second_level_h1s: Vec<Option<Span>>,
}

impl Rule for H1TopLevelHeadingWarning {
    fn id(&self) -> &'static str {
        "html.h1.top_level_heading_warning"
    }

    fn max_severity(&self) -> Severity {
        Severity::Warning
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

        // Match vnu.jar behavior: schema/Assertions checks are skipped inside <template> contents.
        if ctx.has_ancestor("template") {
            return;
        }

        if !self.has_headingoffset && ctx.has_attr(attrs, "headingoffset") {
            self.has_headingoffset = true;
        }

        if namespace_for_next_start_tag(ctx, name) != Namespace::Html {
            return;
        }

        if !ctx.name_is(name, "h1") {
            return;
        }

        if self.has_headingoffset {
            return;
        }

        match sectioning_depth(ctx) {
            depth if depth > 1 => emit_h1_warning(out, *span),
            1 => self.second_level_h1s.push(*span),
            _ => self.has_top_level_h1 = true,
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, out: &mut dyn MessageSink) {
        if self.has_top_level_h1 && !self.has_headingoffset {
            for span in self.second_level_h1s.drain(..) {
                emit_h1_warning(out, span);
            }
        } else {
            self.second_level_h1s.clear();
        }

        self.has_headingoffset = false;
        self.has_top_level_h1 = false;
    }
}

fn emit_h1_warning(out: &mut dyn MessageSink, span: Option<Span>) {
    out.push(Message::new(
        "html.h1.top_level_heading_warning",
        Severity::Warning,
        Category::Html,
        H1_WARNING_MESSAGE,
        span,
    ));
}

fn sectioning_depth(ctx: &ValidationContext) -> usize {
    ctx.open_elements()
        .iter()
        .filter(|n| is_sectioning_element(ctx, n.as_str()))
        .count()
}

fn is_sectioning_element(ctx: &ValidationContext, name: &str) -> bool {
    ctx.name_is(name, "article")
        || ctx.name_is(name, "aside")
        || ctx.name_is(name, "nav")
        || ctx.name_is(name, "section")
}

#[cfg(test)]
mod tests {
    use super::*;

    use html_inspector_core::{Config, InputFormat, RuleSet};
    use html_inspector_html::HtmlEventSource;

    fn validate_html(html: &str) -> Vec<html_inspector_core::Message> {
        let src = HtmlEventSource::from_str("t", InputFormat::Html, html).unwrap();
        html_inspector_core::validate_events(
            src,
            RuleSet::new().push(H1TopLevelHeadingWarning::default()),
            Config::default(),
        )
        .unwrap()
        .messages
    }

    #[test]
    fn warns_for_second_level_h1_when_top_level_h1_exists() {
        let html = "<h1>Top</h1><section><h1>Nested</h1></section>";
        let msgs = validate_html(html);
        assert!(
            msgs.iter()
                .any(|m| m.code == "html.h1.top_level_heading_warning")
        );
    }

    #[test]
    fn does_not_warn_when_only_second_level_h1_exists() {
        let html = "<section><h1>Nested</h1></section>";
        let msgs = validate_html(html);
        assert!(
            !msgs
                .iter()
                .any(|m| m.code == "html.h1.top_level_heading_warning")
        );
    }

    #[test]
    fn warns_immediately_for_deeper_sectioning_h1() {
        let html = "<section><section><h1>Nested</h1></section></section>";
        let msgs = validate_html(html);
        assert_eq!(
            msgs.iter()
                .filter(|m| m.code == "html.h1.top_level_heading_warning")
                .count(),
            1
        );
    }

    #[test]
    fn headingoffset_suppresses_h1_top_level_warnings_even_if_seen_later() {
        let html = "<h1>Top</h1><section><h1>Nested</h1></section><div headingoffset=\"1\"></div>";
        let msgs = validate_html(html);
        assert!(
            !msgs
                .iter()
                .any(|m| m.code == "html.h1.top_level_heading_warning")
        );
    }

    #[test]
    fn h1_warnings_are_skipped_inside_template_contents() {
        let html = "<h1>Top</h1><template><section><h1>Nested</h1></section></template>";
        let msgs = validate_html(html);
        assert!(
            !msgs
                .iter()
                .any(|m| m.code == "html.h1.top_level_heading_warning")
        );
    }
}
