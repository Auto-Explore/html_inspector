use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, Span, ValidationContext,
};
use serde_json::Value;

use super::a_href_constraints::href_issue_severity;

#[derive(Default)]
pub struct ScriptImportmapConstraints {
    in_importmap: bool,
    buf: String,
    span: Option<Span>,
}

impl Rule for ScriptImportmapConstraints {
    fn id(&self) -> &'static str {
        "html.script.importmap.constraints"
    }

    fn interest(&self) -> Interest {
        Interest::START_TAG | Interest::END_TAG | Interest::TEXT
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
                if !ctx.name_is(name, "script") {
                    return;
                }
                let ty = ctx.attr_value(attrs, "type").unwrap_or("").trim();
                if !ty.eq_ignore_ascii_case("importmap") {
                    return;
                }

                // If src is present, ScriptConstraints already flags it; avoid adding more errors.
                if ctx.has_attr(attrs, "src") {
                    return;
                }

                self.in_importmap = true;
                self.buf.clear();
                self.span = *span;
            }
            ParseEvent::Text { text, .. } => {
                if self.in_importmap {
                    self.buf.push_str(text);
                }
            }
            ParseEvent::EndTag { name, .. } => {
                if !self.in_importmap || !ctx.name_is(name, "script") {
                    return;
                }
                self.in_importmap = false;
                self.validate(out);
                self.buf.clear();
                self.span = None;
            }
            _ => {}
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, _out: &mut dyn MessageSink) {
        self.in_importmap = false;
        self.buf.clear();
        self.span = None;
    }
}

impl ScriptImportmapConstraints {
    fn validate(&self, out: &mut dyn MessageSink) {
        let text = self.buf.trim();
        if text.is_empty() {
            out.push(Message::new(
                "html.script.importmap.json.invalid",
                Severity::Error,
                Category::Html,
                "A script “script” with a “type” attribute whose value is “importmap” must have valid JSON content.",
                self.span,
            ));
            return;
        }

        let Ok(value) = serde_json::from_str::<Value>(text) else {
            out.push(Message::new(
                "html.script.importmap.json.invalid",
                Severity::Error,
                Category::Html,
                "A script “script” with a “type” attribute whose value is “importmap” must have valid JSON content.",
                self.span,
            ));
            return;
        };

        let Some(obj) = value.as_object() else {
            out.push(Message::new(
                "html.script.importmap.top_level.object",
                Severity::Error,
                Category::Html,
                "A “script” element with a “type” attribute whose value is “importmap” must contain a JSON object with no properties other than “imports”, “scopes”, and “integrity”.",
                self.span,
            ));
            return;
        };

        for k in obj.keys() {
            if k != "imports" && k != "scopes" && k != "integrity" {
                out.push(Message::new(
                    "html.script.importmap.top_level.properties",
                    Severity::Error,
                    Category::Html,
                    "A “script” element with a “type” attribute whose value is “importmap” must contain a JSON object with no properties other than “imports”, “scopes”, and “integrity”.",
                    self.span,
                ));
                return;
            }
        }

        if let Some(imports) = obj.get("imports") {
            let Some(map) = imports.as_object() else {
                out.push(Message::new(
                    "html.script.importmap.imports.object",
                    Severity::Error,
                    Category::Html,
                    "The value of the “imports” property within the content of a “script” element with a “type” attribute whose value is “importmap” must be a JSON object.",
                    self.span,
                ));
                return;
            };

            for (k, v) in map {
                if k.is_empty() {
                    out.push(Message::new(
                        "html.script.importmap.imports.keys.non_empty",
                        Severity::Error,
                        Category::Html,
                        "A specifier map defined in a “imports” property within the content of a “script” element with a “type” attribute whose value is “importmap” must only contain non-empty keys.",
                        self.span,
                    ));
                    return;
                }
                let Some(vs) = v.as_str() else {
                    out.push(Message::new(
                        "html.script.importmap.imports.values.string",
                        Severity::Error,
                        Category::Html,
                        "A specifier map defined in a “imports” property within the content of a “script” element with a “type” attribute whose value is “importmap” must only contain string values.",
                        self.span,
                    ));
                    return;
                };
                if k.ends_with('/') && !vs.ends_with('/') {
                    out.push(Message::new(
                        "html.script.importmap.imports.slash_match",
                        Severity::Error,
                        Category::Html,
                        "A specifier map defined in a “imports” property within the content of a “script” element with a “type” attribute whose value is “importmap” must have values that end with “/” when its corresponding key ends with “/”.",
                        self.span,
                    ));
                    return;
                }
            }
        }

        if let Some(scopes) = obj.get("scopes") {
            let Some(scopes_obj) = scopes.as_object() else {
                out.push(Message::new(
                    "html.script.importmap.scopes.values.object",
                    Severity::Error,
                    Category::Html,
                    "The value of the “scopes” property within the content of a “script” element with a “type” attribute whose value is “importmap” must be a JSON object whose values are also JSON objects.",
                    self.span,
                ));
                return;
            };

            for (scope_key, scope_val) in scopes_obj {
                if !is_url_like_specifier(scope_key) || href_issue_severity(scope_key).is_some() {
                    out.push(Message::new(
                        "html.script.importmap.scopes.keys.url",
                        Severity::Error,
                        Category::Html,
                        "The value of the “scopes” property within the content of a “script” element with a “type” attribute whose value is “importmap” must be a JSON object whose keys are valid URL strings.",
                        self.span,
                    ));
                    return;
                }

                let Some(spec_map) = scope_val.as_object() else {
                    out.push(Message::new(
                        "html.script.importmap.scopes.values.object",
                        Severity::Error,
                        Category::Html,
                        "The value of the “scopes” property within the content of a “script” element with a “type” attribute whose value is “importmap” must be a JSON object whose values are also JSON objects.",
                        self.span,
                    ));
                    return;
                };

                for (_k, v) in spec_map {
                    let Some(vs) = v.as_str() else {
                        out.push(Message::new(
                            "html.script.importmap.scopes.values.url",
                            Severity::Error,
                            Category::Html,
                            "A specifier map defined in a “scopes” property within the content of a “script” element with a “type” attribute whose value is “importmap” must only contain valid URL values.",
                            self.span,
                        ));
                        return;
                    };
                    if !is_url_like_specifier(vs) || href_issue_severity(vs).is_some() {
                        out.push(Message::new(
                            "html.script.importmap.scopes.values.url",
                            Severity::Error,
                            Category::Html,
                            "A specifier map defined in a “scopes” property within the content of a “script” element with a “type” attribute whose value is “importmap” must only contain valid URL values.",
                            self.span,
                        ));
                        return;
                    }
                }
            }
        }
    }
}

