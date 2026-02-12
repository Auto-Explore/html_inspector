use html_inspector_core::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

use super::foreign_content::{self, Namespace};

#[derive(Default)]
pub struct ScriptConstraints;

impl Rule for ScriptConstraints {
    fn id(&self) -> &'static str {
        "html.script.constraints"
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
        if name != "script" {
            return;
        }

        let has_src = has_attr(attrs, "src");
        let has_charset = has_attr(attrs, "charset");
        if has_charset && !has_src {
            out.push(Message::new(
                "html.script.charset.requires_src",
                Severity::Error,
                Category::Html,
                "Element “script” must not have attribute “charset” unless attribute “src” is also specified.",
                *span,
            ));
            return;
        }

        let type_value = attr_value(attrs, "type").unwrap_or("").trim();
        let type_lower = html_inspector_core::ascii_lowercase_cow_if_needed(type_value);

        if has_attr(attrs, "language") {
            if attr_value(attrs, "language")
                .is_some_and(|l| l.trim().eq_ignore_ascii_case("javascript"))
                && !type_value.is_empty()
                && !type_value.eq_ignore_ascii_case("text/javascript")
            {
                out.push(Message::new(
                    "html.script.language.javascript.type_mismatch",
                    Severity::Error,
                    Category::Html,
                    "A “script” element with the “language=\"JavaScript\"” attribute set must not have a “type” attribute whose value is not “text/javascript”.",
                    *span,
                ));
                out.push(Message::new(
                    "html.script.language.obsolete",
                    Severity::Warning,
                    Category::Html,
                    "The “language” attribute on the “script” element is obsolete. Use the “type” attribute instead.",
                    *span,
                ));
                return;
            }

            out.push(Message::new(
                "html.script.language.obsolete",
                Severity::Warning,
                Category::Html,
                "The “language” attribute on the “script” element is obsolete. Use the “type” attribute instead.",
                *span,
            ));
        }

        if !type_value.is_empty() && is_javascript_mime_type(type_lower.as_ref()) {
            out.push(Message::new(
                "html.script.type.unnecessary",
                Severity::Warning,
                Category::Html,
                "The “type” attribute is unnecessary for JavaScript resources.",
                *span,
            ));
        }

        if has_charset && has_src {
            let charset_value = attr_value(attrs, "charset").unwrap_or("").trim();
            if !charset_value.eq_ignore_ascii_case("utf-8") {
                out.push(Message::new(
                    "html.script.charset.utf8_only",
                    Severity::Error,
                    Category::Html,
                    "The only allowed value for the “charset” attribute for the “script” element is “utf-8”. (But the attribute is not needed and should be omitted altogether.)",
                    *span,
                ));
                return;
            }
        }

        let is_module = type_lower == "module";
        let is_importmap = type_lower == "importmap";
        let is_speculationrules = type_lower == "speculationrules";
        let is_javascript = type_value.is_empty() || is_javascript_mime_type(type_lower.as_ref());
        let is_data_block = !is_javascript && !is_module && !is_importmap && !is_speculationrules;

