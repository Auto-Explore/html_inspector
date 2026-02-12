pub fn is_invalid_media_query_list(media: &str) -> bool {
    let media = media.trim();
    if media.is_empty() {
        return false;
    }

    split_top_level_commas(media).any(|part| {
        let part = part.trim();
        part.is_empty() || is_invalid_media_query(part)
    })
}

struct SplitTopLevelCommas<'a> {
    s: &'a str,
    iter: std::str::CharIndices<'a>,
    depth: u32,
    start: usize,
    done: bool,
}

impl<'a> Iterator for SplitTopLevelCommas<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        for (i, c) in &mut self.iter {
            match c {
                '(' => self.depth += 1,
                ')' => self.depth = self.depth.saturating_sub(1),
                ',' if self.depth == 0 => {
                    let out = &self.s[self.start..i];
                    self.start = i + c.len_utf8();
                    return Some(out);
                }
                _ => {}
            }
        }

        self.done = true;
        Some(&self.s[self.start..])
    }
}

fn split_top_level_commas(s: &str) -> SplitTopLevelCommas<'_> {
    SplitTopLevelCommas {
        s,
        iter: s.char_indices(),
        depth: 0,
        start: 0,
        done: false,
    }
}

fn is_invalid_media_query(query: &str) -> bool {
    let mut p = Parser::new(query);
    p.skip_ws();
    if p.eof() {
        return true;
    }

    // Media condition only, e.g. `(min-width: 600px)`.
    if p.peek_char() == Some('(') {
        return is_invalid_media_condition_only(&mut p);
    }

    let Some(first) = p.parse_ident() else {
        return true;
    };

    let (modifier, media_type) =
        if first.eq_ignore_ascii_case("only") || first.eq_ignore_ascii_case("not") {
            p.skip_ws();
            let Some(t) = p.parse_ident() else {
                return true;
            };
            (Some(first), t)
        } else {
            (None, first)
        };

    if !is_ascii_ident(media_type) {
        return true;
    }
    if !(media_type.eq_ignore_ascii_case("all")
        || media_type.eq_ignore_ascii_case("print")
        || media_type.eq_ignore_ascii_case("screen"))
    {
        return true;
    }

    // `only`/`not` modifiers are only valid for known media types (handled above).
    let _ = modifier;

    loop {
        p.skip_ws();
        if p.eof() {
            return false;
        }

        let Some(and) = p.parse_ident() else {
            return true;
        };
        if !and.eq_ignore_ascii_case("and") {
            return true;
        }

        // CSS tokenization: `and(` is a function token, not an ident token.
        if p.peek_char() == Some('(') {
            return true;
        }

        // Require a media condition after `and`.
        p.skip_ws();
        if !p.consume_char('(') {
            return true;
        }

        if is_invalid_media_feature_expression(&mut p) {
            return true;
        }
    }
}

fn is_invalid_media_condition_only(p: &mut Parser<'_>) -> bool {
    if !p.consume_char('(') {
        return true;
    }
    if is_invalid_media_feature_expression(p) {
        return true;
    }

    loop {
        p.skip_ws();
        if p.eof() {
            return false;
        }

        let Some(and) = p.parse_ident() else {
            return true;
        };
        if !and.eq_ignore_ascii_case("and") {
            return true;
        }
        // CSS tokenization: `and(` is a function token, not an ident token.
        if p.peek_char() == Some('(') {
            return true;
        }
        p.skip_ws();
        if !p.consume_char('(') {
            return true;
        }
        if is_invalid_media_feature_expression(p) {
            return true;
        }
    }
}

fn is_invalid_media_feature_expression(p: &mut Parser<'_>) -> bool {
    p.skip_ws();
    let Some(feature) = p.parse_ident() else {
        return true;
    };
    if !is_ascii_ident(feature) {
        return true;
    }

    p.skip_ws();
    if !p.consume_char(':') {
        return true;
    }

    let Some(raw_value) = p.consume_until(')') else {
        return true;
    };
    if raw_value.contains(';') {
        return true;
    }
    let value = raw_value.trim();
    if value.is_empty() {
        return true;
    }

    let ok =
        if feature.eq_ignore_ascii_case("min-width") || feature.eq_ignore_ascii_case("max-width") {
            is_valid_length(value)
        } else if feature.eq_ignore_ascii_case("min-resolution")
            || feature.eq_ignore_ascii_case("max-resolution")
        {
            is_valid_resolution(value)
        } else if feature.eq_ignore_ascii_case("color") {
            is_valid_non_negative_integer(value)
        } else {
            false
        };
    if !ok {
        return true;
    }

    // `consume_until(')')` ensures the next character is a closing paren.
    p.consume_char(')');
    false
}

fn is_valid_non_negative_integer(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_digit())
}

