use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, Span, ValidationContext,
};

use super::foreign_content::{namespace_for_next_start_tag, Namespace};
use super::shared::attr_value;

#[derive(Default)]
pub struct MathmlConstraints {
    missing_children_stack: Vec<MissingChildrenState>,
    math_start_stack: Vec<usize>,
    closed_math_ranges: Vec<(usize, usize)>,
}

#[derive(Clone, Debug)]
struct MissingChildrenState {
    name_lc: String,
    required: u8,
    seen: u8,
    span: Option<Span>,
}

impl Rule for MathmlConstraints {
    fn id(&self) -> &'static str {
        "html.mathml.constraints"
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
                let name_lc = name.to_ascii_lowercase();
                let in_closed_math_range = span
                    .as_ref()
                    .map(|s| s.byte_start)
                    .is_some_and(|pos| self.is_inside_closed_math_range(pos));

                if let Some(top) = self.missing_children_stack.last_mut()
                    && ctx
                        .current_parent()
                        .is_some_and(|p| p.eq_ignore_ascii_case(&top.name_lc))
                    {
                        top.seen = top.seen.saturating_add(1);
                    }

                let ns = namespace_for_next_start_tag(ctx, name);

                if name_lc == "math" {
                    validate_math_attributes(ctx, attrs, out, *span);
                    if let Some(s) = span {
                        self.math_start_stack.push(s.byte_start);
                    }
                }

                match ns {
                    Namespace::Math => {
                        // annotation-xml with a XHTML-ish encoding is an HTML integration point.
                        if ctx.has_ancestor("annotation-xml") && name_lc != "annotation-xml" {
                            return;
                        }

                        if !is_mathml_token(&name_lc) {
                            out.push(Message::new(
                                "html.mathml.html_start_tag_in_foreign_namespace",
                                Severity::Error,
                                Category::Html,
                                format!(
                                    "HTML start tag “{name_lc}” in a foreign namespace context."
                                ),
                                *span,
                            ));
                            return;
                        }

                        // Unknown MathML-ish elements: report as not allowed in the current context.
                        if !is_known_mathml_element(&name_lc) {
                            let parent = normalize_parent(ctx).unwrap_or("math");
                            out.push(Message::new(
                                "html.mathml.unknown_element",
                                Severity::Error,
                                Category::Html,
                                format!(
                                    "Element “{name_lc}” not allowed as child of “{parent}” in this context."
                                ),
                                *span,
                            ));
                            return;
                        }

                        // Context-sensitive MathML placement constraints (suite-driven).
                        if name_lc == "annotation"
                            && ctx
                                .current_parent()
                                .is_some_and(|p| p.eq_ignore_ascii_case("math"))
                            && !ctx.has_ancestor("semantics")
                        {
                            out.push(Message::new(
                                "html.mathml.annotation.outside_semantics",
                                Severity::Error,
                                Category::Html,
                                "Element “annotation” not allowed as child of “math” in this context.",
                                *span,
                            ));
                            return;
                        }

                        if name_lc == "mprescripts" && !ctx.has_ancestor("mmultiscripts") {
                            let parent = normalize_parent(ctx).unwrap_or("math");
                            out.push(Message::new(
                                "html.mathml.mprescripts.outside_mmultiscripts",
                                Severity::Error,
                                Category::Html,
                                format!(
                                    "Element “mprescripts” not allowed as child of “{parent}” in this context."
                                ),
                                *span,
                            ));
                            return;
                        }

                        if name_lc == "mtr" {
                            let parent = normalize_parent(ctx).unwrap_or("math");
                            if !parent.eq_ignore_ascii_case("mtable") {
                                out.push(Message::new(
                                    "html.mathml.mtr.parent",
                                    Severity::Error,
                                    Category::Html,
                                    format!(
                                        "Element “mtr” not allowed as child of “{parent}” in this context."
                                    ),
                                    *span,
                                ));
                                return;
                            }
                        }

                        if name_lc == "mtd" {
                            let parent = normalize_parent(ctx).unwrap_or("math");
                            if !parent.eq_ignore_ascii_case("mtr") {
                                out.push(Message::new(
                                    "html.mathml.mtd.parent",
                                    Severity::Error,
                                    Category::Html,
                                    format!(
                                        "Element “mtd” not allowed as child of “{parent}” in this context."
                                    ),
                                    *span,
                                ));
                                return;
                            }
                        }

                        if let Some(required) = required_children(&name_lc) {
                            self.missing_children_stack.push(MissingChildrenState {
                                name_lc,
                                required,
                                seen: 0,
                                span: *span,
                            });
                        }
                    }
                    Namespace::Html => {
                        if in_closed_math_range && !is_mathml_token(&name_lc) {
                            out.push(Message::new(
                                "html.mathml.html_start_tag_in_foreign_namespace",
                                Severity::Error,
                                Category::Html,
                                format!(
                                    "HTML start tag “{name_lc}” in a foreign namespace context."
                                ),
                                *span,
                            ));
                            return;
                        }

                        // MathML elements other than <math> are not allowed in the HTML namespace.
                        if name_lc != "math" && is_known_mathml_element(&name_lc) {
                            let parent = normalize_parent(ctx).unwrap_or("body");
                            out.push(Message::new(
                                "html.mathml.element.outside_math",
                                Severity::Error,
                                Category::Html,
                                format!(
                                    "Element “{name_lc}” not allowed as child of “{parent}” in this context."
                                ),
                                *span,
                            ));
                        }
                    }
                    _ => {}
                }
            }
            ParseEvent::EndTag { name, span } => {
                let name_lc = name.to_ascii_lowercase();
                if name_lc == "math" {
                    // Pair up using source locations (useful for cases where the tree builder
                    // implicitly closes <math> before encountering its explicit end tag).
                    if let Some((start, end)) = self
                        .math_start_stack
                        .pop()
                        .zip(span.as_ref().map(|s| s.byte_start))
                        && start < end {
                            self.closed_math_ranges.push((start, end));
                        }
                }
                if let Some(top) = self.missing_children_stack.last()
                    && top.name_lc.eq_ignore_ascii_case(&name_lc) {
                        let top = self.missing_children_stack.pop().expect("just checked");
                        if top.seen < top.required {
                            out.push(Message::new(
                                "html.mathml.missing_children",
                                Severity::Error,
                                Category::Html,
                                format!(
                                    "Element “{}” is missing a required instance of one or more of the following child elements: {}",
                                    top.name_lc, MATHML_MISSING_CHILD_ELEMENTS
                                ),
                                top.span,
                            ));
                        }
                    }
            }
            _ => {}
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, _out: &mut dyn MessageSink) {
        self.missing_children_stack.clear();
        self.math_start_stack.clear();
        self.closed_math_ranges.clear();
    }
}

