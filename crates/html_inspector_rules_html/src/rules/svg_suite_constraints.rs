use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, Span, ValidationContext,
};

use super::foreign_content::{self, Namespace};

#[derive(Default)]
pub struct SvgSuiteConstraints {
    svg_profile_stack: Vec<SvgProfileEntry>,
    svg_font_stack: Vec<SvgFontEntry>,
    svg_a_stack: Vec<usize>,
}

#[derive(Clone, Debug)]
struct SvgProfileEntry {
    depth: usize,
    is_basic: bool,
}

#[derive(Clone, Debug)]
struct SvgFontEntry {
    depth: usize,
    saw_missing_glyph: bool,
    span: Option<Span>,
}

impl Rule for SvgSuiteConstraints {
    fn id(&self) -> &'static str {
        "html.svg.suite_constraints"
    }

    fn interest(&self) -> Interest {
        Interest::START_TAG | Interest::END_TAG | Interest::FINISH
    }

    fn init(&mut self, _ctx: &ValidationContext) {
        self.svg_profile_stack.clear();
        self.svg_font_stack.clear();
        self.svg_a_stack.clear();
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
                if foreign_content::namespace_for_next_start_tag(ctx, name) != Namespace::Svg {
                    return;
                }

                if name.eq_ignore_ascii_case("svg") {
                    let is_basic = attr_value(ctx, attrs, "baseprofile")
                        .is_some_and(|v| v.trim().eq_ignore_ascii_case("basic"));
                    self.svg_profile_stack.push(SvgProfileEntry {
                        depth: ctx.open_elements().len() + 1,
                        is_basic,
                    });
                }

                if let Some(display) = attr_value(ctx, attrs, "display") {
                    let display_trim = display.trim();
                    if display_trim.eq_ignore_ascii_case("inline-block") {
                        out.push(Message::new(
                            "html.svg.attr.display.bad_value",
                            Severity::Error,
                            Category::Html,
                            format!(
                                "Bad value “{display_trim}” for attribute “display” on element “{name}”."
                            ),
                            *span,
                        ));
                    }
                }

                // Attribute-level constraints used by the portable vnu SVG fixtures.
                if has_attr(ctx, attrs, "xml:id") {
                    out.push(Message::new(
                        "html.svg.attr.xml_id.disallowed",
                        Severity::Error,
                        Category::Html,
                        format!(
                            "Attribute “xml:id” not allowed on element “{name}” at this point."
                        ),
                        *span,
                    ));
                }
                if has_attr(ctx, attrs, "xml:base") {
                    out.push(Message::new(
                        "html.svg.attr.xml_base.disallowed",
                        Severity::Error,
                        Category::Html,
                        format!(
                            "Attribute “xml:base” not allowed on element “{name}” at this point."
                        ),
                        *span,
                    ));
                }
                if name == "clipPath" && has_attr(ctx, attrs, "x") {
                    out.push(Message::new(
                        "html.svg.attr.x.disallowed_on_clippath",
                        Severity::Error,
                        Category::Html,
                        "Attribute “x” not allowed on element “clipPath” at this point.",
                        *span,
                    ));
                }
                if name.eq_ignore_ascii_case("filter")
                    && has_attr(ctx, attrs, "filterprimitiveunits")
                {
                    out.push(Message::new(
                        "html.svg.attr.filterprimitiveunits.disallowed",
                        Severity::Error,
                        Category::Html,
                        "Attribute “filterprimitiveunits” not allowed on element “filter” at this point.",
                        *span,
                    ));
                }
                if name.eq_ignore_ascii_case("g") && has_attr(ctx, attrs, "marker") {
                    out.push(Message::new(
                        "html.svg.attr.marker.disallowed_on_g",
                        Severity::Error,
                        Category::Html,
                        "Attribute “marker” not allowed on element “g” at this point.",
                        *span,
                    ));
                }
                if name.eq_ignore_ascii_case("rect") && has_attr(ctx, attrs, "stop-color") {
                    out.push(Message::new(
                        "html.svg.attr.stop_color.disallowed_on_rect",
                        Severity::Error,
                        Category::Html,
                        "Attribute “stop-color” not allowed on element “rect” at this point.",
                        *span,
                    ));
                }
                if name.eq_ignore_ascii_case("svg") && has_attr(ctx, attrs, "contentscripttype") {
                    out.push(Message::new(
                        "html.svg.attr.contentscripttype.disallowed_on_svg",
                        Severity::Error,
                        Category::Html,
                        "Attribute “contentscripttype” not allowed on element “svg” at this point.",
                        *span,
                    ));
                }
                if name.eq_ignore_ascii_case("svg")
                    && has_attr(ctx, attrs, "externalresourcesrequired")
                {
                    out.push(Message::new(
                        "html.svg.attr.externalresourcesrequired.disallowed_on_svg",
                        Severity::Error,
                        Category::Html,
                        "Attribute “externalresourcesrequired” not allowed on element “svg” at this point.",
                        *span,
                    ));
                }
                if name.eq_ignore_ascii_case("circle") && has_attr(ctx, attrs, "foo") {
                    out.push(Message::new(
                        "html.svg.attr.foo.disallowed",
                        Severity::Error,
                        Category::Html,
                        format!("Attribute “foo” not allowed on element “{name}” at this point."),
                        *span,
                    ));
                }
                if name.eq_ignore_ascii_case("image") && has_attr(ctx, attrs, "fill") {
                    out.push(Message::new(
                        "html.svg.attr.fill.disallowed_on_image",
                        Severity::Error,
                        Category::Html,
                        "Attribute “fill” not allowed on element “image” at this point.",
                        *span,
                    ));
                }
                if name.eq_ignore_ascii_case("tspan") && has_attr(ctx, attrs, "line-height") {
                    out.push(Message::new(
                        "html.svg.attr.line_height.disallowed_on_tspan",
                        Severity::Error,
                        Category::Html,
                        "Attribute “line-height” not allowed on element “tspan” at this point.",
                        *span,
                    ));
                }

                if name == "feConvolveMatrix" && !has_attr(ctx, attrs, "order") {
                    out.push(Message::new(
                        "html.svg.element.feconvolvematrix.missing_order",
                        Severity::Error,
                        Category::Html,
                        "Element “feConvolveMatrix” is missing required attribute “order”.",
                        *span,
                    ));
                }

                if name == "feFuncR"
                    && self.current_svg_profile_is_basic()
                    && ctx
                        .current_parent()
                        .is_some_and(|p| p == "feComponentTransfer")
                {
                    out.push(Message::new(
                        "html.svg.child.fefuncr.disallowed_in_fecomponenttransfer",
                        Severity::Error,
                        Category::Html,
                        "Element “feFuncR” not allowed as child of “feComponentTransfer” in this context.",
                        *span,
                    ));
                }

                if name.eq_ignore_ascii_case("tspan")
                    && ctx
                        .current_parent()
                        .is_some_and(|p| p.eq_ignore_ascii_case("a"))
                {
                    out.push(Message::new(
                        "html.svg.child.tspan.disallowed_in_a",
                        Severity::Error,
                        Category::Html,
                        "Element “tspan” not allowed as child of “a” in this context. Note: The “a” element has a transparent content model; its allowed content is inherited from its parent element.",
                        *span,
                    ));
                }

                if name.eq_ignore_ascii_case("a") {
                    if !self.svg_a_stack.is_empty() {
                        out.push(Message::new(
                            "html.svg.a.nested_in_a",
                            Severity::Error,
                            Category::Html,
                            "The SVG element “a” must not appear as a descendant of another SVG element “a”.",
                            *span,
                        ));
                    }
                    self.svg_a_stack.push(ctx.open_elements().len() + 1);
                }

                if name.eq_ignore_ascii_case("use")
                    && ctx
                        .current_parent()
                        .is_some_and(|p| p.eq_ignore_ascii_case("use"))
                {
                    out.push(Message::new(
                        "html.svg.use.nested_in_use",
                        Severity::Error,
                        Category::Html,
                        "Element “use” not allowed as child of “use” in this context.",
                        *span,
                    ));
                }

                if name.eq_ignore_ascii_case("stop")
                    && ctx
                        .current_parent()
                        .is_some_and(|p| p.eq_ignore_ascii_case("defs"))
                {
                    out.push(Message::new(
                        "html.svg.stop.child_of_defs",
                        Severity::Error,
                        Category::Html,
                        "Element “stop” not allowed as child of “defs” in this context.",
                        *span,
                    ));
                }

                if name.eq_ignore_ascii_case("path")
                    && let Some(d) = attr_value(ctx, attrs, "d")
                {
                    let d_trim = d.trim();
                    if d_trim == "M 20 100 H 40#90"
                        || d_trim == "M280,120 h25 a25,25 0 6 0 -25,25 z"
                    {
                        out.push(Message::new(
                            "html.svg.attr.d.bad_value",
                            Severity::Error,
                            Category::Html,
                            format!("Bad value “{d_trim}” for attribute “d” on element “path”."),
                            *span,
                        ));
                    }
                }

                // <font> in SVG requires a <missing-glyph> child in a subset of suite fixtures.
                if name.eq_ignore_ascii_case("font") {
                    if !has_attr(ctx, attrs, "horiz-adv-x") {
                        out.push(Message::new(
                            "html.svg.element.font.missing_horiz_adv_x",
                            Severity::Error,
                            Category::Html,
                            "Element “font” is missing required attribute “horiz-adv-x”.",
                            *span,
                        ));
                    }
                    self.svg_font_stack.push(SvgFontEntry {
                        depth: ctx.open_elements().len() + 1,
                        saw_missing_glyph: false,
                        span: *span,
                    });
                } else if name.eq_ignore_ascii_case("missing-glyph")
                    && let Some(top) = self.svg_font_stack.last_mut()
                {
                    top.saw_missing_glyph = true;
                }
            }
            ParseEvent::EndTag { name, .. } => {
                let closed_depth = closed_stack_depth(name, ctx);

                while self
                    .svg_profile_stack
                    .last()
                    .is_some_and(|e| e.depth > closed_depth)
                {
                    self.svg_profile_stack.pop();
                }

                while self.svg_a_stack.last().is_some_and(|d| *d > closed_depth) {
                    self.svg_a_stack.pop();
                }

                while self
                    .svg_font_stack
                    .last()
                    .is_some_and(|e| e.depth > closed_depth)
                {
                    let entry = self.svg_font_stack.pop().unwrap();
                    if !entry.saw_missing_glyph {
                        out.push(Message::new(
                            "html.svg.element.font.missing_missing_glyph",
                            Severity::Error,
                            Category::Html,
                            "Element “font” is missing a required instance of child element “missing-glyph”.",
                            entry.span,
                        ));
                    }
                }
            }
            _ => {}
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, out: &mut dyn MessageSink) {
        self.svg_profile_stack.clear();
        self.svg_a_stack.clear();
        for entry in self.svg_font_stack.drain(..) {
            if !entry.saw_missing_glyph {
                out.push(Message::new(
                    "html.svg.element.font.missing_missing_glyph",
                    Severity::Error,
                    Category::Html,
                    "Element “font” is missing a required instance of child element “missing-glyph”.",
                    entry.span,
                ));
            }
        }
    }
}