        if is_data_block {
            if has_attr(attrs, "async") {
                out.push(Message::new(
                    "html.script.datablock.async.disallowed",
                    Severity::Error,
                    Category::Html,
                    "A “script” element with a “type” attribute whose value is neither a JavaScript MIME type, “module”, “importmap”, nor “speculationrules” (i.e., a data block) must not have an “async” attribute.",
                    *span,
                ));
                return;
            }
            if has_src {
                out.push(Message::new(
                    "html.script.datablock.src.disallowed",
                    Severity::Error,
                    Category::Html,
                    "A “script” element with a “type” attribute whose value is neither a JavaScript MIME type, “module”, “importmap”, nor “speculationrules” (i.e., a data block) must not have a “src” attribute.",
                    *span,
                ));
                return;
            }
            for (attr, code, msg) in [
                (
                    "blocking",
                    "html.script.datablock.blocking.disallowed",
                    "A “script” element with a “type” attribute whose value is neither a JavaScript MIME type, “module”, “importmap”, nor “speculationrules” (i.e., a data block) must not have a “blocking” attribute.",
                ),
                (
                    "crossorigin",
                    "html.script.datablock.crossorigin.disallowed",
                    "A “script” element with a “type” attribute whose value is neither a JavaScript MIME type, “module”, “importmap”, nor “speculationrules” (i.e., a data block) must not have a “crossorigin” attribute.",
                ),
                (
                    "defer",
                    "html.script.datablock.defer.disallowed",
                    "A “script” element with a “type” attribute whose value is neither a JavaScript MIME type, “module”, “importmap”, nor “speculationrules” (i.e., a data block) must not have a “defer” attribute.",
                ),
                (
                    "fetchpriority",
                    "html.script.datablock.fetchpriority.disallowed",
                    "A “script” element with a “type” attribute whose value is neither a JavaScript MIME type, “module”, “importmap”, nor “speculationrules” (i.e., a data block) must not have a “fetchpriority” attribute.",
                ),
                (
                    "integrity",
                    "html.script.datablock.integrity.disallowed",
                    "A “script” element with a “type” attribute whose value is neither a JavaScript MIME type, “module”, “importmap”, nor “speculationrules” (i.e., a data block) must not have an “integrity” attribute.",
                ),
                (
                    "nomodule",
                    "html.script.datablock.nomodule.disallowed",
                    "A “script” element with a “type” attribute whose value is neither a JavaScript MIME type, “module”, “importmap”, nor “speculationrules” (i.e., a data block) must not have a “nomodule” attribute.",
                ),
                (
                    "referrerpolicy",
                    "html.script.datablock.referrerpolicy.disallowed",
                    "A “script” element with a “type” attribute whose value is neither a JavaScript MIME type, “module”, “importmap”, nor “speculationrules” (i.e., a data block) must not have a “referrerpolicy” attribute.",
                ),
            ] {
                if has_attr(attrs, attr) {
                    out.push(Message::new(code, Severity::Error, Category::Html, msg, *span));
                    return;
                }
            }
        }

        if is_module {
            if has_attr(attrs, "defer") {
                out.push(Message::new(
                    "html.script.module.defer.disallowed",
                    Severity::Error,
                    Category::Html,
                    "A “script” element with “type=module” must not have a “defer” attribute.",
                    *span,
                ));
                return;
            }
            if has_attr(attrs, "nomodule") {
                out.push(Message::new(
                    "html.script.module.nomodule.disallowed",
                    Severity::Error,
                    Category::Html,
                    "A “script” element with a “nomodule” attribute must not have a “type” attribute with the value “module”.",
                    *span,
                ));
                return;
            }
        }

