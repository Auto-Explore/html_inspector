use html_inspector::{
    Category, Interest, Message, MessageSink, ParseEvent, Rule, Severity, ValidationContext,
};

use super::shared::{ascii_lowercase_cow_if_needed, attr_value, eq_name, starts_with_ascii_ci};

#[derive(Default)]
pub struct AutocompleteConstraints;

impl Rule for AutocompleteConstraints {
    fn id(&self) -> &'static str {
        "html.autocomplete.constraints"
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

        if !(eq_name(ctx, name, "input")
            || eq_name(ctx, name, "select")
            || eq_name(ctx, name, "textarea"))
        {
            return;
        }

        let element = name.as_str();

        let autocomplete = attr_value(ctx, attrs, "autocomplete");
        let Some(autocomplete) = autocomplete else {
            return;
        };
        let autocomplete_trim = autocomplete.trim();

        if autocomplete_trim.is_empty() {
            out.push(Message::new(
                "html.autocomplete.empty",
                Severity::Error,
                Category::Html,
                format!("Bad value “” for attribute “autocomplete” on element “{element}”."),
                *span,
            ));
            return;
        }

        if autocomplete_trim.eq_ignore_ascii_case("on")
            || autocomplete_trim.eq_ignore_ascii_case("off")
        {
            if eq_name(ctx, name, "input") {
                let input_type = attr_value(ctx, attrs, "type").unwrap_or("");
                if input_type.eq_ignore_ascii_case("hidden") {
                    out.push(Message::new(
                        "html.input.autocomplete.on_off_disallowed_on_hidden",
                        Severity::Error,
                        Category::Html,
                        "An “input” element with a “type” attribute whose value is “hidden” must not have an “autocomplete” attribute whose value is “on” or “off”.",
                        *span,
                    ));
                }
            }
            return;
        }

        if !is_valid_autocomplete_tokens(autocomplete_trim) {
            out.push(Message::new(
                "html.autocomplete.invalid",
                Severity::Error,
                Category::Html,
                format!("Bad value “{autocomplete_trim}” for attribute “autocomplete” on element “{element}”."),
                *span,
            ));
        }
    }
}

fn is_valid_autocomplete_tokens(value: &str) -> bool {
    let mut tokens = value.split_ascii_whitespace().peekable();

    // Optional section-*
    if tokens
        .peek()
        .is_some_and(|t| starts_with_ascii_ci(t, "section-"))
    {
        let t = tokens.next().unwrap();
        if t.len() <= "section-".len() {
            return false;
        }
    }

    // Optional address type.
    let has_address_type = tokens
        .peek()
        .is_some_and(|t| t.eq_ignore_ascii_case("shipping") || t.eq_ignore_ascii_case("billing"));
    if has_address_type {
        tokens.next();
    }

    // Optional contact type.
    let has_contact_type = tokens.peek().is_some_and(|t| {
        t.eq_ignore_ascii_case("home")
            || t.eq_ignore_ascii_case("work")
            || t.eq_ignore_ascii_case("mobile")
            || t.eq_ignore_ascii_case("fax")
            || t.eq_ignore_ascii_case("pager")
    });
    if has_contact_type {
        tokens.next();
    }

    // Exactly one field token must remain (plus an optional trailing "webauthn").
    let Some(field) = tokens.next() else {
        return false;
    };
    if !is_known_field(field) {
        return false;
    }

    // Contact-type tokens only make sense for contact fields.
    if has_contact_type && !is_contact_field(field) {
        return false;
    }

    match tokens.next() {
        None => true,
        Some(t) if t.eq_ignore_ascii_case("webauthn") => tokens.next().is_none(),
        Some(_) => false,
    }
}

fn is_contact_field(field: &str) -> bool {
    field.eq_ignore_ascii_case("email")
        || field.eq_ignore_ascii_case("impp")
        || field.eq_ignore_ascii_case("tel")
        || starts_with_ascii_ci(field, "tel-")
}

fn is_known_field(field: &str) -> bool {
    let field = ascii_lowercase_cow_if_needed(field);
    matches!(
        field.as_ref(),
        "name"
            | "honorific-prefix"
            | "given-name"
            | "additional-name"
            | "family-name"
            | "honorific-suffix"
            | "nickname"
            | "username"
            | "new-password"
            | "current-password"
            | "one-time-code"
            | "organization-title"
            | "organization"
            | "street-address"
            | "address-line1"
            | "address-line2"
            | "address-line3"
            | "address-level4"
            | "address-level3"
            | "address-level2"
            | "address-level1"
            | "country"
            | "country-name"
            | "postal-code"
            | "cc-name"
            | "cc-given-name"
            | "cc-additional-name"
            | "cc-family-name"
            | "cc-number"
            | "cc-exp"
            | "cc-exp-month"
            | "cc-exp-year"
            | "cc-csc"
            | "cc-type"
            | "transaction-currency"
            | "transaction-amount"
            | "language"
            | "bday"
            | "bday-day"
            | "bday-month"
            | "bday-year"
            | "sex"
            | "url"
            | "photo"
            | "tel"
            | "tel-country-code"
            | "tel-national"
            | "tel-area-code"
            | "tel-local"
            | "tel-local-prefix"
            | "tel-local-suffix"
            | "tel-extension"
            | "email"
            | "impp"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    use html_inspector::{Attribute, Config, InputFormat};

    #[derive(Default)]
    struct Sink(Vec<Message>);
    impl MessageSink for Sink {
        fn push(&mut self, msg: Message) {
            self.0.push(msg);
        }
    }

    fn attr(name: &str, value: Option<&str>) -> Attribute {
        Attribute {
            name: name.to_string(),
            value: value.map(|v| v.to_string()),
            span: None,
        }
    }

    #[test]
    fn duplicate_autocomplete_uses_first_attribute_even_if_valueless() {
        let mut ctx = ValidationContext::new(Config::default(), InputFormat::Html);
        let mut sink = Sink::default();
        let mut rule = AutocompleteConstraints::default();

        rule.on_event(
            &ParseEvent::StartTag {
                name: "input".to_string(),
                attrs: vec![
                    attr("autocomplete", None),
                    attr("autocomplete", Some("nope")),
                ],
                self_closing: true,
                span: None,
            },
            &mut ctx,
            &mut sink,
        );

        // The second attribute would be invalid, but it must not be observed due to the first
        // duplicate attribute having no value.
        assert!(sink.0.is_empty());
    }

    #[test]
    fn validates_token_sequences_without_allocating() {
        assert!(is_valid_autocomplete_tokens("name"));
        assert!(is_valid_autocomplete_tokens("section-x name"));
        assert!(is_valid_autocomplete_tokens("shipping name"));
        assert!(is_valid_autocomplete_tokens("home tel"));
        assert!(is_valid_autocomplete_tokens("name webauthn"));
        assert!(is_valid_autocomplete_tokens(
            "section-x home email webauthn"
        ));

        assert!(!is_valid_autocomplete_tokens(""));
        assert!(!is_valid_autocomplete_tokens("   "));
        assert!(!is_valid_autocomplete_tokens("section- name"));
        assert!(!is_valid_autocomplete_tokens("home name"));
        assert!(!is_valid_autocomplete_tokens("webauthn"));
        assert!(!is_valid_autocomplete_tokens("webauthn name"));
        assert!(!is_valid_autocomplete_tokens("name webauthn extra"));
        assert!(!is_valid_autocomplete_tokens("name extra"));
    }
}
