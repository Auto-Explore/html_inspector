use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

use super::foreign_content::{self, Namespace};

#[derive(Default)]
pub struct ObsoleteElements;

impl Rule for ObsoleteElements {
    fn id(&self) -> &'static str {
        "html.obsolete_elements"
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

        match name.as_str() {
            "head" => {
                let has_profile = attrs.iter().any(|a| a.name == "profile");
                if has_profile {
                    out.push(Message::new(
                        "html.attribute.profile.obsolete",
                        Severity::Warning,
                        Category::Html,
                        "The “profile” attribute on the “head” element is obsolete. To declare which “meta” terms are used in the document, instead register the names as meta extensions. To trigger specific UA behaviors, use a “link” element instead.",
                        *span,
                    ));
                }
            }
            "acronym" => {
                out.push(Message::new(
                    "html.element.acronym.obsolete",
                    Severity::Error,
                    Category::Html,
                    "The “acronym” element is obsolete. Use the “abbr” element instead.",
                    *span,
                ));
            }
            "applet" => {
                out.push(Message::new(
                    "html.element.applet.obsolete",
                    Severity::Error,
                    Category::Html,
                    "The “applet” element is obsolete. Use “embed” or “object” element instead.",
                    *span,
                ));
            }
            "basefont" => {
                if ctx.has_ancestor("head") && !ctx.has_ancestor("body") {
                    out.push(Message::new(
                        "html.element.basefont.disallowed_in_head",
                        Severity::Error,
                        Category::Html,
                        "Element “basefont” not allowed as child of “head” in this context.",
                        *span,
                    ));
                    return;
                }
                out.push(Message::new(
                    "html.element.basefont.obsolete",
                    Severity::Error,
                    Category::Html,
                    "The “basefont” element is obsolete. Use CSS instead.",
                    *span,
                ));
            }
            "big" => {
                out.push(Message::new(
                    "html.element.big.obsolete",
                    Severity::Error,
                    Category::Html,
                    "The “big” element is obsolete. Use CSS instead.",
                    *span,
                ));
            }
            "blink" => {
                out.push(Message::new(
                    "html.element.blink.obsolete",
                    Severity::Error,
                    Category::Html,
                    "The “blink” element is obsolete. Use CSS instead.",
                    *span,
                ));
            }
            "center" => {
                out.push(Message::new(
                    "html.element.center.obsolete",
                    Severity::Error,
                    Category::Html,
                    "The “center” element is obsolete. Use CSS instead.",
                    *span,
                ));
            }
            "font" => {
                out.push(Message::new(
                    "html.element.font.obsolete",
                    Severity::Error,
                    Category::Html,
                    "The “font” element is obsolete. Use CSS instead.",
                    *span,
                ));
            }
            "dir" => {
                out.push(Message::new(
                    "html.element.dir.obsolete",
                    Severity::Error,
                    Category::Html,
                    "The “dir” element is obsolete. Use the “ul” element instead.",
                    *span,
                ));
            }
            "frameset" => {
                out.push(Message::new(
                    "html.element.frameset.obsolete",
                    Severity::Error,
                    Category::Html,
                    "The “frameset” element is obsolete. Use the “iframe” element and CSS instead, or use server-side includes.",
                    *span,
                ));
            }
            "noframes" => {
                out.push(Message::new(
                    "html.element.noframes.obsolete",
                    Severity::Error,
                    Category::Html,
                    "The “noframes” element is obsolete. Use the “iframe” element and CSS instead, or use server-side includes.",
                    *span,
                ));
            }
            "keygen" => {
                out.push(Message::new(
                    "html.element.keygen.obsolete",
                    Severity::Error,
                    Category::Html,
                    "The “keygen” element is obsolete.",
                    *span,
                ));
            }
            "menuitem" => {
                out.push(Message::new(
                    "html.element.menuitem.obsolete",
                    Severity::Error,
                    Category::Html,
                    "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
                    *span,
                ));
            }
            "marquee" => {
                out.push(Message::new(
                    "html.element.marquee.obsolete",
                    Severity::Error,
                    Category::Html,
                    "The “marquee” element is obsolete. Use CSS instead.",
                    *span,
                ));
            }
            "nobr" => {
                out.push(Message::new(
                    "html.element.nobr.obsolete",
                    Severity::Error,
                    Category::Html,
                    "The “nobr” element is obsolete. Use CSS instead.",
                    *span,
                ));
            }
            "param" => {
                out.push(Message::new(
                    "html.element.param.obsolete",
                    Severity::Error,
                    Category::Html,
                    "The “param” element is obsolete. Use the “data” attribute of the “object” element to set the URL of the external resource.",
                    *span,
                ));
            }
            "strike" => {
                out.push(Message::new(
                    "html.element.strike.obsolete",
                    Severity::Error,
                    Category::Html,
                    "The “strike” element is obsolete. Use “del” or “s” element instead.",
                    *span,
                ));
            }
            "tt" => {
                out.push(Message::new(
                    "html.element.tt.obsolete",
                    Severity::Error,
                    Category::Html,
                    "The “tt” element is obsolete. Use CSS instead.",
                    *span,
                ));
            }
            _ => {}
        }
    }
}
