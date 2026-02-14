use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

use super::foreign_content::{self, Namespace};
use super::shared::normalize_name;

#[derive(Default)]
pub struct UnknownElementConstraints;

#[derive(Default)]
pub struct UnknownSvgElementConstraints;

// Lists are copied from vnu.jar `src/nu/validator/checker/schematronequiv/Assertions.java`:
// HTML_ELEMENTS, SVG_ELEMENTS, MATHML_ELEMENTS.
const HTML_ELEMENTS: &[&str] = &[
    "a",
    "abbr",
    "acronym",
    "address",
    "annotation-xml",
    "applet",
    "area",
    "article",
    "aside",
    "attachment",
    "audio",
    "b",
    "base",
    "basefont",
    "bdi",
    "bdo",
    "bgsound",
    "big",
    "blockquote",
    "body",
    "br",
    "button",
    "canvas",
    "caption",
    "center",
    "cite",
    "code",
    "col",
    "colgroup",
    "color-profile",
    "data",
    "datalist",
    "dd",
    "del",
    "details",
    "dfn",
    "dialog",
    "dir",
    "div",
    "dl",
    "dt",
    "em",
    "embed",
    "fieldset",
    "figcaption",
    "figure",
    "font",
    "font-face",
    "font-face-format",
    "font-face-name",
    "font-face-src",
    "font-face-uri",
    "footer",
    "form",
    "frame",
    "frameset",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "head",
    "header",
    "hgroup",
    "hr",
    "html",
    "i",
    "iframe",
    "image",
    "img",
    "input",
    "ins",
    "kbd",
    "keygen",
    "label",
    "legend",
    "li",
    "link",
    "listing",
    "main",
    "map",
    "mark",
    "marquee",
    "menu",
    "meta",
    "meter",
    "missing-glyph",
    "model",
    "nav",
    "nobr",
    "noembed",
    "noframes",
    "noscript",
    "object",
    "ol",
    "optgroup",
    "option",
    "output",
    "p",
    "param",
    "picture",
    "plaintext",
    "pre",
    "progress",
    "q",
    "rb",
    "rp",
    "rt",
    "rtc",
    "ruby",
    "s",
    "samp",
    "script",
    "search",
    "section",
    "select",
    "selectedcontent",
    "slot",
    "small",
    "source",
    "span",
    "strike",
    "strong",
    "style",
    "sub",
    "summary",
    "sup",
    "table",
    "tbody",
    "td",
    "template",
    "textarea",
    "tfoot",
    "th",
    "thead",
    "time",
    "title",
    "tr",
    "track",
    "tt",
    "u",
    "ul",
    "var",
    "video",
    "wbr",
    "xmp",
];

const MATHML_ELEMENTS: &[&str] = &[
    "annotation",
    "annotation-xml",
    "maction",
    "maligngroup",
    "malignmark",
    "math",
    "menclose",
    "merror",
    "mfenced",
    "mfrac",
    "mglyph",
    "mi",
    "mlabeledtr",
    "mlongdiv",
    "mmultiscripts",
    "mn",
    "mo",
    "mover",
    "mpadded",
    "mphantom",
    "mprescripts",
    "mroot",
    "mrow",
    "ms",
    "mscarries",
    "mscarry",
    "msgroup",
    "msline",
    "mspace",
    "msqrt",
    "msrow",
    "mstack",
    "mstyle",
    "msub",
    "msubsup",
    "msup",
    "mtable",
    "mtd",
    "mtext",
    "mtr",
    "munder",
    "munderover",
    "none",
    "semantics",
];

