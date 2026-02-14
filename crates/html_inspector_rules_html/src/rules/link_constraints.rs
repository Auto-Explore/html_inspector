use html_inspector_core::{
    Category, DocumentSection, Interest, Message, MessageSink, ParseEvent, Rule, Severity,
    ValidationContext,
};

#[derive(Default)]
pub struct LinkConstraints;

impl Rule for LinkConstraints {
    fn id(&self) -> &'static str {
        "html.link.constraints"
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
        if !is(ctx, name, "link") {
            return;
        }

        let has_href = attrs.iter().any(|a| match ctx.format {
            html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case("href"),
            html_inspector_core::InputFormat::Xhtml => a.name == "href",
        });
        let imagesrcset = attrs
            .iter()
            .find(|a| match ctx.format {
                html_inspector_core::InputFormat::Html => {
                    a.name.eq_ignore_ascii_case("imagesrcset")
                }
                html_inspector_core::InputFormat::Xhtml => a.name == "imagesrcset",
            })
            .and_then(|a| a.value.as_deref());
        let has_imagesrcset = imagesrcset.is_some();
        let has_imagesizes = attrs.iter().any(|a| match ctx.format {
            html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case("imagesizes"),
            html_inspector_core::InputFormat::Xhtml => a.name == "imagesizes",
        });
        if has_imagesizes && !has_imagesrcset {
            out.push(Message::new(
                "html.link.imagesizes.requires_imagesrcset",
                Severity::Error,
                Category::Html,
                "The “imagesizes” attribute must only be specified if the “imagesrcset” attribute is also specified.",
                *span,
            ));
        }
        let has_rdfa = attrs.iter().any(|a| match ctx.format {
            html_inspector_core::InputFormat::Html => {
                let n = a.name.as_str();
                n.eq_ignore_ascii_case("about")
                    || n.eq_ignore_ascii_case("datatype")
                    || n.eq_ignore_ascii_case("inlist")
                    || n.eq_ignore_ascii_case("prefix")
                    || n.eq_ignore_ascii_case("property")
                    || n.eq_ignore_ascii_case("resource")
                    || n.eq_ignore_ascii_case("typeof")
                    || n.eq_ignore_ascii_case("vocab")
            }
            html_inspector_core::InputFormat::Xhtml => matches!(
                a.name.as_str(),
                "about"
                    | "datatype"
                    | "inlist"
                    | "prefix"
                    | "property"
                    | "resource"
                    | "typeof"
                    | "vocab"
            ),
        });

        if !has_href && !has_imagesrcset && !has_rdfa {
            out.push(Message::new(
                "html.link.href.required",
                Severity::Error,
                Category::Html,
                "A “link” element must have an “href” or “imagesrcset” attribute, or both.",
                *span,
            ));
            return;
        }

        let rel = attrs
            .iter()
            .find(|a| match ctx.format {
                html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case("rel"),
                html_inspector_core::InputFormat::Xhtml => a.name == "rel",
            })
            .and_then(|a| a.value.as_deref())
            .unwrap_or("");

