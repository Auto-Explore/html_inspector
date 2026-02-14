use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, ValidationContext,
};

use super::a_href_constraints::emit_forbidden_url_code_point;
use super::a_href_constraints::href_issue_severity;

#[derive(Default)]
pub struct MicrodataItemidConstraints;

impl Rule for MicrodataItemidConstraints {
    fn id(&self) -> &'static str {
        "html.microdata.itemid.datatype"
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

        let has_itemscope = ctx.has_attr(attrs, "itemscope");
        let has_itemtype = ctx.has_attr(attrs, "itemtype");
        if !(has_itemscope && has_itemtype) {
            return;
        }

        let itemid = ctx.attr_value(attrs, "itemid");
        let Some(itemid) = itemid else { return };

        // Empty string is allowed.
        if itemid.is_empty() {
            return;
        }

        if emit_forbidden_url_code_point(itemid, *span, out) {
            return;
        }

        if let Some(sev) = href_issue_severity(itemid) {
            out.push(Message::new(
                "html.microdata.itemid.invalid",
                sev,
                Category::Html,
                format!(
                    "Bad value “{itemid}” for attribute “itemid” on element “{}”.",
                    normalize_name(ctx, name)
                ),
                *span,
            ));
        }
    }
}

fn normalize_name(ctx: &ValidationContext, name: &str) -> String {
    match ctx.format {
        html_inspector::InputFormat::Html => name.to_ascii_lowercase(),
        html_inspector::InputFormat::Xhtml => name.to_string(),
    }
}