fn is_valid_resolution(s: &str) -> bool {
    let (num, unit) = split_number_and_unit(s);
    let Some(num) = num else { return false };
    if unit.is_empty() {
        return num == 0.0;
    }
    matches!(unit, "dpi" | "dpcm" | "dppx")
}

fn is_valid_length(s: &str) -> bool {
    let (num, unit) = split_number_and_unit(s);
    let Some(num) = num else { return false };

    if unit.is_empty() {
        return num == 0.0;
    }

    matches!(
        unit,
        "px" | "em"
            | "rem"
            | "ex"
            | "ch"
            | "vw"
            | "vh"
            | "vmin"
            | "vmax"
            | "cm"
            | "mm"
            | "q"
            | "in"
            | "pt"
            | "pc"
            | "lh"
            | "rlh"
    )
}

fn split_number_and_unit(s: &str) -> (Option<f64>, &str) {
    let s = s.trim();
    if s.is_empty() {
        return (None, "");
    }
    if s.contains('/') {
        return (None, "");
    }

    let mut idx = 0usize;
    let bytes = s.as_bytes();
    let mut saw_digit = false;
    while idx < bytes.len() && bytes[idx].is_ascii_digit() {
        saw_digit = true;
        idx += 1;
    }
    if idx < bytes.len() && bytes[idx] == b'.' {
        idx += 1;
        while idx < bytes.len() && bytes[idx].is_ascii_digit() {
            saw_digit = true;
            idx += 1;
        }
    }
    if !saw_digit {
        return (None, "");
    }

    let (num_part, unit_part) = s.split_at(idx);
    let num = num_part.parse::<f64>().ok();
    let unit = unit_part.trim();
    if !unit.chars().all(|c| c.is_ascii_alphabetic()) {
        return (None, "");
    }
    (num, unit)
}

fn is_ascii_ident(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let mut saw_alpha = false;
    for b in s.as_bytes() {
        if b.is_ascii_alphabetic() {
            saw_alpha = true;
            continue;
        }
        if b.is_ascii_digit() || *b == b'-' {
            continue;
        }
        return false;
    }
    saw_alpha
}

struct Parser<'a> {
    s: &'a str,
    i: usize,
}

impl<'a> Parser<'a> {
    fn new(s: &'a str) -> Self {
        Self { s, i: 0 }
    }

    fn eof(&self) -> bool {
        self.i >= self.s.len()
    }

    fn peek_char(&self) -> Option<char> {
        self.s[self.i..].chars().next()
    }

    fn skip_ws(&mut self) {
        while let Some(c) = self.peek_char() {
            if c.is_whitespace() {
                self.i += c.len_utf8();
            } else {
                break;
            }
        }
    }

    fn consume_char(&mut self, c: char) -> bool {
        if self.peek_char() == Some(c) {
            self.i += c.len_utf8();
            true
        } else {
            false
        }
    }

