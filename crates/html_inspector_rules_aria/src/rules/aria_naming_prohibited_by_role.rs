use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

use super::shared::tag_name_for_message;

#[derive(Default)]
pub struct AriaNamingProhibitedByRole;

impl Rule for AriaNamingProhibitedByRole {
    fn id(&self) -> &'static str {
        "aria.naming.prohibited_by_role"
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

        let has_aria_label = ctx.has_attr(attrs, "aria-label");
        let has_aria_labelledby = ctx.has_attr(attrs, "aria-labelledby");
        let has_aria_braillelabel = ctx.has_attr(attrs, "aria-braillelabel");
        if !(has_aria_label || has_aria_labelledby || has_aria_braillelabel) {
            return;
        }

        let el = tag_name_for_message(ctx, name);
        let role = computed_role(ctx, el.as_ref(), attrs);
        if !is_name_prohibited_role(&role) {
            return;
        }

        for attr in [
            (has_aria_label, "aria-label"),
            (has_aria_labelledby, "aria-labelledby"),
            (has_aria_braillelabel, "aria-braillelabel"),
        ] {
            if !attr.0 {
                continue;
            }
            out.push(Message::new(
                format!("aria.{}.prohibited_on_role.{}", attr.1.replace('-', "_"), role),
                Severity::Error,
                Category::Aria,
                format!(
                    "The “{}” attribute must not be specified on any “{}” element unless the element has a “role” value other than {}",
                    attr.1,
                    el,
                    NAME_PROHIBITED_ROLE_LIST
                ),
                *span,
            ));
        }
    }
}

const NAME_PROHIBITED_ROLE_LIST: &str = "“caption”, “code”, “deletion”, “emphasis”, “generic”, “insertion”, “paragraph”, “presentation”, “strong”, “subscript”, or “superscript”.";

fn is_name_prohibited_role(role_lc: &str) -> bool {
    matches!(
        role_lc,
        "caption"
            | "code"
            | "deletion"
            | "emphasis"
            | "generic"
            | "insertion"
            | "paragraph"
            | "presentation"
            | "strong"
            | "subscript"
            | "superscript"
    )
}

fn computed_role(
    ctx: &ValidationContext,
    el_lc: &str,
    attrs: &[html_inspector::Attribute],
) -> String {
    if let Some(role) = ctx.attr_value(attrs, "role")
        && let Some(t) = role.split_ascii_whitespace().next()
    {
        return t.to_ascii_lowercase();
    }

    match el_lc {
        "a" | "area" => {
            if ctx.has_attr(attrs, "href") {
                "link".to_string()
            } else {
                "generic".to_string()
            }
        }
        "button" => "button".to_string(),
        "caption" => "caption".to_string(),
        "code" => "code".to_string(),
        "del" => "deletion".to_string(),
        "details" => "group".to_string(),
        "dialog" => "dialog".to_string(),
        "em" => "emphasis".to_string(),
        "figure" => "figure".to_string(),
        "footer" => "contentinfo".to_string(),
        "form" => {
            if has_accessible_name(ctx, attrs) {
                "form".to_string()
            } else {
                "generic".to_string()
            }
        }
        "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => "heading".to_string(),
        "header" => "banner".to_string(),
        "img" => "img".to_string(),
        "input" => implicit_input_role(ctx, attrs),
        "ins" => "insertion".to_string(),
        "li" => "listitem".to_string(),
        "label" => "label".to_string(),
        "main" => "main".to_string(),
        "nav" => "navigation".to_string(),
        "ol" | "ul" => "list".to_string(),
        "option" => "option".to_string(),
        "p" => "paragraph".to_string(),
        "section" => {
            if has_accessible_name(ctx, attrs) {
                "region".to_string()
            } else {
                "generic".to_string()
            }
        }
        "select" => implicit_select_role(ctx, attrs),
        "strong" => "strong".to_string(),
        "sub" => "subscript".to_string(),
        "summary" => "button".to_string(),
        "sup" => "superscript".to_string(),
        "table" => "table".to_string(),
        "td" => "cell".to_string(),
        "textarea" => "textbox".to_string(),
        "th" => "columnheader".to_string(),
        "tr" => "row".to_string(),
        _ => "generic".to_string(),
    }
}

fn implicit_input_role(
    ctx: &ValidationContext,
    attrs: &[html_inspector::Attribute],
) -> String {
    let ty = ctx.attr_value(attrs, "type").unwrap_or("text").trim();
    let ty_lc = ty.to_ascii_lowercase();
    let role = match ty_lc.as_str() {
        "button" | "image" | "reset" | "submit" => "button",
        "checkbox" => "checkbox",
        "number" => "spinbutton",
        "radio" => "radio",
        "range" => "slider",
        "search" => "searchbox",
        "tel" | "text" | "url" | "email" | "" => "textbox",
        _ => "textbox",
    };
    role.to_string()
}