        if is_importmap {
            if has_attr(attrs, "async") {
                out.push(Message::new(
                    "html.script.importmap.async.disallowed",
                    Severity::Error,
                    Category::Html,
                    "A “script” element with “type=importmap” must not have an “async” attribute.",
                    *span,
                ));
                return;
            }
            if has_attr(attrs, "crossorigin") {
                out.push(Message::new(
                    "html.script.importmap.crossorigin.disallowed",
                    Severity::Error,
                    Category::Html,
                    "A “script” element with “type=importmap” must not have a “crossorigin” attribute.",
                    *span,
                ));
                return;
            }
            if has_attr(attrs, "defer") {
                out.push(Message::new(
                    "html.script.importmap.defer.disallowed",
                    Severity::Error,
                    Category::Html,
                    "A “script” element with “type=importmap” must not have a “defer” attribute.",
                    *span,
                ));
                return;
            }
            if has_src {
                out.push(Message::new(
                    "html.script.importmap.src.disallowed",
                    Severity::Error,
                    Category::Html,
                    "A “script” element with a “type” attribute whose value is “importmap” must not have a “src” attribute.",
                    *span,
                ));
                return;
            }
            for (attr, code, msg) in [
                (
                    "blocking",
                    "html.script.importmap.blocking.disallowed",
                    "A “script” element with “type=importmap” must not have a “blocking” attribute.",
                ),
                (
                    "fetchpriority",
                    "html.script.importmap.fetchpriority.disallowed",
                    "A “script” element with “type=importmap” must not have a “fetchpriority” attribute.",
                ),
                (
                    "integrity",
                    "html.script.importmap.integrity.disallowed",
                    "A “script” element with “type=importmap” must not have an “integrity” attribute.",
                ),
                (
                    "nomodule",
                    "html.script.importmap.nomodule.disallowed",
                    "A “script” element with “type=importmap” must not have a “nomodule” attribute.",
                ),
                (
                    "referrerpolicy",
                    "html.script.importmap.referrerpolicy.disallowed",
                    "A “script” element with “type=importmap” must not have a “referrerpolicy” attribute.",
                ),
            ] {
                if has_attr(attrs, attr) {
                    out.push(Message::new(code, Severity::Error, Category::Html, msg, *span));
                    return;
                }
            }
        }

        if is_speculationrules {
            if has_attr(attrs, "async") {
                out.push(Message::new(
                    "html.script.speculationrules.async.disallowed",
                    Severity::Error,
                    Category::Html,
                    "A “script” element with “type=speculationrules” must not have an “async” attribute.",
                    *span,
                ));
                return;
            }
            if has_attr(attrs, "blocking") {
                out.push(Message::new(
                    "html.script.speculationrules.blocking.disallowed",
                    Severity::Error,
                    Category::Html,
                    "A “script” element with “type=speculationrules” must not have a “blocking” attribute.",
                    *span,
                ));
                return;
            }
            if has_attr(attrs, "crossorigin") {
                out.push(Message::new(
                    "html.script.speculationrules.crossorigin.disallowed",
                    Severity::Error,
                    Category::Html,
                    "A “script” element with “type=speculationrules” must not have a “crossorigin” attribute.",
                    *span,
                ));
                return;
            }
            if has_attr(attrs, "defer") {
                out.push(Message::new(
                    "html.script.speculationrules.defer.disallowed",
                    Severity::Error,
                    Category::Html,
                    "A “script” element with “type=speculationrules” must not have a “defer” attribute.",
                    *span,
                ));
                return;
            }
            if has_attr(attrs, "fetchpriority") {
                out.push(Message::new(
                    "html.script.speculationrules.fetchpriority.disallowed",
                    Severity::Error,
                    Category::Html,
                    "A “script” element with “type=speculationrules” must not have a “fetchpriority” attribute.",
                    *span,
                ));
                return;
            }
            if has_attr(attrs, "integrity") {
                out.push(Message::new(
                    "html.script.speculationrules.integrity.disallowed",
                    Severity::Error,
                    Category::Html,
                    "A “script” element with “type=speculationrules” must not have an “integrity” attribute.",
                    *span,
                ));
                return;
            }
            if has_attr(attrs, "nomodule") {
                out.push(Message::new(
                    "html.script.speculationrules.nomodule.disallowed",
                    Severity::Error,
                    Category::Html,
                    "A “script” element with “type=speculationrules” must not have a “nomodule” attribute.",
                    *span,
                ));
                return;
            }
            if has_attr(attrs, "referrerpolicy") {
                out.push(Message::new(
                    "html.script.speculationrules.referrerpolicy.disallowed",
                    Severity::Error,
                    Category::Html,
                    "A “script” element with “type=speculationrules” must not have a “referrerpolicy” attribute.",
                    *span,
                ));
                return;
            }
            if has_src {
                out.push(Message::new(
                    "html.script.speculationrules.src.disallowed",
                    Severity::Error,
                    Category::Html,
                    "A “script” element with a “type” attribute whose value is “speculationrules” must not have a “src” attribute.",
                    *span,
                ));
                return;
            }
        }

