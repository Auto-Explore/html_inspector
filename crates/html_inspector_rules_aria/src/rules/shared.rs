use std::borrow::Cow;

use html_inspector_core::{InputFormat, ValidationContext};

pub(crate) fn tag_name_for_message<'a>(ctx: &ValidationContext, name: &'a str) -> Cow<'a, str> {
    match ctx.format {
        InputFormat::Html => html_inspector_core::ascii_lowercase_cow_if_needed(name),
        InputFormat::Xhtml => Cow::Borrowed(name),
    }
}

#[cfg(test)]
mod tests {
    use super::tag_name_for_message;
    use html_inspector_core::{Config, InputFormat, ValidationContext};

    #[test]
    fn tag_name_for_message_preserves_case_in_xhtml() {
        let ctx = ValidationContext::new(Config::default(), InputFormat::Xhtml);
        assert_eq!(tag_name_for_message(&ctx, "BR"), "BR");
    }

    #[test]
    fn tag_name_for_message_lowercases_ascii_in_html() {
        let ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        assert_eq!(tag_name_for_message(&ctx, "BR"), "br");
        assert_eq!(tag_name_for_message(&ctx, "br"), "br");
    }
}