        let mut rel_has_preload = false;
        let mut rel_has_modulepreload = false;
        let mut rel_has_stylesheet = false;
        let mut rel_has_alternate = false;
        let mut rel_has_mask_icon = false;
        let mut rel_has_dns_prefetch = false;
        let mut rel_has_pingback = false;
        let mut rel_has_preconnect = false;
        let mut rel_has_prefetch = false;
        let mut rel_has_prerender = false;
        let mut rel_has_icon = false;
        let mut rel_has_apple_touch_icon = false;
        let mut rel_has_apple_touch_icon_precomposed = false;
        for t in rel.split_ascii_whitespace() {
            if t.eq_ignore_ascii_case("preload") {
                rel_has_preload = true;
            } else if t.eq_ignore_ascii_case("modulepreload") {
                rel_has_modulepreload = true;
            } else if t.eq_ignore_ascii_case("stylesheet") {
                rel_has_stylesheet = true;
            } else if t.eq_ignore_ascii_case("alternate") {
                rel_has_alternate = true;
            } else if t.eq_ignore_ascii_case("mask-icon") {
                rel_has_mask_icon = true;
            } else if t.eq_ignore_ascii_case("dns-prefetch") {
                rel_has_dns_prefetch = true;
            } else if t.eq_ignore_ascii_case("pingback") {
                rel_has_pingback = true;
            } else if t.eq_ignore_ascii_case("preconnect") {
                rel_has_preconnect = true;
            } else if t.eq_ignore_ascii_case("prefetch") {
                rel_has_prefetch = true;
            } else if t.eq_ignore_ascii_case("prerender") {
                rel_has_prerender = true;
            } else if t.eq_ignore_ascii_case("icon") {
                rel_has_icon = true;
            } else if t.eq_ignore_ascii_case("apple-touch-icon") {
                rel_has_apple_touch_icon = true;
            } else if t.eq_ignore_ascii_case("apple-touch-icon-precomposed") {
                rel_has_apple_touch_icon_precomposed = true;
            }
        }

        let as_attr = attrs
            .iter()
            .find(|a| match ctx.format {
                html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case("as"),
                html_inspector_core::InputFormat::Xhtml => a.name == "as",
            })
            .and_then(|a| a.value.as_deref());
        let has_as = as_attr.is_some();
        let as_value = as_attr.unwrap_or("");
        if has_as && !(rel_has_preload || rel_has_modulepreload) {
            out.push(Message::new(
                "html.link.as.requires_preload",
                Severity::Error,
                Category::Html,
                "A “link” element with an “as” attribute must have a “rel” attribute that contains the value “preload” or the value “modulepreload”.",
                *span,
            ));
        }

        if rel_has_preload && !has_as {
            out.push(Message::new(
                "html.link.preload.requires_as",
                Severity::Error,
                Category::Html,
                "A “link” element with a “rel” attribute that contains the value “preload” must have an “as” attribute.",
                *span,
            ));
        }

        if has_imagesrcset {
            if !rel_has_preload {
                out.push(Message::new(
                    "html.link.imagesrcset.requires_preload",
                    Severity::Error,
                    Category::Html,
                    "A “link” element with an “imagesrcset” attribute must have a “rel” attribute that contains the value “preload”.",
                    *span,
                ));
            }

            let as_is_image = match ctx.format {
                html_inspector_core::InputFormat::Html => as_value.eq_ignore_ascii_case("image"),
                html_inspector_core::InputFormat::Xhtml => as_value == "image",
            };
            if !as_is_image {
                out.push(Message::new(
                    "html.link.imagesrcset.requires_as_image",
                    Severity::Error,
                    Category::Html,
                    "A “link” element with an “imagesrcset” attribute must have an “as” attribute with value “image”.",
                    *span,
                ));
            }

            if let Some(imagesrcset) = imagesrcset
                && !has_imagesizes && srcset_has_width_descriptor(imagesrcset) {
                    out.push(Message::new(
                        "html.link.imagesrcset.width_descriptor_requires_imagesizes",
                        Severity::Error,
                        Category::Html,
                        "When the “imagesrcset” attribute has any image candidate string with a width descriptor, the “imagesizes” attribute must also be specified.",
                        *span,
                    ));
                }
        }

        let has_blocking = attrs.iter().any(|a| match ctx.format {
            html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case("blocking"),
            html_inspector_core::InputFormat::Xhtml => a.name == "blocking",
        });
        if has_blocking {
            let rel_is_stylesheet_only = match ctx.format {
                html_inspector_core::InputFormat::Html => {
                    rel.trim().eq_ignore_ascii_case("stylesheet")
                }
                html_inspector_core::InputFormat::Xhtml => rel.trim() == "stylesheet",
            };
            if !rel_is_stylesheet_only {
                out.push(Message::new(
                    "html.link.blocking.requires_stylesheet",
                    Severity::Error,
                    Category::Html,
                    "A “link” element with a “blocking” attribute must have a “rel” attribute whose value is “stylesheet”.",
                    *span,
                ));
            }
        }

