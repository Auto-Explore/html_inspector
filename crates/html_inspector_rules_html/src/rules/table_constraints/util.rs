use html_inspector::{Attribute, ValidationContext};

pub(super) fn attr_u32(ctx: &ValidationContext, attrs: &[Attribute], needle: &str) -> Option<u32> {
    ctx.attr_value(attrs, needle)?.trim().parse::<u32>().ok()
}

pub(super) fn attr_value<'a>(
    ctx: &ValidationContext,
    attrs: &'a [Attribute],
    needle: &str,
) -> Option<&'a str> {
    ctx.attr_value(attrs, needle)
}

pub(super) fn is(ctx: &ValidationContext, actual: &str, expected: &str) -> bool {
    ctx.name_is(actual, expected)
}

pub(super) fn is_row_group(ctx: &ValidationContext, actual: &str) -> bool {
    is(ctx, actual, "tbody") || is(ctx, actual, "thead") || is(ctx, actual, "tfoot")
}