fn is_url_like_specifier(s: &str) -> bool {
    let s = s.trim();
    if s.is_empty() {
        return false;
    }
    if s.starts_with('/') || s.starts_with("./") || s.starts_with("../") {
        return true;
    }
    // absolute URL (scheme:)
    let mut chars = s.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !first.is_ascii_alphabetic() {
        return false;
    }
    let mut i = 1usize;
    for c in chars {
        if c == ':' {
            return i > 0;
        }
        if c.is_ascii_alphanumeric() || c == '+' || c == '-' || c == '.' {
            i += 1;
            continue;
        }
        return false;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    use html_inspector_core::{Config, InputFormat, RuleSet};
    use html_inspector_html::SimpleHtmlEventSource;

    fn validate(html: &str) -> Vec<html_inspector_core::Message> {
        let src = SimpleHtmlEventSource::from_str("t", InputFormat::Html, html);
        html_inspector_core::validate_events(
            src,
            RuleSet::new().push(ScriptImportmapConstraints::default()),
            Config::default(),
        )
        .unwrap()
        .messages
    }

    #[test]
    fn url_like_specifier_requires_path_like_or_scheme() {
        assert!(is_url_like_specifier("/a"));
        assert!(is_url_like_specifier("./a"));
        assert!(is_url_like_specifier("../a"));
        assert!(is_url_like_specifier("http://example.com/"));
        assert!(!is_url_like_specifier(""));
        assert!(!is_url_like_specifier("   "));
        assert!(!is_url_like_specifier("1:foo"));
        assert!(!is_url_like_specifier("abc"));
    }

    #[test]
    fn non_script_tags_and_non_importmap_types_are_ignored() {
        assert!(validate("<div></div>").is_empty());
        assert!(validate("<script type=\"text/javascript\"></script>").is_empty());
    }

    #[test]
    fn empty_importmap_content_is_invalid_json() {
        let msgs = validate("<script type=\"importmap\"></script>");
        assert!(
            msgs.iter()
                .any(|m| m.code == "html.script.importmap.json.invalid")
        );
    }

    #[test]
    fn empty_importmap_content_is_invalid_json_with_uppercase_tag_and_type() {
        let msgs = validate("<SCRIPT TYPE=\"importmap\"></SCRIPT>");
        assert!(
            msgs.iter()
                .any(|m| m.code == "html.script.importmap.json.invalid")
        );
    }

    #[test]
    fn importmap_with_src_is_ignored_even_if_attr_name_case_differs() {
        assert!(validate("<script type=\"importmap\" SrC=\"x\"></script>").is_empty());
    }

    #[test]
    fn imports_slash_match_is_enforced() {
        let msgs = validate(
            "<script type=\"importmap\">{\"imports\":{\"a/\":\"https://example.com/a\"}}</script>",
        );
        assert!(
            msgs.iter()
                .any(|m| m.code == "html.script.importmap.imports.slash_match")
        );
    }

    #[test]
    fn scopes_values_must_be_valid_urls() {
        let msgs =
            validate("<script type=\"importmap\">{\"scopes\":{\"./\":{\"a\":\"1\"}}}</script>");
        assert!(
            msgs.iter()
                .any(|m| m.code == "html.script.importmap.scopes.values.url")
        );
    }
}