        let has_disabled = attrs.iter().any(|a| match ctx.format {
            html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case("disabled"),
            html_inspector_core::InputFormat::Xhtml => a.name == "disabled",
        });
        if has_disabled && !rel_has_stylesheet {
            out.push(Message::new(
                "html.link.disabled.requires_stylesheet",
                Severity::Error,
                Category::Html,
                "A “link” element with a “disabled” attribute must have a “rel” attribute that contains the value “stylesheet”.",
                *span,
            ));
        }

        let has_color = attrs.iter().any(|a| match ctx.format {
            html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case("color"),
            html_inspector_core::InputFormat::Xhtml => a.name == "color",
        });
        if has_color && !rel_has_mask_icon {
            out.push(Message::new(
                "html.link.color.requires_mask_icon",
                Severity::Error,
                Category::Html,
                "A “link” element with a “color” attribute must have a “rel” attribute that contains the value “mask-icon”.",
                *span,
            ));
        }

        if rel_has_alternate && rel_has_stylesheet {
            let title = attrs
                .iter()
                .find(|a| match ctx.format {
                    html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case("title"),
                    html_inspector_core::InputFormat::Xhtml => a.name == "title",
                })
                .and_then(|a| a.value.as_deref())
                .unwrap_or("");
            if title.is_empty() {
                out.push(Message::new(
                    "html.link.alternate_stylesheet.title.required",
                    Severity::Error,
                    Category::Html,
                    "A “link” element with a “rel” attribute that contains both the values “alternate” and “stylesheet” must have a “title” attribute with a non-empty value.",
                    *span,
                ));
            }
        }

        let has_integrity = attrs.iter().any(|a| match ctx.format {
            html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case("integrity"),
            html_inspector_core::InputFormat::Xhtml => a.name == "integrity",
        });
        if has_integrity && !(rel_has_stylesheet || rel_has_preload || rel_has_modulepreload) {
            out.push(Message::new(
                "html.link.integrity.requires_stylesheet_or_preload",
                Severity::Error,
                Category::Html,
                "A “link” element with an “integrity” attribute must have a “rel” attribute that contains the value “stylesheet” or the value “preload” or the value “modulepreload”.",
                *span,
            ));
        }

        let has_itemprop = attrs.iter().any(|a| match ctx.format {
            html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case("itemprop"),
            html_inspector_core::InputFormat::Xhtml => a.name == "itemprop",
        });
        let has_property = attrs.iter().any(|a| match ctx.format {
            html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case("property"),
            html_inspector_core::InputFormat::Xhtml => a.name == "property",
        });
        let has_rel = attrs.iter().any(|a| match ctx.format {
            html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case("rel"),
            html_inspector_core::InputFormat::Xhtml => a.name == "rel",
        });
        if !has_itemprop && !has_property && !has_rel {
            out.push(Message::new(
                "html.link.missing_rel_or_itemprop_or_property",
                Severity::Error,
                Category::Html,
                "Element “link” is missing one or more of the following attributes: “itemprop”, “property”, “rel”.",
                *span,
            ));
        }
        if has_itemprop && has_rel {
            out.push(Message::new(
                "html.link.itemprop.rel.disallowed",
                Severity::Error,
                Category::Html,
                "Attribute “rel” not allowed on element “link” at this point.",
                *span,
            ));
        }

        let has_sizes = attrs.iter().any(|a| match ctx.format {
            html_inspector_core::InputFormat::Html => a.name.eq_ignore_ascii_case("sizes"),
            html_inspector_core::InputFormat::Xhtml => a.name == "sizes",
        });
        if has_sizes
            && !(rel_has_icon || rel_has_apple_touch_icon || rel_has_apple_touch_icon_precomposed)
        {
            out.push(Message::new(
                "html.link.sizes.requires_icon_rel",
                Severity::Error,
                Category::Html,
                "A “link” element with a “sizes” attribute must have a “rel” attribute that contains the value “icon” or the value “apple-touch-icon” or the value “apple-touch-icon-precomposed”.",
                *span,
            ));
        }

