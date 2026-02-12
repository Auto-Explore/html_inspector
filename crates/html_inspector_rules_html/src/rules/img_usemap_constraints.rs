use std::collections::HashSet;

use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, Span, ValidationContext,
};

#[derive(Default)]
pub struct ImgUsemapConstraints {
    map_names: HashSet<String>,
    pending_refs: Vec<(String, Option<Span>, String)>,
}

impl Rule for ImgUsemapConstraints {
    fn id(&self) -> &'static str {
        "html.img.usemap_constraints"
    }

    fn interest(&self) -> Interest {
        Interest::START_TAG | Interest::FINISH
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

        if is(ctx, name, "map") {
            if let Some(name) = attr_value(ctx, attrs, "name") {
                if !name.is_empty() {
                    self.map_names.insert(name.to_string());
                }
            }
            return;
        }

        let element = if is(ctx, name, "img") {
            "img"
        } else if is(ctx, name, "object") {
            "object"
        } else {
            return;
        };

        let usemap = attr_value(ctx, attrs, "usemap");
        let Some(usemap) = usemap else { return };

        // hash-name reference, with at least one character after '#'
        if !usemap.starts_with('#') || usemap.len() == 1 {
            out.push(Message::new(
                bad_value_code(element),
                Severity::Error,
                Category::Html,
                format!("Bad value “{usemap}” for attribute “usemap” on element “{element}”."),
                *span,
            ));
            return;
        }

        let reference = usemap.trim_start_matches('#').to_string();
        self.pending_refs
            .push((reference, *span, element.to_string()));
    }

    fn on_finish(&mut self, _ctx: &mut ValidationContext, out: &mut dyn MessageSink) {
        for (reference, span, element) in self.pending_refs.drain(..) {
            if !self.map_names.contains(&reference) {
                out.push(Message::new(
                    missing_map_code(&element),
                    Severity::Error,
                    Category::Html,
                    format!(
                        "The hash-name reference in attribute “usemap” referred to “{reference}”, but there is no “map” element with a “name” attribute with that value."
                    ),
                    span,
                ));
            }
        }
        self.map_names.clear();
    }
}

fn bad_value_code(element: &str) -> &'static str {
    match element {
        "img" => "html.img.usemap.bad_value",
        "object" => "html.object.usemap.bad_value",
        _ => "html.usemap.bad_value",
    }
}

fn missing_map_code(element: &str) -> &'static str {
    match element {
        "img" => "html.img.usemap.missing_map_name",
        "object" => "html.object.usemap.missing_map_name",
        _ => "html.usemap.missing_map_name",
    }
}

fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    match ctx.format {
        html_inspector_core::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector_core::InputFormat::Xhtml => actual == expected,
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    use html_inspector_core::{Config, InputFormat, RuleSet};
    use html_inspector_html::HtmlEventSource;

    fn codes(html: &str) -> Vec<String> {
        let src = HtmlEventSource::from_str("t", InputFormat::Html, html).unwrap();
        html_inspector_core::validate_events(
            src,
            RuleSet::new().push(ImgUsemapConstraints::default()),
            Config::default(),
        )
        .unwrap()
        .messages
        .into_iter()
        .map(|m| m.code)
        .collect()
    }

    #[test]
    fn object_usemap_missing_map_name_is_reported() {
        let html = "<object usemap=\"#nope\"></object>";
        assert!(codes(html)
            .iter()
            .any(|c| c == "html.object.usemap.missing_map_name"));
    }

    #[test]
    fn object_usemap_bad_value_is_reported() {
        let html = "<object usemap=\"nohash\"></object>";
        assert!(codes(html)
            .iter()
            .any(|c| c == "html.object.usemap.bad_value"));
    }
}
