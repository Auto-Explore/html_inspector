use std::path::{Path, PathBuf};
use std::sync::Arc;

use html_inspector::{Config, InputFormat, Severity};
use html_inspector_html::HtmlEventSource;
use html_inspector_rules_aria::pack_aria;
use html_inspector_rules_css::pack_css_checks;
use html_inspector_rules_html::pack_html_conformance;
use html_inspector_rules_i18n::pack_i18n;

#[derive(Debug)]
struct ActualError {
    code: String,
    span: html_inspector::Span,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct ExpectedError {
    code: String,
    line: u32,
    col: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    contains: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
enum ExpectedEntry {
    Detailed(ExpectedError),
    CodeOnly(String),
}

fn fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("invalid_html_fixtures")
}

fn expected_path_for_fixture(path: &Path) -> PathBuf {
    path.with_extension("expected.json")
}

fn suggested_expected_at(bytes: &[u8], byte_start: usize) -> Option<String> {
    let tail = bytes.get(byte_start..)?;
    let first = *tail.first()?;
    let is_name_char = |b: u8| matches!(b, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b':');

    if first == b'<' && tail.get(1).is_some_and(|&b| is_name_char(b)) {
        let mut end = 2usize;
        while end < tail.len() && is_name_char(tail[end]) {
            end += 1;
        }
        return Some(String::from_utf8_lossy(&tail[..end]).to_string());
    }

    if matches!(first, b'a'..=b'z' | b'A'..=b'Z') {
        let mut end = 1usize;
        while end < tail.len() && is_name_char(tail[end]) {
            end += 1;
        }
        if end >= 3 {
            return Some(String::from_utf8_lossy(&tail[..end]).to_string());
        }
    }

    None
}

fn line_col_at_byte_offset(bytes: &[u8], byte_off: usize) -> (u32, u32) {
    let mut line = 1u32;
    let mut col = 1u32;
    for &b in &bytes[..byte_off.min(bytes.len())] {
        if b == b'\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }
    (line, col)
}

fn byte_offset_at_line_col(bytes: &[u8], target_line: u32, target_col: u32) -> Option<usize> {
    if target_line < 1 || target_col < 1 {
        return None;
    }

    let mut line = 1u32;
    let mut col = 1u32;

    if target_line == 1 && target_col == 1 {
        return Some(0);
    }

    for (idx, &b) in bytes.iter().enumerate() {
        if line == target_line && col == target_col {
            return Some(idx);
        }
        if b == b'\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }

    if line == target_line && col == target_col {
        return Some(bytes.len());
    }

    None
}

fn validate_file_errors(path: &Path, format: InputFormat, bytes: Arc<Vec<u8>>) -> Vec<ActualError> {
    let rules = pack_html_conformance()
        .merge(pack_aria())
        .merge(pack_i18n())
        .merge(pack_css_checks());
    let source = HtmlEventSource::from_shared_bytes(path.to_string_lossy(), format, bytes).unwrap();
    let report = html_inspector::validate_events(
        source,
        rules,
        Config {
            min_severity: Severity::Error,
            ..Config::default()
        },
    )
    .unwrap();

    let mut errors: Vec<ActualError> = report
        .messages
        .into_iter()
        .map(|m| ActualError {
            code: m.code,
            span: m.span.expect("all error messages should have a span"),
        })
        .collect();

    errors.sort_by(|a, b| {
        (
            a.code.as_str(),
            a.span.byte_start,
            a.span.byte_end,
            a.span.line,
            a.span.col,
        )
            .cmp(&(
                b.code.as_str(),
                b.span.byte_start,
                b.span.byte_end,
                b.span.line,
                b.span.col,
            ))
    });

    errors
}

fn read_expected_errors(path: &Path) -> Vec<ExpectedError> {
    let expected_json = std::fs::read_to_string(path).unwrap();
    let entries: Vec<ExpectedEntry> = serde_json::from_str(&expected_json).unwrap();
    let mut expected: Vec<ExpectedError> = entries
        .into_iter()
        .map(|entry| match entry {
            ExpectedEntry::Detailed(v) => v,
            ExpectedEntry::CodeOnly(code) => panic!(
                "expected entries must be objects with code/line/col (got string {code:?} in {})",
                path.display()
            ),
        })
        .collect();

    expected
        .sort_by(|a, b| (a.code.as_str(), a.line, a.col).cmp(&(b.code.as_str(), b.line, b.col)));
    expected
}

fn write_expected_errors(path: &Path, bytes: &[u8], actual: &[ActualError]) {
    let expected: Vec<ExpectedError> = actual
        .iter()
        .map(|a| ExpectedError {
            code: a.code.clone(),
            line: a.span.line,
            col: a.span.col,
            at: suggested_expected_at(bytes, a.span.byte_start),
            contains: None,
        })
        .collect();
    let mut json = serde_json::to_string_pretty(&expected).unwrap();
    json.push('\n');
    std::fs::write(path, json).unwrap();
}

#[test]
fn invalid_html_fixtures_match_expected_errors_and_spans() {
    let dir = fixture_dir();
    let mut fixtures: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap()
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|p| p.is_file())
        .filter(|p| {
            matches!(
                p.extension().and_then(|e| e.to_str()),
                Some("html" | "xhtml")
            )
        })
        .collect();
    fixtures.sort();

    assert!(
        !fixtures.is_empty(),
        "no fixtures found in {}",
        dir.display()
    );

    for fixture in fixtures {
        let format = match fixture.extension().and_then(|e| e.to_str()) {
            Some("xhtml") => InputFormat::Xhtml,
            _ => InputFormat::Html,
        };
        let expected_path = expected_path_for_fixture(&fixture);

        let bytes = Arc::new(std::fs::read(&fixture).unwrap());

        let actual = validate_file_errors(&fixture, format, bytes.clone());
        if !expected_path.exists() {
            write_expected_errors(&expected_path, bytes.as_ref(), &actual);
            panic!(
                "wrote missing expected results to {} (please review/commit and re-run tests)",
                expected_path.display()
            );
        }
        let expected = read_expected_errors(&expected_path);

        assert_eq!(
            actual.len(),
            expected.len(),
            "unexpected number of errors for {}",
            fixture.display()
        );

        for (actual, expected) in actual.iter().zip(expected.iter()) {
            assert_eq!(
                actual.code,
                expected.code,
                "unexpected error code for {}",
                fixture.display()
            );

            assert_eq!(
                actual.span.line,
                expected.line,
                "unexpected error line for {} ({})",
                fixture.display(),
                actual.code
            );
            assert_eq!(
                actual.span.col,
                expected.col,
                "unexpected error column for {} ({})",
                fixture.display(),
                actual.code
            );

            let offset = byte_offset_at_line_col(bytes.as_ref(), actual.span.line, actual.span.col)
                .unwrap_or_else(|| {
                    panic!(
                        "could not resolve line/col {}:{} in {}",
                        actual.span.line,
                        actual.span.col,
                        fixture.display()
                    )
                });
            assert_eq!(
                offset,
                actual.span.byte_start,
                "span byte_start did not match computed line/col offset for {} ({})",
                fixture.display(),
                actual.code
            );

            let (computed_line, computed_col) =
                line_col_at_byte_offset(bytes.as_ref(), actual.span.byte_start);
            assert_eq!(
                (computed_line, computed_col),
                (actual.span.line, actual.span.col),
                "span line/col did not match computed values for {} ({})",
                fixture.display(),
                actual.code
            );

            if let Some(at) = expected.at.as_deref() {
                assert!(
                    bytes
                        .as_ref()
                        .get(actual.span.byte_start..)
                        .is_some_and(|tail| tail.starts_with(at.as_bytes())),
                    "expected source at {}:{} to start with {at:?} for {} ({})",
                    actual.span.line,
                    actual.span.col,
                    fixture.display(),
                    actual.code
                );
            }

            if let Some(contains) = expected.contains.as_deref() {
                let end = actual.span.byte_end.min(bytes.len());
                let slice = &bytes.as_ref()[actual.span.byte_start..end];
                let slice = String::from_utf8_lossy(slice);
                assert!(
                    slice.contains(contains),
                    "expected source at {}:{} to contain {contains:?} for {} ({})",
                    actual.span.line,
                    actual.span.col,
                    fixture.display(),
                    actual.code
                );
            }
        }
    }
}