        if is_module && !has_src {
            for (attr, code, msg) in [
                (
                    "blocking",
                    "html.script.module.inline.blocking.disallowed",
                    "An inline “script” element with “type=module” must not have a “blocking” attribute.",
                ),
                (
                    "fetchpriority",
                    "html.script.module.inline.fetchpriority.disallowed",
                    "An inline “script” element with “type=module” must not have a “fetchpriority” attribute.",
                ),
                (
                    "integrity",
                    "html.script.module.inline.integrity.disallowed",
                    "An inline “script” element with “type=module” must not have an “integrity” attribute.",
                ),
            ] {
                if has_attr(attrs, attr) {
                    out.push(Message::new(code, Severity::Error, Category::Html, msg, *span));
                    return;
                }
            }
        }

        if is_javascript && !has_src {
            if has_attr(attrs, "async") {
                out.push(Message::new(
                    "html.script.inline_classic.async.disallowed",
                    Severity::Error,
                    Category::Html,
                    "An inline classic “script” element (i.e., a “script” element without a “src” attribute and with a “type” attribute that is either unspecified, empty, or a JavaScript MIME type) must not have an “async” attribute.",
                    *span,
                ));
                return;
            }
            if has_attr(attrs, "defer") {
                out.push(Message::new(
                    "html.script.inline.defer.disallowed",
                    Severity::Error,
                    Category::Html,
                    "An inline “script” element (i.e., a “script” element without a “src” attribute and with a “type” attribute that is either unspecified, empty, or a JavaScript MIME type) must not have a “defer” attribute.",
                    *span,
                ));
                return;
            }
            for (attr, code, msg) in [
                (
                    "blocking",
                    "html.script.inline_classic.blocking.disallowed",
                    "An inline classic “script” element (i.e., a “script” element without a “src” attribute and with a “type” attribute that is either unspecified, empty, or a JavaScript MIME type) must not have a “blocking” attribute.",
                ),
                (
                    "fetchpriority",
                    "html.script.inline_classic.fetchpriority.disallowed",
                    "An inline classic “script” element (i.e., a “script” element without a “src” attribute and with a “type” attribute that is either unspecified, empty, or a JavaScript MIME type) must not have a “fetchpriority” attribute.",
                ),
                (
                    "integrity",
                    "html.script.inline_classic.integrity.disallowed",
                    "An inline classic “script” element (i.e., a “script” element without a “src” attribute and with a “type” attribute that is either unspecified, empty, or a JavaScript MIME type) must not have an “integrity” attribute.",
                ),
            ] {
                if has_attr(attrs, attr) {
                    out.push(Message::new(code, Severity::Error, Category::Html, msg, *span));
                    return;
                }
            }
        }
    }
}

fn is_javascript_mime_type(type_lower: &str) -> bool {
    matches!(
        type_lower,
        "application/javascript"
            | "application/ecmascript"
            | "text/javascript"
            | "text/ecmascript"
            | "application/x-javascript"
            | "application/x-ecmascript"
            | "text/javascript1.0"
            | "text/javascript1.1"
            | "text/javascript1.2"
            | "text/javascript1.3"
            | "text/javascript1.4"
            | "text/javascript1.5"
            | "text/jscript"
            | "text/livescript"
            | "text/x-javascript"
            | "text/x-ecmascript"
    )
}

fn has_attr(attrs: &[html_inspector_core::Attribute], name: &str) -> bool {
    attrs.iter().any(|a| a.name == name)
}

fn attr_value<'a>(attrs: &'a [html_inspector_core::Attribute], name: &str) -> Option<&'a str> {
    attrs
        .iter()
        .find(|a| a.name == name)
        .and_then(|a| a.value.as_deref())
}
