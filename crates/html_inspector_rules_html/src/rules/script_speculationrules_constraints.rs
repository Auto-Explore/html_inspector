use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, Span, ValidationContext,
};
use serde_json::{Map, Value};

#[derive(Default)]
pub struct ScriptSpeculationrulesConstraints {
    in_speculationrules: bool,
    buf: String,
    span: Option<Span>,
}

impl Rule for ScriptSpeculationrulesConstraints {
    fn id(&self) -> &'static str {
        "html.script.speculationrules.constraints"
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
                if !is(ctx, name, "script") {
                    return;
                }
                let ty = attr_value(ctx, attrs, "type").unwrap_or("").trim();
                if !ty.eq_ignore_ascii_case("speculationrules") {
                    return;
                }
                // If src is present, ScriptConstraints reports it; avoid adding more errors here.
                if has_attr(ctx, attrs, "src") {
                    return;
                }

                self.in_speculationrules = true;
                self.buf.clear();
                self.span = *span;
            }
            ParseEvent::Text { text, .. } => {
                if self.in_speculationrules {
                    self.buf.push_str(text);
                }
            }
            ParseEvent::EndTag { name, .. } => {
                if !self.in_speculationrules || !is(ctx, name, "script") {
                    return;
                }
                self.in_speculationrules = false;
                self.validate(out);
                self.buf.clear();
                self.span = None;
            }
            _ => {}
        }
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, _out: &mut dyn MessageSink) {
        self.in_speculationrules = false;
        self.buf.clear();
        self.span = None;
    }
}

impl ScriptSpeculationrulesConstraints {
    fn validate(&self, out: &mut dyn MessageSink) {
        let text = self.buf.trim();
        if text.is_empty() {
            self.err_json(out);
            return;
        }
        let Ok(value) = serde_json::from_str::<Value>(text) else {
            self.err_json(out);
            return;
        };

        let Some(obj) = value.as_object() else {
            out.push(Message::new(
                "html.script.speculationrules.json.object",
                Severity::Error,
                Category::Html,
                "A “script” element with a “type” attribute whose value is “speculationrules” must contain a JSON object.",
                self.span,
            ));
            return;
        };

        let has_prefetch = obj.contains_key("prefetch");
        let has_prerender = obj.contains_key("prerender");
        if !has_prefetch && !has_prerender {
            out.push(Message::new(
                "html.script.speculationrules.top_level.missing",
                Severity::Error,
                Category::Html,
                "A “script” element with a “type” attribute whose value is “speculationrules” must contain a JSON object with at least one of the properties “prefetch” or “prerender”.",
                self.span,
            ));
            return;
        }

        if obj.keys().any(|k| k != "prefetch" && k != "prerender") {
            out.push(Message::new(
                "html.script.speculationrules.top_level.properties",
                Severity::Error,
                Category::Html,
                "A “script” element with a “type” attribute whose value is “speculationrules” must contain a JSON object with only “prefetch” and/or “prerender” as properties.",
                self.span,
            ));
            return;
        }

        if let Some(v) = obj.get("prefetch")
            && let Some(msg) = validate_rules_array("prefetch", v) {
                out.push(Message::new(
                    "html.script.speculationrules.prefetch.invalid",
                    Severity::Error,
                    Category::Html,
                    msg,
                    self.span,
                ));
                return;
            }
        if let Some(v) = obj.get("prerender")
            && let Some(msg) = validate_rules_array("prerender", v) {
                out.push(Message::new(
                    "html.script.speculationrules.prerender.invalid",
                    Severity::Error,
                    Category::Html,
                    msg,
                    self.span,
                ));
            }
    }

    fn err_json(&self, out: &mut dyn MessageSink) {
        out.push(Message::new(
            "html.script.speculationrules.json.invalid",
            Severity::Error,
            Category::Html,
            "A “script” element with a “type” attribute whose value is “speculationrules” must have valid JSON content.",
            self.span,
        ));
    }
}