impl MathmlConstraints {
    fn is_inside_closed_math_range(&self, pos: usize) -> bool {
        self.closed_math_ranges
            .iter()
            .any(|&(start, end)| start < pos && pos < end)
    }
}

fn normalize_parent(ctx: &ValidationContext) -> Option<&str> {
    ctx.current_parent()
}

fn required_children(name_lc: &str) -> Option<u8> {
    match name_lc {
        "mfrac" | "mover" | "mroot" | "msub" | "msup" | "munder" => Some(2),
        "msubsup" | "munderover" => Some(3),
        _ => None,
    }
}

fn validate_math_attributes(
    ctx: &ValidationContext,
    attrs: &[html_inspector_core::Attribute],
    out: &mut dyn MessageSink,
    span: Option<Span>,
) {
    let display = attr_value(ctx, attrs, "display");
    if let Some(v) = display {
        let v_trim = v.trim();
        let v_lc = v_trim.to_ascii_lowercase();
        if v_lc != "block" && v_lc != "inline" {
            out.push(Message::new(
                "html.mathml.math.display.bad_value",
                Severity::Error,
                Category::Html,
                format!("Bad value “{v_trim}” for attribute “display” on element “math”."),
                span,
            ));
        }
    }

    let overflow = attr_value(ctx, attrs, "overflow");
    if let Some(v) = overflow {
        let v_trim = v.trim();
        if !v_trim.eq_ignore_ascii_case("scroll") {
            out.push(Message::new(
                "html.mathml.math.overflow.bad_value",
                Severity::Error,
                Category::Html,
                format!("Bad value “{v_trim}” for attribute “overflow” on element “math”."),
                span,
            ));
        }
    }
}

fn is_mathml_token(name_lc: &str) -> bool {
    name_lc.starts_with('m')
        || matches!(name_lc, "math" | "annotation" | "annotation-xml")
        || is_known_mathml_element(name_lc)
}

fn is_known_mathml_element(name_lc: &str) -> bool {
    MATHML_KNOWN_ELEMENTS.contains(&name_lc)
        || matches!(
            name_lc,
            "math" | "annotation" | "annotation-xml" | "mtr" | "mtd" | "mprescripts"
        )
}

