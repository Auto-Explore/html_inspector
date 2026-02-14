use html_inspector::{
    Category, DocumentSection, Interest, Message, MessageSink, ParseEvent, Rule, Severity,
    ValidationContext,
};

use super::foreign_content::{self, Namespace};

#[derive(Default)]
pub struct StyleConstraints;

impl Rule for StyleConstraints {
    fn id(&self) -> &'static str {
        "html.style.constraints"
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
        if foreign_content::namespace_for_next_start_tag(ctx, name) != Namespace::Html {
            return;
        }
        if name != "style" {
            return;
        }

        // Match vnu.jar behavior: schema/Assertions checks are skipped inside <template> contents.
        if ctx.has_ancestor("template") {
            return;
        }

        if ctx.document.section != DocumentSection::Head {
            if foreign_content::last_open_svg_integration_point(ctx) == Some("desc") {
                return;
            }
            if let Some(parent) = ctx.current_parent() {
                out.push(Message::new(
                    "html.style.not_allowed_here",
                    Severity::Error,
                    Category::Html,
                    format!("Element “style” not allowed as child of “{parent}” in this context."),
                    *span,
                ));
            }
            return;
        }

        if attrs.iter().any(|a| a.name == "scoped") {
            out.push(Message::new(
                "html.style.scoped.disallowed",
                Severity::Error,
                Category::Html,
                "Attribute “scoped” not allowed on element “style” at this point.",
                *span,
            ));
            return;
        }

        let t = attrs
            .iter()
            .find(|a| a.name == "type")
            .and_then(|a| a.value.as_deref());
        if let Some(t) = t {
            let v = t.trim();
            if v.eq_ignore_ascii_case("text/css") {
                out.push(Message::new(
                    "html.style.type.unnecessary",
                    Severity::Warning,
                    Category::Html,
                    "The “type” attribute for the “style” element is not needed and should be omitted.",
                    *span,
                ));
            } else {
                out.push(Message::new(
                    "html.style.type.text_css_only",
                    Severity::Error,
                    Category::Html,
                    "The only allowed value for the “type” attribute for the “style” element is “text/css” (with no parameters). (But the attribute is not needed and should be omitted altogether.)",
                    *span,
                ));
            }
        }
    }
}
