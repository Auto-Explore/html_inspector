pub use css_inspector::{
    starts_with_ascii_ci, url_decode_plus, validate_css_declarations_text, validate_css_text,
    validate_css_text_with_fetcher, validate_css_uri_with_fetcher, Config, Message, Report,
    Severity, StdFetcher, ValidatorError,
};

#[cfg(test)]
mod tests {
    #[test]
    fn starts_with_ascii_ci_is_available_via_wrapper_crate() {
        assert!(super::starts_with_ascii_ci("FILE://x", "file://"));
        assert!(!super::starts_with_ascii_ci("ftp://x", "file://"));
    }

    #[test]
    fn url_decode_plus_is_available_via_wrapper_crate() {
        assert_eq!(super::url_decode_plus("a+b%2F").unwrap(), "a b/");
    }
}