const SVG_ELEMENTS: &[&str] = &[
    "a",
    "altGlyph",
    "altGlyphDef",
    "altGlyphItem",
    "animate",
    "animateColor",
    "animateMotion",
    "animateTransform",
    "circle",
    "clipPath",
    "color-profile",
    "cursor",
    "defs",
    "desc",
    "ellipse",
    "feBlend",
    "feColorMatrix",
    "feComponentTransfer",
    "feComposite",
    "feConvolveMatrix",
    "feDiffuseLighting",
    "feDisplacementMap",
    "feDistantLight",
    "feDropShadow",
    "feFlood",
    "feFuncA",
    "feFuncB",
    "feFuncG",
    "feFuncR",
    "feGaussianBlur",
    "feImage",
    "feMerge",
    "feMergeNode",
    "feMorphology",
    "feOffset",
    "fePointLight",
    "feSpecularLighting",
    "feSpotLight",
    "feTile",
    "feTurbulence",
    "filter",
    "font",
    "font-face",
    "font-face-format",
    "font-face-name",
    "font-face-src",
    "font-face-uri",
    "foreignObject",
    "g",
    "glyph",
    "glyphRef",
    "hkern",
    "image",
    "line",
    "linearGradient",
    "marker",
    "mask",
    "metadata",
    "missing-glyph",
    "mpath",
    "path",
    "pattern",
    "polygon",
    "polyline",
    "radialGradient",
    "rect",
    "script",
    "set",
    "stop",
    "style",
    "svg",
    "switch",
    "symbol",
    "text",
    "textPath",
    "title",
    "tref",
    "tspan",
    "use",
    "view",
    "vkern",
];

impl Rule for UnknownElementConstraints {
    fn id(&self) -> &'static str {
        "html.unknown_element.constraints"
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
        let ParseEvent::StartTag { name, span, .. } = event else {
            return;
        };

        // Match vnu.jar behavior: schema/Assertions checks are skipped inside <template> contents.
        if ctx.has_ancestor("template") {
            return;
        }

        let ns = foreign_content::namespace_for_next_start_tag(ctx, name);
        let parent = ctx.current_parent().unwrap_or("body");

        let name_norm = normalize_name(ctx, name);
        let parent_norm = normalize_name(ctx, parent);

        let (name_display, parent_display) = match ns {
            Namespace::Svg => (name.as_str(), parent),
            Namespace::Html | Namespace::Math => (name_norm.as_ref(), parent_norm.as_ref()),
        };

        match ns {
            Namespace::Html => {
                // Match vnu.jar behavior: don't treat potential custom elements as "completely-unknown".
                if name_norm.contains('-') || HTML_ELEMENTS.contains(&name_norm.as_ref()) {
                    return;
                }
                report_unknown(ns, name_display, parent_display, *span, out);
            }
            Namespace::Svg => {}
            Namespace::Math => {
                if MATHML_ELEMENTS.contains(&name_norm.as_ref()) {
                    return;
                }
                report_unknown(ns, name_display, parent_display, *span, out);
            }
        }
    }
}

impl Rule for UnknownSvgElementConstraints {
    fn id(&self) -> &'static str {
        "html.unknown_svg_element.constraints"
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
        let ParseEvent::StartTag { name, span, .. } = event else {
            return;
        };

        // Match vnu.jar behavior: schema/Assertions checks are skipped inside <template> contents.
        if ctx.has_ancestor("template") {
            return;
        }

        let ns = foreign_content::namespace_for_next_start_tag(ctx, name);
        if ns != Namespace::Svg {
            return;
        }

        if is_known_svg_element(ctx, name) {
            return;
        }

        let parent = ctx.current_parent().unwrap_or("body");
        report_unknown(ns, name.as_str(), parent, *span, out);
    }
}

fn is_known_svg_element(ctx: &ValidationContext, name: &str) -> bool {
    SVG_ELEMENTS.iter().any(|n| ctx.name_is(name, n))
}

fn report_unknown(
    ns: Namespace,
    name: &str,
    parent: &str,
    span: Option<html_inspector::Span>,
    out: &mut dyn MessageSink,
) {
    let content_kind = match ns {
        Namespace::Html => "HTML",
        Namespace::Svg => "SVG",
        Namespace::Math => "MathML",
    };
    // Match vnu.jar behavior: removed elements like <menuitem> can omit the schema-style
    // "not allowed as child of ..." message depending on context, but still get the
    // completely-unknown diagnostic.
    let suppress_child_not_allowed =
        ns == Namespace::Html && ((name == "menuitem" && parent != "menu") || name == "blink");
    if !suppress_child_not_allowed {
        out.push(Message::new(
            "html.unknown_element.not_allowed",
            Severity::Error,
            Category::Html,
            format!("Element “{name}” not allowed as child of “{parent}” in this context."),
            span,
        ));
    }
    out.push(Message::new(
        "html.unknown_element.completely_unknown",
        Severity::Error,
        Category::Html,
        format!(
            "The “{name}” element is a completely-unknown element that is not allowed anywhere in any {content_kind} content."
        ),
        span,
    ));
}
