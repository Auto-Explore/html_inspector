use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct UncheckedSubtreeWarnings {
    warned_rdf: bool,
    warned_openmath: bool,
    warned_inkscape: bool,
    warned_svg_version: bool,
}

impl Rule for UncheckedSubtreeWarnings {
    fn id(&self) -> &'static str {
        "html.unchecked_subtree.warnings"
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

        if !self.warned_rdf && has_prefix(ctx, name, "rdf") {
            self.warned_rdf = true;
            out.push(Message::new(
                "html.unchecked_subtree.rdf",
                Severity::Warning,
                Category::Html,
                "This validator does not validate RDF. RDF subtrees go unchecked.",
                *span,
            ));
        }

        if !self.warned_openmath && has_prefix(ctx, name, "om") {
            self.warned_openmath = true;
            out.push(Message::new(
                "html.unchecked_subtree.openmath",
                Severity::Warning,
                Category::Html,
                "This validator does not validate OpenMath. OpenMath subtrees go unchecked.",
                *span,
            ));
        }

        if !self.warned_inkscape
            && (has_prefix(ctx, name, "inkscape")
                || has_prefix(ctx, name, "sodipodi")
                || attrs.iter().any(|a| {
                    has_prefix(ctx, &a.name, "inkscape") || has_prefix(ctx, &a.name, "sodipodi")
                }))
        {
            self.warned_inkscape = true;
            out.push(Message::new(
                "html.unchecked_subtree.inkscape",
                Severity::Warning,
                Category::Html,
                "This validator does not validate Inkscape extensions properly. Inkscape-specific errors may go unnoticed.",
                *span,
            ));
        }

        if !self.warned_svg_version
            && is_svg_element(ctx, name)
            && let Some(version) = attr_value(ctx, attrs, "version")
            && (version == "1.0" || version == "1.2")
        {
            self.warned_svg_version = true;
            out.push(Message::new(
                        "html.unchecked_subtree.svg_version",
                        Severity::Warning,
                        Category::Html,
                        "Unsupported SVG version specified. This validator only supports SVG 1.1. The recommended way to suppress this warning is to remove the “version” attribute altogether.",
                        *span,
                    ));
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, _out: &mut dyn MessageSink) {
        *self = Self::default();
    }
}

fn has_prefix(ctx: &ValidationContext, name: &str, prefix: &str) -> bool {
    let Some((p, _)) = name.split_once(':') else {
        return false;
    };
    match ctx.format {
        html_inspector_core::InputFormat::Html => p.eq_ignore_ascii_case(prefix),
        html_inspector_core::InputFormat::Xhtml => p == prefix,
    }
}

fn is_svg_element(ctx: &ValidationContext, name: &str) -> bool {
    if let Some((_, local)) = name.split_once(':') {
        match ctx.format {
            html_inspector_core::InputFormat::Html => local.eq_ignore_ascii_case("svg"),
            html_inspector_core::InputFormat::Xhtml => local == "svg",
        }
    } else {
        ctx.name_is(name, "svg")
    }
}

fn attr_value<'a>(
    ctx: &ValidationContext,
    attrs: &'a [html_inspector_core::Attribute],
    needle: &str,
) -> Option<&'a str> {
    attrs
        .iter()
        .find(|a| ctx.name_is(&a.name, needle))
        .and_then(|a| a.value.as_deref())
}

#[cfg(test)]
mod tests {
    use super::*;

    use html_inspector_core::{Config, InputFormat, RuleSet};
    use html_inspector_html::HtmlEventSource;

    fn codes(html: &str) -> Vec<String> {
        let src = HtmlEventSource::from_str("t", InputFormat::Html, html).unwrap();
        html_inspector_core::validate_events(
            src,
            RuleSet::new().push(UncheckedSubtreeWarnings::default()),
            Config::default(),
        )
        .unwrap()
        .messages
        .into_iter()
        .map(|m| m.code)
        .collect()
    }

    #[test]
    fn rdf_prefix_warns_once() {
        let html = "<rdf:RDF></rdf:RDF><rdf:RDF></rdf:RDF>";
        let codes = codes(html);
        assert_eq!(
            codes
                .iter()
                .filter(|c| c.as_str() == "html.unchecked_subtree.rdf")
                .count(),
            1
        );
    }

    #[test]
    fn openmath_prefix_warns() {
        let html = "<om:OMOBJ></om:OMOBJ>";
        assert!(
            codes(html)
                .iter()
                .any(|c| c == "html.unchecked_subtree.openmath")
        );
    }

    #[test]
    fn inkscape_attribute_warns() {
        let html = "<svg inkscape:foo=\"bar\"></svg>";
        assert!(
            codes(html)
                .iter()
                .any(|c| c == "html.unchecked_subtree.inkscape")
        );
    }

    #[test]
    fn svg_version_1_2_warns() {
        let html = "<svg version=\"1.2\"></svg>";
        assert!(
            codes(html)
                .iter()
                .any(|c| c == "html.unchecked_subtree.svg_version")
        );
    }
}