fn implicit_select_role(
    ctx: &ValidationContext,
    attrs: &[html_inspector::Attribute],
) -> String {
    let is_listbox = ctx.has_attr(attrs, "multiple")
        || ctx
            .attr_value(attrs, "size")
            .and_then(|size| size.trim().parse::<u32>().ok())
            .is_some_and(|n| n > 1);
    if is_listbox {
        "listbox".to_string()
    } else {
        "combobox".to_string()
    }
}

fn has_accessible_name(ctx: &ValidationContext, attrs: &[html_inspector::Attribute]) -> bool {
    ctx.has_attr(attrs, "aria-label") || ctx.has_attr(attrs, "aria-labelledby")
}

#[cfg(test)]
mod tests {
    use super::*;

    use html_inspector::{Attribute, Config, InputFormat, ValidationContext};

    fn attr(name: &str, value: Option<&str>) -> Attribute {
        Attribute {
            name: name.to_string(),
            value: value.map(|v| v.to_string()),
            span: None,
        }
    }

    #[test]
    fn computed_role_covers_area_form_section_and_select_branches() {
        let ctx = ValidationContext::new(Config::default(), InputFormat::Html);

        assert_eq!(
            computed_role(&ctx, "a", &[attr("href", Some("https://example.com/"))]),
            "link"
        );
        assert_eq!(computed_role(&ctx, "a", &[]), "generic");

        assert_eq!(
            computed_role(&ctx, "area", &[attr("href", Some("https://example.com/"))]),
            "link"
        );
        assert_eq!(computed_role(&ctx, "area", &[]), "generic");

        assert_eq!(computed_role(&ctx, "form", &[]), "generic");
        assert_eq!(
            computed_role(&ctx, "form", &[attr("aria-label", Some("x"))]),
            "form"
        );

        assert_eq!(computed_role(&ctx, "section", &[]), "generic");
        assert_eq!(
            computed_role(&ctx, "section", &[attr("aria-labelledby", Some("x"))]),
            "region"
        );

        assert_eq!(
            implicit_select_role(&ctx, &[attr("multiple", None)]),
            "listbox"
        );
        assert_eq!(
            implicit_select_role(&ctx, &[attr("size", Some("2"))]),
            "listbox"
        );
        assert_eq!(
            implicit_select_role(&ctx, &[attr("size", Some("1"))]),
            "combobox"
        );
    }

    #[test]
    fn computed_role_prefers_role_attribute_and_handles_xhtml_matching() {
        let ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        let attrs = [attr("role", Some("  presentation \t somethingelse  "))];
        assert_eq!(computed_role(&ctx, "div", &attrs), "presentation");
        assert!(ctx.has_attr(&attrs, "role"));
        assert_eq!(
            ctx.attr_value(&attrs, "role"),
            Some("  presentation \t somethingelse  ")
        );
    }

    #[test]
    fn computed_role_covers_more_branches_and_input_role_mapping() {
        let ctx = ValidationContext::new(Config::default(), InputFormat::Html);

        assert_eq!(computed_role(&ctx, "details", &[]), "group");
        assert_eq!(computed_role(&ctx, "dialog", &[]), "dialog");
        assert_eq!(computed_role(&ctx, "figure", &[]), "figure");
        assert_eq!(computed_role(&ctx, "footer", &[]), "contentinfo");
        assert_eq!(computed_role(&ctx, "header", &[]), "banner");
        assert_eq!(computed_role(&ctx, "h1", &[]), "heading");
        assert_eq!(computed_role(&ctx, "nav", &[]), "navigation");
        assert_eq!(computed_role(&ctx, "table", &[]), "table");
        assert_eq!(computed_role(&ctx, "textarea", &[]), "textbox");
        assert_eq!(computed_role(&ctx, "unknown", &[]), "generic");

        assert_eq!(
            implicit_input_role(&ctx, &[attr("type", Some("checkbox"))]),
            "checkbox"
        );
        assert_eq!(
            implicit_input_role(&ctx, &[attr("type", Some("range"))]),
            "slider"
        );
        assert_eq!(
            implicit_input_role(&ctx, &[attr("type", Some("number"))]),
            "spinbutton"
        );
        assert_eq!(
            implicit_input_role(&ctx, &[attr("type", Some("search"))]),
            "searchbox"
        );
        assert_eq!(
            implicit_input_role(&ctx, &[attr("type", Some("tel"))]),
            "textbox"
        );
        assert_eq!(
            implicit_input_role(&ctx, &[attr("type", Some("button"))]),
            "button"
        );
        assert_eq!(
            implicit_input_role(&ctx, &[attr("type", Some("CHECKBOX"))]),
            "checkbox"
        );

        assert_eq!(super::tag_name_for_message(&ctx, "DIV"), "div");
    }
}
