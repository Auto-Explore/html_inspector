use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

use super::shared::tag_name_for_message;

#[derive(Default)]
pub struct AriaPropertiesSupportedByRole;

impl Rule for AriaPropertiesSupportedByRole {
    fn id(&self) -> &'static str {
        "aria.properties.supported_by_role"
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

        // Native-element special cases for ARIA range properties.
        if ctx.name_is(name, "progress")
            && ctx.has_attr(attrs, "aria-valuemax")
            && ctx.has_attr(attrs, "max")
        {
            out.push(Message::new(
                "aria.aria_valuemax.progress.with_max",
                Severity::Error,
                Category::Aria,
                "The “aria-valuemax” attribute must not be used on an element which has a “max” attribute.",
                *span,
            ));
            return;
        }

        if ctx.name_is(name, "meter") && ctx.has_attr(attrs, "aria-valuemin") {
            out.push(Message::new(
                "aria.aria_valuemin.meter.discouraged",
                Severity::Warning,
                Category::Aria,
                "The “aria-valuemin” attribute should not be used on a “meter” element.",
                *span,
            ));
            return;
        }

        if ctx.name_is(name, "input")
            && ctx
                .attr_value(attrs, "type")
                .is_some_and(|t| t.eq_ignore_ascii_case("number"))
        {
            if ctx.has_attr(attrs, "aria-valuemin") {
                if ctx.has_attr(attrs, "min") {
                    out.push(Message::new(
                        "aria.aria_valuemin.input_number.with_min",
                        Severity::Error,
                        Category::Aria,
                        "The “aria-valuemin” attribute must not be used on an element which has a “min” attribute.",
                        *span,
                    ));
                } else {
                    out.push(Message::new(
                        "aria.aria_valuemin.input_number.discouraged",
                        Severity::Warning,
                        Category::Aria,
                        "The “aria-valuemin” attribute should not be used on an “input” element which has a “type” attribute whose value is “number”.",
                        *span,
                    ));
                }
                return;
            }
            if ctx.has_attr(attrs, "aria-valuemax") {
                if ctx.has_attr(attrs, "max") {
                    out.push(Message::new(
                        "aria.aria_valuemax.input_number.with_max",
                        Severity::Error,
                        Category::Aria,
                        "The “aria-valuemax” attribute must not be used on an element which has a “max” attribute.",
                        *span,
                    ));
                } else {
                    out.push(Message::new(
                        "aria.aria_valuemax.input_number.discouraged",
                        Severity::Warning,
                        Category::Aria,
                        "The “aria-valuemax” attribute should not be used on an “input” element which has a “type” attribute whose value is “number”.",
                        *span,
                    ));
                }
                return;
            }
        }

        let role = ctx
            .attr_value(attrs, "role")
            .and_then(|v| v.split_ascii_whitespace().next())
            .map(|t| t.to_ascii_lowercase());
        let Some(role) = role else {
            // For these checks, lack of a role implies unsupported.
            let is_select = ctx.name_is(name, "select");
            let is_option = ctx.name_is(name, "option");
            let is_summary = ctx.name_is(name, "summary");
            let is_td_or_th = ctx.name_is(name, "td") || ctx.name_is(name, "th");
            if !is_select {
                emit_not_allowed(
                    out,
                    ctx,
                    name,
                    span,
                    "aria-multiselectable",
                    ctx.has_attr(attrs, "aria-multiselectable"),
                );
            }
            emit_not_allowed(
                out,
                ctx,
                name,
                span,
                "aria-placeholder",
                ctx.has_attr(attrs, "aria-placeholder"),
            );
            emit_not_allowed(
                out,
                ctx,
                name,
                span,
                "aria-readonly",
                ctx.has_attr(attrs, "aria-readonly"),
            );
            if !is_option && !is_summary && !is_td_or_th {
                emit_not_allowed(
                    out,
                    ctx,
                    name,
                    span,
                    "aria-selected",
                    ctx.has_attr(attrs, "aria-selected"),
                );
            }
            emit_not_allowed(
                out,
                ctx,
                name,
                span,
                "aria-valuemin",
                ctx.has_attr(attrs, "aria-valuemin"),
            );
            emit_not_allowed(
                out,
                ctx,
                name,
                span,
                "aria-valuemax",
                ctx.has_attr(attrs, "aria-valuemax"),
            );
            return;
        };