fn validate_rules_array(kind: &str, v: &Value) -> Option<String> {
    let Some(arr) = v.as_array() else {
        return Some(format!(
            "The “{kind}” property within the content of a “script” element with a “type” attribute whose value is “speculationrules” must be a JSON array."
        ));
    };

    for item in arr {
        let Some(rule) = item.as_object() else {
            return Some(format!(
                "Each item in the “{kind}” array within the content of a “script” element with a “type” attribute whose value is “speculationrules” must be a JSON object."
            ));
        };
        if let Some(msg) = validate_rule(kind, rule) {
            return Some(msg);
        }
    }
    None
}

fn validate_rule(kind: &str, rule: &Map<String, Value>) -> Option<String> {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum EffectiveSource {
        List,
        Document,
    }

    for k in rule.keys() {
        if k != "source" && k != "urls" && k != "where" && k != "eagerness" {
            return Some(format!(
                "Each rule in the “{kind}” array must only contain the properties “source”, “urls”, “where”, and “eagerness”."
            ));
        }
    }

    let declared_source = match rule.get("source") {
        None => None,
        Some(v) => {
            let Some(s) = v.as_str() else {
                return Some(
                    "The “source” property in a speculation rule must be a string.".to_string(),
                );
            };
            match s {
                "list" => Some(EffectiveSource::List),
                "document" => Some(EffectiveSource::Document),
                _ => {
                    return Some(
                        "The “source” property in a speculation rule must be either “list” or “document”."
                            .to_string(),
                    );
                }
            }
        }
    };

    let has_urls = rule.contains_key("urls");
    let has_where = rule.contains_key("where");

    let effective_source = match (declared_source, has_urls, has_where) {
        (Some(source), _, _) => source,
        (None, true, false) => EffectiveSource::List,
        (None, false, true) => EffectiveSource::Document,
        (None, _, _) => {
            return Some(
                "A speculation rule must have a “source” property, or a “urls” property (for list rules), or a “where” property (for document rules)."
                    .to_string(),
            );
        }
    };

    if let Some(v) = rule.get("eagerness") {
        let Some(s) = v.as_str() else {
            return Some(
                "The “eagerness” property in a speculation rule must be a string.".to_string(),
            );
        };
        if !matches!(s, "eager" | "moderate" | "conservative") {
            return Some(
                "The “eagerness” property in a speculation rule must be one of “eager”, “moderate”, or “conservative”."
                    .to_string(),
            );
        }
    }

    match effective_source {
        EffectiveSource::List => {
            if has_where {
                return Some("A speculation rule with “source” set to “list” must not have a “where” property.".to_string());
            }
            let Some(urls) = rule.get("urls") else {
                return Some(
                    "A speculation rule with “source” set to “list” must have a “urls” property."
                        .to_string(),
                );
            };
            validate_urls(urls)
        }
        EffectiveSource::Document => {
            if has_urls {
                return Some(
                    "A speculation rule with “source” set to “document” must not have a “urls” property.".to_string(),
                );
            }
            let Some(where_val) = rule.get("where") else {
                return Some(
                    "A speculation rule with “source” set to “document” must have a “where” property.".to_string(),
                );
            };
            validate_where(where_val)
        }
    }
}

fn validate_urls(v: &Value) -> Option<String> {
    let Some(arr) = v.as_array() else {
        return Some("The “urls” property in a speculation rule must be a JSON array.".to_string());
    };
    if arr.is_empty() {
        return Some(
            "The “urls” property in a speculation rule must contain at least one URL.".to_string(),
        );
    }
    for item in arr {
        let Some(s) = item.as_str() else {
            return Some("Each item in the “urls” array must be a string.".to_string());
        };
        if s.is_empty() {
            return Some("Each URL in the “urls” array must be a non-empty string.".to_string());
        }
    }
    None
}