const MATHML_MISSING_CHILD_ELEMENTS: &str = "“abs”, “and”, “apply”, “approx”, “arccos”, “arccosh”, “arccot”, “arccoth”, “arccsc”, “arccsch”, “arcsec”, “arcsech”, “arcsin”, “arcsinh”, “arctan”, “arctanh”, “arg”, “bind”, “card”, “cartesianproduct”, “cbytes”, “ceiling”, “cerror”, “ci”, “cn”, “codomain”, “complexes”, “compose”, “conjugate”, “cos”, “cosh”, “cot”, “coth”, “cs”, “csc”, “csch”, “csymbol”, “curl”, “declare”, “determinant”, “diff”, “divergence”, “divide”, “domain”, “emptyset”, “eq”, “equivalent”, “eulergamma”, “exists”, “exp”, “exponentiale”, “factorial”, “factorof”, “false”, “floor”, “fn”, “forall”, “gcd”, “geq”, “grad”, “gt”, “ident”, “image”, “imaginary”, “imaginaryi”, “implies”, “in”, “infinity”, “int”, “integers”, “intersect”, “interval”, “inverse”, “lambda”, “laplacian”, “lcm”, “leq”, “limit”, “list”, “ln”, “log”, “lt”, “maction”, “maligngroup”, “malignmark”, “matrix”, “matrixrow”, “max”, “mean”, “median”, “menclose”, “merror”, “mfenced”, “mfrac”, “mi”, “min”, “minus”, “mlongdiv”, “mmultiscripts”, “mn”, “mo”, “mode”, “moment”, “mover”, “mpadded”, “mphantom”, “mroot”, “mrow”, “ms”, “mspace”, “msqrt”, “mstack”, “mstyle”, “msub”, “msubsup”, “msup”, “mtable”, “mtext”, “munder”, “munderover”, “naturalnumbers”, “neq”, “not”, “notanumber”, “notin”, “notprsubset”, “notsubset”, “or”, “outerproduct”, “partialdiff”, “pi”, “piecewise”, “plus”, “power”, “primes”, “product”, “prsubset”, “quotient”, “rationals”, “real”, “reals”, “reln”, “rem”, “root”, “scalarproduct”, “sdev”, “sec”, “sech”, “selector”, “semantics”, “set”, “setdiff”, “share”, “sin”, “sinh”, “subset”, “sum”, “tan”, “tanh”, “tendsto”, “times”, “transpose”, “true”, “union”, “variance”, “vector”, “vectorproduct”, “xor”.";

const MATHML_KNOWN_ELEMENTS: [&str; 168] = [
    "abs",
    "and",
    "apply",
    "approx",
    "arccos",
    "arccosh",
    "arccot",
    "arccoth",
    "arccsc",
    "arccsch",
    "arcsec",
    "arcsech",
    "arcsin",
    "arcsinh",
    "arctan",
    "arctanh",
    "arg",
    "bind",
    "card",
    "cartesianproduct",
    "cbytes",
    "ceiling",
    "cerror",
    "ci",
    "cn",
    "codomain",
    "complexes",
    "compose",
    "conjugate",
    "cos",
    "cosh",
    "cot",
    "coth",
    "cs",
    "csc",
    "csch",
    "csymbol",
    "curl",
    "declare",
    "determinant",
    "diff",
    "divergence",
    "divide",
    "domain",
    "emptyset",
    "eq",
    "equivalent",
    "eulergamma",
    "exists",
    "exp",
    "exponentiale",
    "factorial",
    "factorof",
    "false",
    "floor",
    "fn",
    "forall",
    "gcd",
    "geq",
    "grad",
    "gt",
    "ident",
    "image",
    "imaginary",
    "imaginaryi",
    "implies",
    "in",
    "infinity",
    "int",
    "integers",
    "intersect",
    "interval",
    "inverse",
    "lambda",
    "laplacian",
    "lcm",
    "leq",
    "limit",
    "list",
    "ln",
    "log",
    "lt",
    "maction",
    "maligngroup",
    "malignmark",
    "matrix",
    "matrixrow",
    "max",
    "mean",
    "median",
    "menclose",
    "merror",
    "mfenced",
    "mfrac",
    "mi",
    "min",
    "minus",
    "mlongdiv",
    "mmultiscripts",
    "mn",
    "mo",
    "mode",
    "moment",
    "mover",
    "mpadded",
    "mphantom",
    "mroot",
    "mrow",
    "ms",
    "mspace",
    "msqrt",
    "mstack",
    "mstyle",
    "msub",
    "msubsup",
    "msup",
    "mtable",
    "mtext",
    "munder",
    "munderover",
    "naturalnumbers",
    "neq",
    "not",
    "notanumber",
    "notin",
    "notprsubset",
    "notsubset",
    "or",
    "outerproduct",
    "partialdiff",
    "pi",
    "piecewise",
    "plus",
    "power",
    "primes",
    "product",
    "prsubset",
    "quotient",
    "rationals",
    "real",
    "reals",
    "reln",
    "rem",
    "root",
    "scalarproduct",
    "sdev",
    "sec",
    "sech",
    "selector",
    "semantics",
    "set",
    "setdiff",
    "share",
    "sin",
    "sinh",
    "subset",
    "sum",
    "tan",
    "tanh",
    "tendsto",
    "times",
    "transpose",
    "true",
    "union",
    "variance",
    "vector",
    "vectorproduct",
    "xor",
];