        if ctx.has_attr(attrs, "aria-multiselectable") && !supports_multiselectable(&role) {
            emit_not_allowed(out, ctx, name, span, "aria-multiselectable", true);
        }
        if ctx.has_attr(attrs, "aria-placeholder") && !supports_placeholder(&role) {
            emit_not_allowed(out, ctx, name, span, "aria-placeholder", true);
        }
        if ctx.has_attr(attrs, "aria-readonly") && !supports_readonly(&role) {
            emit_not_allowed(out, ctx, name, span, "aria-readonly", true);
        }
        if ctx.has_attr(attrs, "aria-expanded") && !supports_expanded(&role) {
            emit_not_allowed(out, ctx, name, span, "aria-expanded", true);
        }
        if ctx.has_attr(attrs, "aria-selected") && !supports_selected(&role) {
            emit_not_allowed(out, ctx, name, span, "aria-selected", true);
        }
        if ctx.has_attr(attrs, "aria-valuemin") && !supports_range(&role) {
            emit_not_allowed(out, ctx, name, span, "aria-valuemin", true);
        }
        if ctx.has_attr(attrs, "aria-valuemax") && !supports_range(&role) {
            emit_not_allowed(out, ctx, name, span, "aria-valuemax", true);
        }
    }
}

fn emit_not_allowed(
    out: &mut dyn MessageSink,
    ctx: &ValidationContext,
    name: &str,
    span: &Option<html_inspector::Span>,
    attr: &str,
    present: bool,
) {
    if !present {
        return;
    }
    let el = tag_name_for_message(ctx, name);
    out.push(Message::new(
        format!("aria.{attr}.not_allowed"),
        Severity::Error,
        Category::Aria,
        format!("Attribute “{attr}” not allowed on element “{el}” at this point."),
        *span,
    ));
}

fn supports_multiselectable(role_lc: &str) -> bool {
    matches!(
        role_lc,
        "grid" | "listbox" | "tablist" | "tree" | "treegrid"
    )
}

fn supports_placeholder(role_lc: &str) -> bool {
    matches!(role_lc, "textbox" | "searchbox" | "combobox")
}

fn supports_readonly(role_lc: &str) -> bool {
    matches!(
        role_lc,
        "textbox"
            | "searchbox"
            | "combobox"
            | "spinbutton"
            | "grid"
            | "gridcell"
            | "columnheader"
            | "rowheader"
            | "treegrid"
    )
}

fn supports_expanded(role_lc: &str) -> bool {
    matches!(
        role_lc,
        "alert"
            | "alertdialog"
            | "application"
            | "article"
            | "banner"
            | "button"
            | "columnheader"
            | "combobox"
            | "complementary"
            | "contentinfo"
            | "definition"
            | "dialog"
            | "document"
            | "form"
            | "grid"
            | "gridcell"
            | "group"
            | "heading"
            | "img"
            | "link"
            | "list"
            | "log"
            | "main"
            | "marquee"
            | "math"
            | "menu"
            | "menubar"
            | "navigation"
            | "note"
            | "radiogroup"
            | "region"
            | "rowheader"
            | "search"
            | "separator"
            | "status"
            | "tab"
            | "tablist"
            | "tabpanel"
            | "timer"
            | "toolbar"
            | "tooltip"
            | "tree"
            | "treeitem"
            | "treegrid"
    )
}

fn supports_selected(role_lc: &str) -> bool {
    matches!(
        role_lc,
        "option" | "tab" | "row" | "treeitem" | "gridcell" | "rowheader" | "columnheader"
    )
}

fn supports_range(role_lc: &str) -> bool {
    matches!(
        role_lc,
        "progressbar" | "scrollbar" | "slider" | "spinbutton"
    )
}