        if ctx.document.section == DocumentSection::Body && !has_itemprop && !has_rdfa {
            let allowed_in_body = rel_has_dns_prefetch
                || rel_has_modulepreload
                || rel_has_pingback
                || rel_has_preconnect
                || rel_has_prefetch
                || rel_has_preload
                || rel_has_prerender
                || rel_has_stylesheet;
            if !allowed_in_body {
                out.push(Message::new(
                    "html.link.in_body.disallowed_rel",
                    Severity::Error,
                    Category::Html,
                    "A “link” element must not appear as a descendant of a “body” element unless the “link” element has an “itemprop” attribute or has a “rel” attribute whose value contains “dns-prefetch”, “modulepreload”, “pingback”, “preconnect”, “prefetch”, “preload”, “prerender”, or “stylesheet”.",
                    *span,
                ));
            }
        }
    }
}

fn srcset_has_width_descriptor(srcset: &str) -> bool {
    for candidate in srcset.split(',') {
        let candidate = candidate.trim();
        if candidate.is_empty() {
            continue;
        }
        for token in candidate.split_ascii_whitespace().skip(1) {
            let token = token.trim();
            if let Some(num) = token.strip_suffix('w')
                && !num.is_empty() && num.chars().all(|c| c.is_ascii_digit()) {
                    return true;
                }
        }
    }
    false
}

fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    match ctx.format {
        html_inspector_core::InputFormat::Html => actual.eq_ignore_ascii_case(expected),
        html_inspector_core::InputFormat::Xhtml => actual == expected,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use html_inspector_core::{Config, InputFormat, RuleSet};
    use html_inspector_html::HtmlEventSource;

    fn validate_xhtml(xhtml: &str) -> Vec<html_inspector_core::Message> {
        let src = HtmlEventSource::from_str("t", InputFormat::Xhtml, xhtml).unwrap();
        html_inspector_core::validate_events(
            src,
            RuleSet::new().push(LinkConstraints::default()),
            Config::default(),
        )
        .unwrap()
        .messages
    }

    #[test]
    fn xhtml_attribute_matching_covers_multiple_link_checks() {
        let msgs = validate_xhtml(
            r#"
            <body>
              <link href="x" imagesizes="100vw"/>
              <link href="x" imagesrcset="a 1w"/>
              <link href="x" rel="alternate stylesheet" title=""/>
              <link href="x" rel="icon" integrity="sha256-x"/>
              <link href="x" blocking rel="preload"/>
              <link href="x" disabled rel="preload"/>
              <link href="x" color="red" rel="icon"/>
              <link href="x"/>
              <link href="x" rel="next"/>
            </body>
            "#,
        );
        assert!(msgs
            .iter()
            .any(|m| m.code == "html.link.imagesizes.requires_imagesrcset"));
        assert!(msgs
            .iter()
            .any(|m| m.code == "html.link.imagesrcset.width_descriptor_requires_imagesizes"));
        assert!(msgs
            .iter()
            .any(|m| m.code == "html.link.alternate_stylesheet.title.required"));
        assert!(msgs
            .iter()
            .any(|m| m.code == "html.link.integrity.requires_stylesheet_or_preload"));
        assert!(msgs
            .iter()
            .any(|m| m.code == "html.link.blocking.requires_stylesheet"));
        assert!(msgs
            .iter()
            .any(|m| m.code == "html.link.disabled.requires_stylesheet"));
        assert!(msgs
            .iter()
            .any(|m| m.code == "html.link.color.requires_mask_icon"));
        assert!(msgs
            .iter()
            .any(|m| m.code == "html.link.missing_rel_or_itemprop_or_property"));
        assert!(msgs
            .iter()
            .any(|m| m.code == "html.link.in_body.disallowed_rel"));
    }
}