    fn parse_ident(&mut self) -> Option<&'a str> {
        let start = self.i;
        let mut saw = false;
        while let Some(c) = self.peek_char() {
            if c.is_ascii_alphanumeric() || c == '-' {
                saw = true;
                self.i += c.len_utf8();
            } else {
                break;
            }
        }
        if !saw {
            return None;
        }
        Some(&self.s[start..self.i])
    }

    fn consume_until(&mut self, needle: char) -> Option<&'a str> {
        let start = self.i;
        while let Some(c) = self.peek_char() {
            if c == needle {
                return Some(&self.s[start..self.i]);
            }
            self.i += c.len_utf8();
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn media_query_list_accepts_common_valid_cases() {
        assert!(!is_invalid_media_query_list("screen"));
        assert!(!is_invalid_media_query_list("  screen  "));
        assert!(!is_invalid_media_query_list("ONLY screen"));
        assert!(!is_invalid_media_query_list(
            "screen and (min-width: 600px)"
        ));
        assert!(!is_invalid_media_query_list("(min-width: 0px)"));
        assert!(!is_invalid_media_query_list("(min-resolution: 0)"));
        assert!(!is_invalid_media_query_list(
            "screen and (max-resolution: 2dppx), (min-width: 1px)"
        ));
    }

    #[test]
    fn media_query_list_rejects_empty_items_and_unknown_media_types() {
        assert!(is_invalid_media_query_list(","));
        assert!(is_invalid_media_query_list("screen,   "));
        assert!(is_invalid_media_query_list("tv"));
        assert!(is_invalid_media_query_list("only"));
        assert!(is_invalid_media_query_list("not"));
    }

    #[test]
    fn media_query_list_rejects_bad_and_syntax_and_missing_parens() {
        assert!(is_invalid_media_query_list("screen and(min-width: 1px)"));
        assert!(is_invalid_media_query_list("screen and min-width: 1px"));
        assert!(is_invalid_media_query_list(
            "(min-width: 1px and (max-width: 2px)"
        ));
    }

    #[test]
    fn media_query_list_rejects_invalid_feature_expressions() {
        assert!(is_invalid_media_query_list("(min-width: )"));
        assert!(is_invalid_media_query_list("(min-width: 1px;)"));
        assert!(is_invalid_media_query_list("(min-width: 1/2px)"));
        assert!(is_invalid_media_query_list("(min-resolution: 1foo)"));
        assert!(is_invalid_media_query_list("(foo: 1px)"));
    }

    #[test]
    fn media_query_list_splits_top_level_commas_only() {
        let parts: Vec<&str> =
            split_top_level_commas("screen and (min-width: 1px), (min-width: 2px)").collect();
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0].trim(), "screen and (min-width: 1px)");
        assert_eq!(parts[1].trim(), "(min-width: 2px)");
    }

    #[test]
    fn split_top_level_commas_is_non_panicking_with_unmatched_closing_parens() {
        let parts: Vec<&str> = split_top_level_commas("a),b").collect();
        assert_eq!(parts, vec!["a)", "b"]);
    }

    #[test]
    fn parser_helpers_cover_ident_and_until_paths() {
        let mut p = Parser::new("  abc-123)");
        p.skip_ws();
        assert_eq!(p.parse_ident(), Some("abc-123"));
        assert_eq!(p.consume_until(')').unwrap(), "");

        let mut p = Parser::new("   ");
        p.skip_ws();
        assert!(p.eof());
        assert_eq!(p.parse_ident(), None);
        assert_eq!(p.consume_until(')'), None);
    }

    #[test]
    fn media_query_internal_invalid_paths_cover_more_branches() {
        // Empty/whitespace queries.
        assert!(is_invalid_media_query(""));
        assert!(is_invalid_media_query("   "));

        // Non-ident at start.
        assert!(is_invalid_media_query("1"));

        // Modifier without media type.
        assert!(is_invalid_media_query("only"));

        // Missing colon in feature expression.
        assert!(is_invalid_media_query("(min-width 1px)"));

        // Missing closing paren in condition-only query.
        assert!(is_invalid_media_query("(min-width: 1px"));

        // Directly cover condition-only and feature-expression entry guards.
        let mut p = Parser::new("min-width: 1px)");
        assert!(is_invalid_media_condition_only(&mut p));

        let mut p = Parser::new("min-width 1px)");
        assert!(is_invalid_media_feature_expression(&mut p));

        let mut p = Parser::new("min-width: 1px");
        assert!(is_invalid_media_feature_expression(&mut p));
    }

    #[test]
    fn condition_only_queries_can_chain_with_and() {
        assert!(!is_invalid_media_query_list(
            "(min-width: 1px) and (max-width: 2px)"
        ));
        assert!(!is_invalid_media_query_list(
            "(min-resolution: 0dppx) and (color: 1)"
        ));
    }

    #[test]
    fn condition_only_and_chaining_rejects_bad_tokens_and_missing_parens() {
        assert!(is_invalid_media_query_list(
            "(min-width: 1px) foo (max-width: 2px)"
        ));
        assert!(is_invalid_media_query_list(
            "(min-width: 1px) and(max-width: 2px)"
        ));
        assert!(is_invalid_media_query_list(
            "(min-width: 1px) and max-width: 2px"
        ));
    }

    #[test]
    fn number_and_unit_parsing_rejects_missing_digits_bad_units_and_resolution_without_unit() {
        assert!(is_invalid_media_query_list("(min-width: px)"));
        assert!(is_invalid_media_query_list("(min-width: 1p%)"));
        assert!(is_invalid_media_query_list("(min-resolution: 1)"));
    }

    #[test]
    fn media_query_rejects_non_ident_start_and_incomplete_and_clause() {
        assert!(is_invalid_media_query("@"));
        assert!(is_invalid_media_query_list("screen and"));
        assert!(is_invalid_media_query_list("screen foo (min-width: 1px)"));
    }

    #[test]
    fn media_query_missing_expected_idents_and_feature_names_are_invalid() {
        assert!(is_invalid_media_query("screen (min-width: 1px)"));
        assert!(is_invalid_media_query_list(
            "(min-width: 1px)(max-width: 2px)"
        ));
        assert!(is_invalid_media_query_list("( : 1px)"));
        assert!(is_invalid_media_query_list("(123: 1px)"));
        assert!(is_invalid_media_query_list(
            "(min-width: 1px) and (min-width: )"
        ));
    }

    #[test]
    fn split_number_and_unit_covers_empty_input() {
        assert_eq!(split_number_and_unit(""), (None, ""));
    }
}
