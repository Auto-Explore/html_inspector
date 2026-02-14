use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

use super::shared::normalize_name;

#[derive(Default)]
pub struct UnnecessaryRoleWarnings;

impl Rule for UnnecessaryRoleWarnings {
    fn id(&self) -> &'static str {
        "html.role.unnecessary_warnings"
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

        let role = ctx.attr_value(attrs, "role");
        let Some(role) = role else { return };
        let role_trim = role.trim();
        if role_trim.is_empty() {
            return;
        }
        let role_lc = role_trim.to_ascii_lowercase();

        // role=directory (deprecated/removed role) is treated as invalid by vnu, but only as a warning.
        if role_lc == "directory" {
            out.push(Message::new(
                "html.role.directory.bad_value",
                Severity::Warning,
                Category::Html,
                format!(
                    "Bad value “directory” for attribute “role” on element “{}”.",
                    normalize_name(ctx, name)
                ),
                *span,
            ));
            return;
        }

        let el = normalize_name(ctx, name);
        let is_unnecessary = match el.as_ref() {
            "a" => role_lc == "link" && ctx.attr_value(attrs, "href").is_some(),
            "article" => role_lc == "article",
            "aside" => role_lc == "complementary",
            "button" => role_lc == "button",
            "footer" => role_lc == "contentinfo" && !in_sectioning_context(ctx),
            "header" => role_lc == "banner" && !in_sectioning_context(ctx),
            "dd" => role_lc == "definition",
            "dt" => role_lc == "term",
            "details" => role_lc == "group",
            "dialog" => role_lc == "dialog",
            "ul" => role_lc == "list",
            "main" => role_lc == "main",
            "nav" => role_lc == "navigation",
            "progress" => role_lc == "progressbar",
            "hr" => role_lc == "separator",
            "output" => role_lc == "status",
            "summary" => role_lc == "button",
            "table" => role_lc == "table",
            "figure" => role_lc == "figure",
            "tbody" => role_lc == "rowgroup",
            "form" => role_lc == "form" && has_accessible_name(attrs, ctx),
            "section" => role_lc == "region" && has_accessible_name(attrs, ctx),
            "s" => role_lc == "deletion",
            "img" => {
                role_lc == "img" && ctx.attr_value(attrs, "alt").is_some_and(|v| !v.is_empty())
            }
            "select" => {
                if role_lc != "listbox" {
                    false
                } else {
                    let has_multiple = ctx.has_attr(attrs, "multiple");
                    let size_is_gt_one = ctx
                        .attr_value(attrs, "size")
                        .and_then(|s| s.trim().parse::<u32>().ok())
                        .is_some_and(|n| n > 1);
                    has_multiple || size_is_gt_one
                }
            }
            "input" => {
                let ty = ctx.attr_value(attrs, "type").unwrap_or("text");
                let has_list = ctx.has_attr(attrs, "list");
                (role_lc == "spinbutton" && ty.eq_ignore_ascii_case("number"))
                    || (role_lc == "searchbox" && !has_list && ty.eq_ignore_ascii_case("search"))
                    || (role_lc == "textbox" && !has_list && ty.eq_ignore_ascii_case("text"))
            }
            _ => false,
        };

        if !is_unnecessary {
            return;
        }

        let msg = if el == "input" && role_lc == "spinbutton" {
            "The “spinbutton” role is unnecessary for element “input” whose type is “number”."
                .to_string()
        } else if el == "input" && role_lc == "searchbox" {
            "The “searchbox” role is unnecessary for an “input” element that has no “list” attribute and whose type is “search”.".to_string()
        } else if el == "input" && role_lc == "textbox" {
            "The “textbox” role is unnecessary for an “input” element that has no “list” attribute and whose type is “text”.".to_string()
        } else if el == "a" && role_lc == "link" {
            "The “link” role is unnecessary for element “a” with attribute “href”.".to_string()
        } else if el == "select" && role_lc == "listbox" {
            "The “listbox” role is unnecessary for element “select” with a “multiple” attribute or with a “size” attribute whose value is greater than 1.".to_string()
        } else {
            format!("The “{role_trim}” role is unnecessary for element “{el}”.")
        };

        out.push(Message::new(
            "html.role.unnecessary",
            Severity::Warning,
            Category::Html,
            msg,
            *span,
        ));
    }
}

fn in_sectioning_context(ctx: &ValidationContext) -> bool {
    // HTML AAM: header/footer implicit landmark roles are suppressed within sectioning contexts.
    ctx.has_ancestor("article")
        || ctx.has_ancestor("aside")
        || ctx.has_ancestor("main")
        || ctx.has_ancestor("nav")
        || ctx.has_ancestor("section")
}