fn validate_where(v: &Value) -> Option<String> {
    let Some(obj) = v.as_object() else {
        return Some(
            "The “where” property in a speculation rule must be a JSON object.".to_string(),
        );
    };
    validate_predicate(obj)
}

fn validate_predicate(obj: &Map<String, Value>) -> Option<String> {
    const KEYS: [&str; 5] = ["and", "or", "not", "href_matches", "selector_matches"];

    let mut kind: Option<&'static str> = None;
    for &k in &KEYS {
        if obj.contains_key(k) {
            if kind.is_some() {
                return Some(
                    "A document rule predicate must have only one of the properties “and”, “or”, “not”, “href_matches”, or “selector_matches”."
                        .to_string(),
                );
            }
            kind = Some(k);
        }
    }

    let Some(kind) = kind else {
        return Some(
            "A document rule predicate must have one of the properties “and”, “or”, “not”, “href_matches”, or “selector_matches”."
                .to_string(),
        );
    };

    match kind {
        "and" | "or" => {
            let Some(items) = obj.get(kind).and_then(|v| v.as_array()) else {
                return Some(match kind {
                    "and" => "The “and” property in a document rule must be a JSON array.",
                    "or" => "The “or” property in a document rule must be a JSON array.",
                    _ => unreachable!("kind is selected from a fixed list"),
                }
                .to_string());
            };

            if items.is_empty() {
                return Some(match kind {
                    "and" => "The “and” property in a document rule must contain at least one item.",
                    "or" => "The “or” property in a document rule must contain at least one item.",
                    _ => unreachable!("kind is selected from a fixed list"),
                }.to_string());
            }

            for item in items {
                let Some(child) = item.as_object() else {
                    return Some(
                        "A document rule predicate must have one of the properties “and”, “or”, “not”, “href_matches”, or “selector_matches”."
                            .to_string(),
                    );
                };
                if let Some(msg) = validate_predicate(child) {
                    return Some(msg);
                }
            }
            None
        }
        "not" => {
            let Some(child) = obj.get("not").and_then(|v| v.as_object()) else {
                return Some(
                    "The “not” property in a document rule must be a JSON object.".to_string(),
                );
            };
            validate_predicate(child)
        }
        "href_matches" | "selector_matches" => match &obj[kind] {
            Value::String(s) if !s.is_empty() => None,
            Value::String(_) => Some(
                match kind {
                    "href_matches" => {
                        "The “href_matches” property in a document rule must be a non-empty string."
                    }
                    "selector_matches" => {
                        "The “selector_matches” property in a document rule must be a non-empty string."
                    }
                    _ => unreachable!("kind is selected from a fixed list"),
                }
                .to_string(),
            ),
            _ => Some(
                match kind {
                    "href_matches" => {
                        "The “href_matches” property in a document rule must be a string."
                    }
                    "selector_matches" => {
                        "The “selector_matches” property in a document rule must be a string."
                    }
                    _ => unreachable!("kind is selected from a fixed list"),
                }
                .to_string(),
            ),
        },
        _ => unreachable!("kind is selected from a fixed list"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use html_inspector_core::{Config, InputFormat, RuleSet};
    use html_inspector_html::SimpleHtmlEventSource;
    use serde_json::json;

    fn validate(html: &str) -> Vec<html_inspector_core::Message> {
        let src = SimpleHtmlEventSource::from_str("t", InputFormat::Html, html);
        html_inspector_core::validate_events(
            src,
            RuleSet::new().push(ScriptSpeculationrulesConstraints::default()),
            Config::default(),
        )
        .unwrap()
        .messages
    }

    #[test]
    fn validate_predicate_accepts_href_and_selector_matches() {
        assert!(validate_predicate(json!({"href_matches": "/a"}).as_object().unwrap()).is_none());
        assert!(
            validate_predicate(json!({"selector_matches": ".x"}).as_object().unwrap()).is_none()
        );
        assert!(validate_predicate(json!({"href_matches": ""}).as_object().unwrap()).is_some());
    }

    #[test]
    fn validate_predicate_requires_exactly_one_known_key() {
        let msg = validate_predicate(json!({"x": 1}).as_object().unwrap()).unwrap();
        assert!(msg.contains("must have one of the properties"));

        let msg = validate_predicate(json!({"and": [], "or": []}).as_object().unwrap()).unwrap();
        assert!(msg.contains("must have only one of the properties"));
    }

    #[test]
    fn validate_predicate_and_or_error_messages_are_stable() {
        let msg = validate_predicate(json!({"and": {}}).as_object().unwrap()).unwrap();
        assert_eq!(
            msg,
            "The “and” property in a document rule must be a JSON array."
        );

        let msg = validate_predicate(json!({"or": []}).as_object().unwrap()).unwrap();
        assert_eq!(
            msg,
            "The “or” property in a document rule must contain at least one item."
        );
    }

    #[test]
    fn validate_predicate_rejects_multiple_known_keys_even_when_one_is_href_matches() {
        let msg = validate_predicate(
            json!({"and": [], "href_matches": "/a"})
                .as_object()
                .unwrap(),
        )
        .unwrap();
        assert!(msg.contains("must have only one of the properties"));
    }

    #[test]
    fn validate_rule_infers_source_from_urls_or_where() {
        let list_rule = json!({"urls": ["/a"]});
        assert!(validate_rule("prefetch", list_rule.as_object().unwrap()).is_none());

        let doc_rule = json!({"where": {"href_matches": "/a"}});
        assert!(validate_rule("prefetch", doc_rule.as_object().unwrap()).is_none());
    }

    #[test]
    fn non_script_tags_and_non_speculationrules_types_are_ignored() {
        assert!(validate("<div></div>").is_empty());
        assert!(validate("<script type=\"text/javascript\"></script>").is_empty());
    }

    #[test]
    fn empty_or_invalid_json_is_reported() {
        let msgs = validate("<script type=\"speculationrules\"></script>");
        assert!(msgs
            .iter()
            .any(|m| m.code == "html.script.speculationrules.json.invalid"));

        let msgs2 = validate("<script type=\"speculationrules\">{</script>");
        assert!(msgs2
            .iter()
            .any(|m| m.code == "html.script.speculationrules.json.invalid"));
    }

    #[test]
    fn top_level_object_rules_are_enforced() {
        let msgs = validate("<script type=\"speculationrules\">{}</script>");
        assert!(msgs
            .iter()
            .any(|m| m.code == "html.script.speculationrules.top_level.missing"));

        let msgs2 =
            validate("<script type=\"speculationrules\">{\"prefetch\":[],\"x\":1}</script>");
        assert!(msgs2
            .iter()
            .any(|m| m.code == "html.script.speculationrules.top_level.properties"));
    }

    #[test]
    fn lists_must_be_arrays() {
        let msgs = validate("<script type=\"speculationrules\">{\"prefetch\":{}}</script>");
        assert!(msgs
            .iter()
            .any(|m| m.code == "html.script.speculationrules.prefetch.invalid"));
    }

    #[test]
    fn source_list_requires_urls_and_validates_urls_shape() {
        let msgs = validate(
            "<script type=\"speculationrules\">{\"prefetch\":[{\"source\":\"list\"}]}</script>",
        );
        assert!(msgs.iter().any(|m| {
            m.code == "html.script.speculationrules.prefetch.invalid"
                && m.message
                    .contains("source” set to “list” must have a “urls” property")
        }));

        let msgs2 = validate(
            "<script type=\"speculationrules\">{\"prefetch\":[{\"source\":\"list\",\"urls\":{}}]}</script>",
        );
        assert!(msgs2.iter().any(|m| {
            m.code == "html.script.speculationrules.prefetch.invalid"
                && m.message
                    .contains("The “urls” property in a speculation rule must be a JSON array")
        }));

        let msgs3 = validate(
            "<script type=\"speculationrules\">{\"prefetch\":[{\"source\":\"list\",\"urls\":[1]}]}</script>",
        );
        assert!(msgs3.iter().any(|m| {
            m.code == "html.script.speculationrules.prefetch.invalid"
                && m.message
                    .contains("Each item in the “urls” array must be a string")
        }));
    }

    #[test]
    fn source_document_requires_where_and_validates_predicate_shape() {
        let msgs = validate(
            "<script type=\"speculationrules\">{\"prefetch\":[{\"source\":\"document\"}]}</script>",
        );
        assert!(msgs.iter().any(|m| {
            m.code == "html.script.speculationrules.prefetch.invalid"
                && m.message
                    .contains("source” set to “document” must have a “where” property")
        }));

        let msgs2 = validate(
            "<script type=\"speculationrules\">{\"prefetch\":[{\"source\":\"document\",\"where\":[]}]}</script>",
        );
        assert!(msgs2.iter().any(|m| {
            m.code == "html.script.speculationrules.prefetch.invalid"
                && m.message
                    .contains("The “where” property in a speculation rule must be a JSON object")
        }));

        let msgs3 = validate(
            "<script type=\"speculationrules\">{\"prefetch\":[{\"source\":\"document\",\"where\":{}}]}</script>",
        );
        assert!(msgs3.iter().any(|m| {
            m.code == "html.script.speculationrules.prefetch.invalid"
                && m.message
                    .contains("A document rule predicate must have one of the properties")
        }));

        let msgs4 = validate(
            "<script type=\"speculationrules\">{\"prefetch\":[{\"source\":\"document\",\"where\":{\"and\":{}}}]}</script>",
        );
        assert!(msgs4.iter().any(|m| {
            m.code == "html.script.speculationrules.prefetch.invalid"
                && m.message
                    .contains("The “and” property in a document rule must be a JSON array")
        }));

        let msgs5 = validate(
            "<script type=\"speculationrules\">{\"prefetch\":[{\"source\":\"document\",\"where\":{\"or\":[]}}]}</script>",
        );
        assert!(msgs5.iter().any(|m| {
            m.code == "html.script.speculationrules.prefetch.invalid"
                && m.message
                    .contains("The “or” property in a document rule must contain at least one item")
        }));

        let msgs6 = validate(
            "<script type=\"speculationrules\">{\"prefetch\":[{\"source\":\"document\",\"where\":{\"not\":[]}}]}</script>",
        );
        assert!(msgs6.iter().any(|m| {
            m.code == "html.script.speculationrules.prefetch.invalid"
                && m.message
                    .contains("The “not” property in a document rule must be a JSON object")
        }));
    }

    #[test]
    fn xhtml_attribute_matching_is_case_sensitive() {
        fn validate_xhtml(xhtml: &str) -> Vec<html_inspector_core::Message> {
            let src = SimpleHtmlEventSource::from_str("t", InputFormat::Xhtml, xhtml);
            html_inspector_core::validate_events(
                src,
                RuleSet::new().push(ScriptSpeculationrulesConstraints::default()),
                Config::default(),
            )
            .unwrap()
            .messages
        }

        let msgs = validate_xhtml("<script TYPE=\"speculationrules\">{}</script>");
        assert!(msgs.is_empty());

        let msgs2 = validate_xhtml("<script type=\"speculationrules\">{}</script>");
        assert!(msgs2
            .iter()
            .any(|m| m.code == "html.script.speculationrules.top_level.missing"));
    }

    #[test]
    fn validate_rules_array_and_predicates_cover_more_error_branches() {
        let msg = validate_rules_array("prefetch", &json!(["x"])).unwrap();
        assert!(msg.contains("must be a JSON object"));

        let msg = validate_rule("prefetch", json!({}).as_object().unwrap()).unwrap();
        assert!(msg.contains("must have a “source” property"));

        let msg = validate_rule(
            "prefetch",
            json!({"urls": [], "where": {}}).as_object().unwrap(),
        )
        .unwrap();
        assert!(msg.contains("must have a “source” property"));

        let msg = validate_rule(
            "prefetch",
            json!({"source": 1, "urls": ["/a"]}).as_object().unwrap(),
        )
        .unwrap();
        assert!(msg.contains("The “source” property in a speculation rule must be a string"));

        let msg = validate_predicate(
            json!({"and":[{"href_matches":"/a"}],"or":[{"href_matches":"/b"}]})
                .as_object()
                .unwrap(),
        )
        .unwrap();
        assert!(msg.contains("must have only one of the properties"));

        let msg = validate_predicate(json!({"or":{}}).as_object().unwrap()).unwrap();
        assert!(msg.contains("The “or” property in a document rule must be a JSON array"));

        let msg = validate_predicate(json!({"or":[1]}).as_object().unwrap()).unwrap();
        assert!(msg.contains("A document rule predicate must have one of the properties"));

        let msg =
            validate_predicate(json!({"not":{"href_matches":""}}).as_object().unwrap()).unwrap();
        assert!(msg.contains("href_matches"));
    }

    #[test]
    fn script_with_src_is_ignored_by_speculationrules_rule() {
        let msgs = validate("<script type=\"speculationrules\" src=\"x\"></script>");
        assert!(msgs.is_empty());
    }

    #[test]
    fn valid_rules_cover_success_paths_in_validate_and_predicates() {
        let msgs = validate(
            "<script type=\"speculationrules\">{\"prefetch\":[{\"urls\":[\"/a\"]}]}</script>",
        );
        assert!(msgs.is_empty());

        let and_ok =
            validate_predicate(json!({"and":[{"href_matches":"/a"}]}).as_object().unwrap());
        assert!(and_ok.is_none());

        let or_ok = validate_predicate(json!({"or":[{"href_matches":"/a"}]}).as_object().unwrap());
        assert!(or_ok.is_none());
    }

    #[test]
    fn valid_both_prefetch_and_prerender_are_allowed() {
        let html = "<script type=\"speculationrules\">{\"prefetch\":[{\"urls\":[\"/a\"]}],\"prerender\":[{\"where\":{\"href_matches\":\"/\"}}]}</script>";
        let msgs = validate(html);
        assert!(msgs.is_empty());
    }

    #[test]
    fn eagerness_accepts_known_values_and_rejects_unknowns() {
        let html = "<script type=\"speculationrules\">{\"prefetch\":[{\"urls\":[\"/a\"],\"eagerness\":\"moderate\"}]}</script>";
        let msgs = validate(html);
        assert!(msgs.is_empty());

        let html = "<script type=\"speculationrules\">{\"prefetch\":[{\"urls\":[\"/a\"],\"eagerness\":\"invalid\"}]}</script>";
        let msgs = validate(html);
        assert!(msgs.iter().any(|m| {
            m.code == "html.script.speculationrules.prefetch.invalid" && m.message.contains("eager")
        }));
    }

    #[test]
    fn empty_urls_array_is_rejected() {
        let html = "<script type=\"speculationrules\">{\"prefetch\":[{\"source\":\"list\",\"urls\":[]}]}</script>";
        let msgs = validate(html);
        assert!(msgs.iter().any(|m| {
            m.code == "html.script.speculationrules.prefetch.invalid"
                && m.message.contains("at least one URL")
        }));
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

fn attr_value<'a>(
    ctx: &ValidationContext,
    attrs: &'a [html_inspector_core::Attribute],
    needle: &str,
) -> Option<&'a str> {
    attrs
        .iter()
        .find(|a| match ctx.format {
            html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case(needle),
            html_inspector_core::InputFormat::Xhtml => a.name == needle,
        })
        .and_then(|a| a.value.as_deref())
}
