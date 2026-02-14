use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

#[derive(Default)]
pub struct MicrodataConstraints;

impl Rule for MicrodataConstraints {
    fn id(&self) -> &'static str {
        "html.microdata.constraints"
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
        let ParseEvent::StartTag { attrs, span, .. } = event else {
            return;
        };

        let has_itemscope = attrs.iter().any(|a| match ctx.format {
            html_inspector::InputFormat::Html => a.name.eq_ignore_ascii_case("itemscope"),
            html_inspector::InputFormat::Xhtml => a.name == "itemscope",
        });
        let has_itemtype = attrs.iter().any(|a| match ctx.format {
            html_inspector::InputFormat::Html => a.name.eq_ignore_ascii_case("itemtype"),
            html_inspector::InputFormat::Xhtml => a.name == "itemtype",
        });
        let has_itemid = attrs.iter().any(|a| match ctx.format {
            html_inspector::InputFormat::Html => a.name.eq_ignore_ascii_case("itemid"),
            html_inspector::InputFormat::Xhtml => a.name == "itemid",
        });
        let has_itemref = attrs.iter().any(|a| match ctx.format {
            html_inspector::InputFormat::Html => a.name.eq_ignore_ascii_case("itemref"),
            html_inspector::InputFormat::Xhtml => a.name == "itemref",
        });

        if has_itemid && !(has_itemscope && has_itemtype) {
            out.push(Message::new(
                "html.microdata.itemid.requires_itemscope_itemtype",
                Severity::Error,
                Category::Html,
                "The “itemid” attribute must not be specified on elements that do not have both an “itemscope” attribute and an “itemtype” attribute specified.",
                *span,
            ));
        }

        if has_itemtype && !has_itemscope {
            out.push(Message::new(
                "html.microdata.itemtype.requires_itemscope",
                Severity::Error,
                Category::Html,
                "The “itemtype” attribute must not be specified on elements that do not have an “itemscope” attribute specified.",
                *span,
            ));
        }

        if has_itemref && !has_itemscope {
            out.push(Message::new(
                "html.microdata.itemref.requires_itemscope",
                Severity::Error,
                Category::Html,
                "The “itemref” attribute must not be specified on elements that do not have an “itemscope” attribute specified.",
                *span,
            ));
        }
    }
}