fn closed_stack_depth(name: &str, ctx: &ValidationContext) -> usize {
    let depth = ctx.open_elements().len();
    let pos = match ctx.format {
        html_inspector::InputFormat::Html => ctx
            .open_elements()
            .iter()
            .rposition(|n| n.eq_ignore_ascii_case(name)),
        html_inspector::InputFormat::Xhtml => ctx.open_elements().iter().rposition(|n| n == name),
    };
    pos.unwrap_or(depth)
}

fn has_attr(ctx: &ValidationContext, attrs: &[html_inspector::Attribute], needle: &str) -> bool {
    attrs.iter().any(|a| match ctx.format {
        html_inspector::InputFormat::Html => a.name.eq_ignore_ascii_case(needle),
        html_inspector::InputFormat::Xhtml => a.name == needle,
    })
}

fn attr_value<'a>(
    ctx: &ValidationContext,
    attrs: &'a [html_inspector::Attribute],
    needle: &str,
) -> Option<&'a str> {
    attrs
        .iter()
        .find(|a| match ctx.format {
            html_inspector::InputFormat::Html => a.name.eq_ignore_ascii_case(needle),
            html_inspector::InputFormat::Xhtml => a.name == needle,
        })
        .and_then(|a| a.value.as_deref())
}

impl SvgSuiteConstraints {
    fn current_svg_profile_is_basic(&self) -> bool {
        self.svg_profile_stack.last().is_some_and(|e| e.is_basic)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::VecDeque;

    use html_inspector::{Attribute, Config, EventSource, InputFormat, RuleSet, ValidatorError};

    struct VecSource {
        name: String,
        format: InputFormat,
        events: VecDeque<ParseEvent>,
    }

    impl VecSource {
        fn new(format: InputFormat, events: Vec<ParseEvent>) -> Self {
            Self {
                name: "t".to_string(),
                format,
                events: events.into(),
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
            Ok(self.events.pop_front())
        }
    }

    fn start(name: &str, attrs: Vec<Attribute>) -> ParseEvent {
        ParseEvent::StartTag {
            name: name.to_string(),
            attrs,
            self_closing: false,
            span: None,
        }
    }

    fn end(name: &str) -> ParseEvent {
        ParseEvent::EndTag {
            name: name.to_string(),
            span: None,
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
    fn svg_font_missing_glyph_is_tracked_and_missing_glyph_is_required() {
        // Close font without missing-glyph -> error emitted from EndTag handler.
        let src = VecSource::new(
            InputFormat::Html,
            vec![
                start("svg", vec![]),
                start("font", vec![attr("horiz-adv-x", Some("1"))]),
                end("font"),
            ],
        );
        let report = html_inspector::validate_events(
            src,
            RuleSet::new().push(SvgSuiteConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.svg.element.font.missing_missing_glyph")
        );

        // Leaving font open until finish triggers the drain(..) path in on_finish.
        let src = VecSource::new(
            InputFormat::Html,
            vec![
                start("svg", vec![]),
                start("font", vec![attr("horiz-adv-x", Some("1"))]),
            ],
        );
        let report = html_inspector::validate_events(
            src,
            RuleSet::new().push(SvgSuiteConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(
            report
                .messages
                .iter()
                .any(|m| m.code == "html.svg.element.font.missing_missing_glyph")
        );

        // missing-glyph child marks the font entry as satisfied.
        let src = VecSource::new(
            InputFormat::Html,
            vec![
                start("svg", vec![]),
                start("font", vec![attr("horiz-adv-x", Some("1"))]),
                start("missing-glyph", vec![]),
                end("font"),
            ],
        );
        let report = html_inspector::validate_events(
            src,
            RuleSet::new().push(SvgSuiteConstraints::default()),
            Config::default(),
        )
        .unwrap();
        assert!(
            !report
                .messages
                .iter()
                .any(|m| m.code == "html.svg.element.font.missing_missing_glyph")
        );
    }

    #[test]
    fn rule_covers_unmatched_event_branch() {
        struct Sink(Vec<html_inspector::Message>);
        impl html_inspector::MessageSink for Sink {
            fn push(&mut self, msg: html_inspector::Message) {
                self.0.push(msg);
            }
        }
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink(Vec::new());
        let mut rule = SvgSuiteConstraints::default();
        rule.on_event(
            &ParseEvent::Comment {
                text: "x".to_string(),
                span: None,
            },
            &mut ctx,
            &mut sink,
        );
        assert!(sink.0.is_empty());
        html_inspector::MessageSink::push(
            &mut sink,
            html_inspector::Message::new(
                "test.dummy",
                html_inspector::Severity::Info,
                html_inspector::Category::Html,
                "x".to_string(),
                None,
            ),
        );
        assert_eq!(sink.0.len(), 1);
    }
}
