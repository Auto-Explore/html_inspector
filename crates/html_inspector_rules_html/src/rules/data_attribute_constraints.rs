use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct DataAttributeConstraints;

impl Rule for DataAttributeConstraints {
    fn id(&self) -> &'static str {
        "html.data_attribute.constraints"
    }

    fn interest(&self) -> Interest {
        Interest::START_TAG
    }

    fn on_event(
        &mut self,
        event: &ParseEvent,
        _ctx: &mut ValidationContext,
        out: &mut dyn MessageSink,
    ) {
        let ParseEvent::StartTag {
            name, attrs, span, ..
        } = event
        else {
            return;
        };

        for attr in attrs {
            let attr_name = attr.name.as_str();
            if !attr_name.starts_with("data-") {
                continue;
            }

            if attr_name == "data-" {
                out.push(Message::new(
                    "html.data_attribute.empty_suffix",
                    Severity::Error,
                    Category::Html,
                    format!("Attribute “data-” not allowed on element “{name}” at this point."),
                    *span,
                ));
                return;
            }

            let suffix = &attr_name["data-".len()..];
            if suffix.as_bytes().iter().any(|b| b.is_ascii_uppercase()) {
                out.push(Message::new(
                    "html.data_attribute.uppercase",
                    Severity::Error,
                    Category::Html,
                    "“data-*” attributes must not have characters from the range “A”…“Z” in the name.",
                    *span,
                ));
                return;
            }

            if !is_valid_data_suffix(suffix) {
                out.push(Message::new(
                    "html.data_attribute.not_xml_serializable",
                    Severity::Error,
                    Category::Html,
                    "“data-*” attribute names must be XML 1.0 4th ed. plus Namespaces NCNames.",
                    *span,
                ));
                return;
            }
        }
    }
}

fn is_valid_data_suffix(suffix: &str) -> bool {
    // Light NCName-ish check: allow typical HTML data-* characters; reject spaces/control.
    !suffix.is_empty()
        && !suffix.contains(':')
        && suffix
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.')
}

#[cfg(test)]
mod is_valid_data_suffix_tests {
    use super::is_valid_data_suffix;

    #[test]
    fn rejects_empty_suffix() {
        assert!(!is_valid_data_suffix(""));
    }

    #[test]
    fn rejects_colon() {
        assert!(!is_valid_data_suffix("a:b"));
    }

    #[test]
    fn accepts_common_ncname_like_suffixes() {
        for s in ["x", "x1", "x_y", "x-y", "x.y", "a_b-c9."] {
            assert!(is_valid_data_suffix(s), "{s}");
        }
    }

    #[test]
    fn rejects_spaces_controls_and_non_ascii() {
        for s in ["a b", "a\tb", "a\nb", "a©b", "a/b", "a{b"] {
            assert!(!is_valid_data_suffix(s), "{s}");
        }
    }
}
