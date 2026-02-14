use html_inspector::{
    Category, ForeignContentNamespace, Interest, Message, MessageSink, ParseEvent, Rule, Severity,
    ValidationContext,
};

#[derive(Default)]
pub struct XmlLangConsistency;

impl Rule for XmlLangConsistency {
    fn id(&self) -> &'static str {
        "i18n.lang.xml_lang_consistency"
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

        // In XHTML, `xml:lang` is in the XML namespace and doesn't carry the HTML "no namespace"
        // constraint that VNU reports for HTML documents.
        if ctx.format == html_inspector::InputFormat::Xhtml {
            return;
        }

        if start_tag_namespace(ctx, name) != ForeignContentNamespace::Html {
            return;
        }

        let xml_lang = attrs
            .iter()
            .find(|a| a.name.eq_ignore_ascii_case("xml:lang"))
            .and_then(|a| a.value.as_deref());

        let Some(xml_lang) = xml_lang else { return };

        let lang = attrs
            .iter()
            .find(|a| a.name.eq_ignore_ascii_case("lang"))
            .and_then(|a| a.value.as_deref());

        let Some(lang) = lang else {
            out.push(Message::new(
                "i18n.lang.xml_lang.requires_lang",
                Severity::Error,
                Category::I18n,
                "When the attribute “xml:lang” in no namespace is specified, the element must also have the attribute “lang” present with the same value.",
                *span,
            ));
            return;
        };

        if xml_lang != lang {
            out.push(Message::new(
                "i18n.lang.xml_lang.mismatch",
                Severity::Error,
                Category::I18n,
                "When the attribute “xml:lang” in no namespace is specified, the element must also have the attribute “lang” present with the same value.",
                *span,
            ));
        }
    }
}

fn start_tag_namespace(ctx: &ValidationContext, name: &str) -> ForeignContentNamespace {
    match ctx.foreign_insertion_namespace() {
        ForeignContentNamespace::Html => {
            if name.eq_ignore_ascii_case("svg") {
                ForeignContentNamespace::Svg
            } else if name.eq_ignore_ascii_case("math") {
                ForeignContentNamespace::Math
            } else {
                ForeignContentNamespace::Html
            }
        }
        ns => ns,
    }
}
