pub(crate) use html_inspector::ends_with_ascii_ci;

#[cfg(test)]
mod tests {
    use super::ends_with_ascii_ci;

    #[test]
    fn ends_with_ascii_ci_matches_case_insensitively() {
        assert!(ends_with_ascii_ci("Hello.CSS", ".css"));
        assert!(ends_with_ascii_ci("Hello.css", ".CSS"));
        assert!(!ends_with_ascii_ci("Hello.html", ".css"));
    }

    #[test]
    fn ends_with_ascii_ci_handles_shorter_inputs_and_empty_suffix() {
        assert!(ends_with_ascii_ci("", ""));
        assert!(ends_with_ascii_ci("a", ""));
        assert!(!ends_with_ascii_ci("", "x"));
        assert!(!ends_with_ascii_ci("❤", "h"));
        assert!(ends_with_ascii_ci("❤H", "h"));
    }
}