fn has_accessible_name(attrs: &[html_inspector_core::Attribute], ctx: &ValidationContext) -> bool {
    ctx.attr_value(attrs, "aria-label")
        .is_some_and(|v| !v.trim().is_empty())
        || ctx
            .attr_value(attrs, "aria-labelledby")
            .is_some_and(|v| !v.trim().is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    use html_inspector_core::{Config, InputFormat, RuleSet};
    use html_inspector_html::HtmlEventSource;

    fn validate_fmt(format: InputFormat, html: &str) -> Vec<html_inspector_core::Message> {
        let src = HtmlEventSource::from_str("t", format, html).unwrap();
        html_inspector_core::validate_events(
            src,
            RuleSet::new().push(UnnecessaryRoleWarnings::default()),
            Config::default(),
        )
        .unwrap()
        .messages
    }

    #[test]
    fn directory_role_is_warned() {
        let msgs = validate_fmt(InputFormat::Html, "<div role=\"directory\"></div>");
        assert!(
            msgs.iter()
                .any(|m| m.code == "html.role.directory.bad_value")
        );
    }

    #[test]
    fn summary_button_role_is_unnecessary() {
        let msgs = validate_fmt(InputFormat::Html, "<summary role=\"button\"></summary>");
        assert!(msgs.iter().any(|m| m.code == "html.role.unnecessary"));
        assert!(msgs.iter().any(|m| {
            m.code == "html.role.unnecessary"
                && m.severity == html_inspector_core::Severity::Warning
                && m.message == "The “button” role is unnecessary for element “summary”."
        }));
    }

    #[test]
    fn unnecessary_role_warnings_are_skipped_inside_template() {
        let msgs = validate_fmt(
            InputFormat::Html,
            "<template><summary role=\"button\"></summary></template>",
        );
        assert!(!msgs.iter().any(|m| m.code == "html.role.unnecessary"));
    }

    #[test]
    fn sectioning_context_suppresses_header_footer_unnecessary_role_warnings() {
        let msgs = validate_fmt(
            InputFormat::Html,
            "<article><header role=\"banner\"></header></article>",
        );
        assert!(!msgs.iter().any(|m| m.code == "html.role.unnecessary"));
    }

    #[test]
    fn accessible_name_enables_form_and_section_unnecessary_role_warnings() {
        let msgs = validate_fmt(
            InputFormat::Html,
            "<form role=\"form\" aria-label=\"x\"></form>",
        );
        assert!(msgs.iter().any(|m| m.code == "html.role.unnecessary"));

        let msgs2 = validate_fmt(
            InputFormat::Html,
            "<section role=\"region\" aria-labelledby=\"x\"></section>",
        );
        assert!(msgs2.iter().any(|m| m.code == "html.role.unnecessary"));
    }

    #[test]
    fn select_listbox_is_unnecessary_when_multiple_or_size_gt_one() {
        let msgs = validate_fmt(
            InputFormat::Html,
            "<select role=\"listbox\" multiple></select>",
        );
        assert!(msgs.iter().any(|m| m.code == "html.role.unnecessary"));

        let msgs2 = validate_fmt(
            InputFormat::Html,
            "<select role=\"listbox\" size=\"2\"></select>",
        );
        assert!(msgs2.iter().any(|m| m.code == "html.role.unnecessary"));
    }

    #[test]
    fn xhtml_name_normalization_and_attribute_matching_are_case_sensitive() {
        let msgs = validate_fmt(InputFormat::Xhtml, "<DIV role=\"directory\"></DIV>");
        let msg = msgs
            .iter()
            .find(|m| m.code == "html.role.directory.bad_value")
            .expect("expected directory warning");
        assert!(msg.message.contains("DIV"));

        let msgs2 = validate_fmt(
            InputFormat::Xhtml,
            "<select role=\"listbox\" multiple></select>",
        );
        assert!(msgs2.iter().any(|m| m.code == "html.role.unnecessary"));
    }

    #[test]
    fn xhtml_does_not_treat_uppercase_element_names_as_html_equivalents() {
        let msgs = validate_fmt(InputFormat::Xhtml, "<A role=\"link\" href=\"/\">x</A>");
        assert!(!msgs.iter().any(|m| m.code == "html.role.unnecessary"));

        let msgs2 = validate_fmt(InputFormat::Xhtml, "<a role=\"link\" href=\"/\">x</a>");
        assert!(msgs2.iter().any(|m| m.code == "html.role.unnecessary"));
    }
}
