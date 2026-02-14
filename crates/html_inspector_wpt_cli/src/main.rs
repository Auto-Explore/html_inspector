use std::env;
use std::fs;
use std::io::Write;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

use html_inspector_core::{
    Category, Config, InputFormat, Message, MessageOrder, Report, Severity, SeverityProfile,
    ValidatorError,
};
use html_inspector_html::HtmlEventSource;
use html_inspector_rules_aria::pack_aria;
use html_inspector_rules_css::pack_css_checks;
use html_inspector_rules_html::pack_html_conformance;
use html_inspector_rules_i18n::pack_i18n;
use rustc_hash::FxHashSet;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

pub const WPT_HTML_RESULTS_FORMAT_VERSION: u32 = 1;
pub const WPT_HTML_RESULTS_META_FILE: &str = "_meta.md";
pub const WPT_HTML_RESULTS_VALIDATED_HTML_MAX_BYTES: usize = 16 * 1024;

#[derive(Debug, Error)]
pub enum SuiteError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("{0}")]
    InvalidManifest(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WptMinSeverity {
    Error,
    Warning,
    Info,
}

impl WptMinSeverity {
    fn parse_cli(v: &str) -> Result<Self, SuiteError> {
        match v {
            "error" => Ok(Self::Error),
            "warning" => Ok(Self::Warning),
            "info" => Ok(Self::Info),
            other => Err(SuiteError::InvalidManifest(format!(
                "invalid value for --min-severity: {other} (expected: error|warning|info)"
            ))),
        }
    }

    fn rank(self) -> u8 {
        match self {
            Self::Info => 0,
            Self::Warning => 1,
            Self::Error => 2,
        }
    }

    fn as_core(self) -> Severity {
        match self {
            Self::Error => Severity::Error,
            Self::Warning => Severity::Warning,
            Self::Info => Severity::Info,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WptMessageOrder {
    Emit,
    BySpan,
    Vnu,
}

impl WptMessageOrder {
    fn parse_cli(v: &str) -> Result<Self, SuiteError> {
        match v {
            "emit" => Ok(Self::Emit),
            "by-span" => Ok(Self::BySpan),
            "vnu" => Ok(Self::Vnu),
            other => Err(SuiteError::InvalidManifest(format!(
                "invalid value for --message-order: {other} (expected: emit|by-span|vnu)"
            ))),
        }
    }

    fn as_core(self) -> MessageOrder {
        match self {
            Self::Emit => MessageOrder::Emit,
            Self::BySpan => MessageOrder::BySpan,
            Self::Vnu => MessageOrder::Vnu,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WptSeverityProfile {
    Conformance,
    Risk,
}

impl WptSeverityProfile {
    fn parse_cli(v: &str) -> Result<Self, SuiteError> {
        match v {
            "conformance" => Ok(Self::Conformance),
            "risk" => Ok(Self::Risk),
            other => Err(SuiteError::InvalidManifest(format!(
                "invalid value for --severity-profile: {other} (expected: conformance|risk)"
            ))),
        }
    }

    fn as_core(self) -> SeverityProfile {
        match self {
            Self::Conformance => SeverityProfile::Conformance,
            Self::Risk => SeverityProfile::Risk,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WptHtmlResultsConfig {
    pub also_check_css: bool,
    pub ignore_missing_lang: bool,
    pub message_order: WptMessageOrder,
    pub severity_profile: WptSeverityProfile,
    pub min_severity: WptMinSeverity,
}

impl Default for WptHtmlResultsConfig {
    fn default() -> Self {
        Self {
            also_check_css: false,
            ignore_missing_lang: false,
            message_order: WptMessageOrder::Emit,
            severity_profile: WptSeverityProfile::Risk,
            min_severity: WptMinSeverity::Warning,
        }
    }
}

impl WptHtmlResultsConfig {
    fn as_core(&self) -> Config {
        Config {
            ignore_missing_lang: self.ignore_missing_lang,
            message_order: self.message_order.as_core(),
            also_check_css: self.also_check_css,
            severity_profile: self.severity_profile.as_core(),
            min_severity: self.min_severity.as_core(),
            ..Config::default()
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct WptHtmlResultsTotals {
    pub files: usize,
    pub errors: usize,
    pub warnings: usize,
    pub infos: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WptHtmlResultsMeta {
    pub format_version: u32,
    pub wpt_commit: String,
    pub config: WptHtmlResultsConfig,
    #[serde(default)]
    pub totals: WptHtmlResultsTotals,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WptHtmlFileResults {
    pub format_version: u32,
    pub file: String,
    #[serde(default)]
    pub validated_html: Option<String>,
    #[serde(default)]
    pub validated_html_truncated: bool,
    pub report: Value,
}

#[derive(Clone, Debug, Default)]
pub struct WptHtmlCheckOptions {
    pub id_contains: Option<String>,
    pub max_failures: Option<usize>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WptHtmlCheckSummary {
    pub files_total: usize,
    pub files_matched: usize,
    pub files_failed: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WptHtmlFailureKind {
    MissingResultsFile,
    UnexpectedResultsFile,
    InvalidResultsFile { message: String },
    ReportMismatch { expected: Value, actual: Value },
}

#[derive(Clone, Debug, PartialEq)]
pub struct WptHtmlFailure {
    pub id: String,
    pub file: String,
    pub kind: WptHtmlFailureKind,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WptHtmlWriteSummary {
    pub files_written: usize,
}

pub fn workspace_root() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for dir in manifest_dir.ancestors() {
        let cargo_toml = dir.join("Cargo.toml");
        if !cargo_toml.exists() {
            continue;
        }
        let Ok(s) = fs::read_to_string(&cargo_toml) else {
            continue;
        };
        if s.contains("[workspace]") {
            return dir.to_path_buf();
        }
    }
    panic!(
        "workspace root not found from CARGO_MANIFEST_DIR={}",
        manifest_dir.display()
    );
}

pub fn git_head_commit(dir: &Path) -> Result<String, SuiteError> {
    let output = Command::new("git")
        .arg("-C")
        .arg(dir)
        .arg("rev-parse")
        .arg("HEAD")
        .output()?;
    if !output.status.success() {
        return Err(SuiteError::InvalidManifest(format!(
            "git rev-parse HEAD failed for {}: {}",
            dir.display(),
            String::from_utf8_lossy(&output.stderr).trim()
        )));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

pub fn wpt_html_results_meta_path(results_root: &Path) -> PathBuf {
    results_root.join(WPT_HTML_RESULTS_META_FILE)
}

pub fn wpt_html_results_file_path(
    results_root: &Path,
    wpt_file: &str,
) -> Result<PathBuf, SuiteError> {
    let rel = safe_rel_path_from_slash(wpt_file)?;
    let mut out = results_root.to_path_buf();
    out.push(rel);

    let Some(file_name) = out.file_name().map(|s| s.to_string_lossy()) else {
        return Err(SuiteError::InvalidManifest(format!(
            "invalid wpt file path: {wpt_file}"
        )));
    };
    out.set_file_name(format!("{file_name}.md"));
    Ok(out)
}

fn resolve_wpt_repo_root(wpt_root: &Path) -> Result<PathBuf, SuiteError> {
    if wpt_root.join("html").is_dir() {
        return Ok(wpt_root.to_path_buf());
    }

    if wpt_root.is_dir()
        && wpt_root
            .file_name()
            .is_some_and(|s| s == std::ffi::OsStr::new("html"))
        && let Some(parent) = wpt_root.parent() {
            return Ok(parent.to_path_buf());
        }

    if !wpt_root.is_dir() {
        return Err(SuiteError::InvalidManifest(format!(
            "WPT root not found: {}",
            wpt_root.display()
        )));
    }

    Err(SuiteError::InvalidManifest(format!(
        "WPT html root not found: {}",
        wpt_root.join("html").display()
    )))
}

fn collect_wpt_html_files(
    wpt_root: &Path,
    out: &mut Vec<(String, PathBuf)>,
) -> Result<(), SuiteError> {
    let wpt_html_root = wpt_root.join("html");
    if !wpt_html_root.is_dir() {
        return Err(SuiteError::InvalidManifest(format!(
            "WPT html root not found: {}",
            wpt_html_root.display()
        )));
    }
    collect_html_files_rec(&wpt_html_root, out, wpt_root)
}

pub fn write_wpt_html_results_tree(
    wpt_root: &Path,
    wpt_commit: &str,
    results_root: &Path,
    cfg: &WptHtmlResultsConfig,
) -> Result<WptHtmlWriteSummary, SuiteError> {
    if !wpt_root.is_dir() {
        return Err(SuiteError::InvalidManifest(format!(
            "WPT root not found: {}",
            wpt_root.display()
        )));
    }

    // Track existing result files so we can remove stale ones after writing.
    let mut existing_results_files: FxHashSet<PathBuf> = FxHashSet::default();
    if results_root.is_dir() {
        let mut md_files: Vec<PathBuf> = Vec::new();
        collect_markdown_files_rec(results_root, &mut md_files)?;
        existing_results_files.reserve(md_files.len());
        existing_results_files.extend(md_files);
    }
    let meta_path = wpt_html_results_meta_path(results_root);

    let mut files: Vec<(String, PathBuf)> = Vec::new();
    collect_wpt_html_files(wpt_root, &mut files)?;
    files.sort_by(|a, b| a.0.cmp(&b.0));
    let files_len = files.len();

    let mut totals = WptHtmlResultsTotals::default();
    let mut summary = WptHtmlWriteSummary::default();

    let mut written_results_files: FxHashSet<PathBuf> =
        FxHashSet::with_capacity_and_hasher(files_len, Default::default());

    for (rel, path) in files {
        let bytes = fs::read(&path)?;
        let (validated_html, validated_html_truncated) =
            html_excerpt_bytes(&bytes, WPT_HTML_RESULTS_VALIDATED_HTML_MAX_BYTES);
        let report = validate_wpt_html_bytes(&rel, bytes, cfg);
        let (errors, warnings, infos) = report.counts();

        let results = WptHtmlFileResults {
            format_version: WPT_HTML_RESULTS_FORMAT_VERSION,
            file: rel.clone(),
            validated_html: Some(validated_html),
            validated_html_truncated,
            report: serde_json::to_value(&report)?,
        };
        let out_path = wpt_html_results_file_path(results_root, &rel)?;
        write_wpt_html_file_results_atomic(&out_path, &results)?;
        written_results_files.insert(out_path);

        totals.files += 1;
        totals.errors += errors;
        totals.warnings += warnings;
        totals.infos += infos;

        summary.files_written += 1;
    }

    for path in existing_results_files {
        if path == meta_path || written_results_files.contains(&path) {
            continue;
        }
        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            fs::remove_file(&path)?;
        }
    }

    let meta = WptHtmlResultsMeta {
        format_version: WPT_HTML_RESULTS_FORMAT_VERSION,
        wpt_commit: wpt_commit.to_string(),
        config: cfg.clone(),
        totals,
    };
    write_wpt_html_results_meta_atomic(results_root, &meta)?;

    Ok(summary)
}

pub fn check_wpt_html_results_tree(
    wpt_root: &Path,
    wpt_commit: &str,
    results_root: &Path,
    cfg: &WptHtmlResultsConfig,
    options: WptHtmlCheckOptions,
) -> Result<(WptHtmlCheckSummary, Vec<WptHtmlFailure>), SuiteError> {
    let meta = load_wpt_html_results_meta(results_root)?;
    if meta.format_version != WPT_HTML_RESULTS_FORMAT_VERSION {
        return Err(SuiteError::InvalidManifest(format!(
            "unsupported wpt html results format_version={}",
            meta.format_version
        )));
    }
    if meta.wpt_commit != wpt_commit {
        return Err(SuiteError::InvalidManifest(format!(
            "wpt commit mismatch: results meta has {}, but {} is {}",
            meta.wpt_commit,
            wpt_root.display(),
            wpt_commit
        )));
    }
    if !wpt_results_config_compatible_for_check(&meta.config, cfg) {
        return Err(SuiteError::InvalidManifest(format!(
            "validator config mismatch: results meta has {:?}, but current run is {:?} (note: check accepts equal config or stricter --min-severity)",
            meta.config, cfg
        )));
    }

    if !wpt_root.is_dir() {
        return Err(SuiteError::InvalidManifest(format!(
            "WPT root not found: {}",
            wpt_root.display()
        )));
    }

    let mut files: Vec<(String, PathBuf)> = Vec::new();
    collect_wpt_html_files(wpt_root, &mut files)?;
    files.sort_by(|a, b| a.0.cmp(&b.0));
    let files_len = files.len();

    let max_failures = options
        .max_failures
        .filter(|&n| n != 0)
        .unwrap_or(usize::MAX);

    let mut expected_results: FxHashSet<PathBuf> =
        FxHashSet::with_capacity_and_hasher(files_len, Default::default());
    let mut summary = WptHtmlCheckSummary::default();
    let mut failures: Vec<WptHtmlFailure> = Vec::with_capacity(files_len.min(max_failures));

    for (rel, path) in files {
        if failures.len() >= max_failures {
            break;
        }
        if options
            .id_contains
            .as_deref()
            .is_some_and(|needle| !rel.contains(needle))
        {
            continue;
        }

        summary.files_total += 1;

        let results_path = wpt_html_results_file_path(results_root, &rel)?;
        if options.id_contains.is_none() {
            expected_results.insert(results_path.clone());
        }

        let mut file_failed = false;

        if !results_path.exists() {
            file_failed = true;
            push_failure(
                &mut failures,
                max_failures,
                WptHtmlFailure {
                    id: rel.clone(),
                    file: rel.clone(),
                    kind: WptHtmlFailureKind::MissingResultsFile,
                },
            );
        } else {
            let stored = match load_wpt_html_file_results(&results_path) {
                Ok(v) => v,
                Err(e) => {
                    file_failed = true;
                    push_failure(
                        &mut failures,
                        max_failures,
                        WptHtmlFailure {
                            id: rel.clone(),
                            file: rel.clone(),
                            kind: WptHtmlFailureKind::InvalidResultsFile {
                                message: e.to_string(),
                            },
                        },
                    );
                    finalize_file_summary(&mut summary, file_failed);
                    continue;
                }
            };

            if stored.format_version != WPT_HTML_RESULTS_FORMAT_VERSION {
                file_failed = true;
                push_failure(
                    &mut failures,
                    max_failures,
                    WptHtmlFailure {
                        id: rel.clone(),
                        file: rel.clone(),
                        kind: WptHtmlFailureKind::InvalidResultsFile {
                            message: format!(
                                "unsupported format_version={}",
                                stored.format_version
                            ),
                        },
                    },
                );
            } else if stored.file != rel {
                file_failed = true;
                push_failure(
                    &mut failures,
                    max_failures,
                    WptHtmlFailure {
                        id: rel.clone(),
                        file: rel.clone(),
                        kind: WptHtmlFailureKind::InvalidResultsFile {
                            message: format!("stored file mismatch: {}", stored.file),
                        },
                    },
                );
            } else {
                let bytes = fs::read(&path)?;
                let report = validate_wpt_html_bytes(&rel, bytes, cfg);
                let actual_report = serde_json::to_value(&report)?;

                let expected = filter_report_by_min_severity(&stored.report, cfg.min_severity);
                let actual = filter_report_by_min_severity(&actual_report, cfg.min_severity);

                if actual != expected {
                    file_failed = true;
                    push_failure(
                        &mut failures,
                        max_failures,
                        WptHtmlFailure {
                            id: rel.clone(),
                            file: rel.clone(),
                            kind: WptHtmlFailureKind::ReportMismatch { expected, actual },
                        },
                    );
                }
            }
        }

        finalize_file_summary(&mut summary, file_failed);
    }

    if options.id_contains.is_none() && results_root.is_dir() && failures.len() < max_failures {
        let mut result_files: Vec<PathBuf> = Vec::new();
        collect_markdown_files_rec(results_root, &mut result_files)?;
        result_files.sort_by(|a, b| a.to_string_lossy().cmp(&b.to_string_lossy()));

        for path in result_files {
            if failures.len() >= max_failures {
                break;
            }
            if path.file_name().and_then(|s| s.to_str()) == Some(WPT_HTML_RESULTS_META_FILE) {
                continue;
            }
            if !expected_results.contains(&path) {
                let rel = path
                    .strip_prefix(results_root)
                    .unwrap_or(&path)
                    .to_path_buf();
                let rel_str = path_to_slash(&rel);
                push_failure(
                    &mut failures,
                    max_failures,
                    WptHtmlFailure {
                        id: rel_str.clone(),
                        file: rel_str,
                        kind: WptHtmlFailureKind::UnexpectedResultsFile,
                    },
                );
            }
        }
    }

    Ok((summary, failures))
}

fn finalize_file_summary(summary: &mut WptHtmlCheckSummary, file_failed: bool) {
    if file_failed {
        summary.files_failed += 1;
    } else {
        summary.files_matched += 1;
    }
}

fn push_failure(out: &mut Vec<WptHtmlFailure>, max: usize, f: WptHtmlFailure) {
    if out.len() < max {
        out.push(f);
    }
}

fn input_format_for_rel_path(path: &str) -> InputFormat {
    let Some((_, ext)) = path.rsplit_once('.') else {
        return InputFormat::Html;
    };
    match ext.to_ascii_lowercase().as_str() {
        "xhtml" | "xht" | "xml" | "svg" => InputFormat::Xhtml,
        _ => InputFormat::Html,
    }
}

fn validate_wpt_html_bytes(rel: &str, bytes: Vec<u8>, cfg: &WptHtmlResultsConfig) -> Report {
    let format = input_format_for_rel_path(rel);
    let rules = pack_html_conformance()
        .merge(pack_aria())
        .merge(pack_i18n())
        .merge(pack_css_checks());

    let source = match HtmlEventSource::from_bytes(rel, format, bytes) {
        Ok(v) => v,
        Err(e) => return report_from_validator_error(rel, e),
    };

    report_from_validator_result(
        rel,
        html_inspector_core::validate_events(source, rules, cfg.as_core()),
    )
}

fn report_from_validator_error(source_name: &str, error: ValidatorError) -> Report {
    let (code, message) = match error {
        ValidatorError::InvalidInput(msg) => ("internal.invalid_input", msg),
        ValidatorError::Io(e) => ("internal.io_error", e.to_string()),
    };
    Report {
        source_name: source_name.to_string(),
        messages: vec![Message::new(
            code,
            Severity::Error,
            Category::Internal,
            message,
            None,
        )],
    }
}

fn report_from_validator_result(
    source_name: &str,
    result: Result<Report, ValidatorError>,
) -> Report {
    match result {
        Ok(report) => report,
        Err(e) => report_from_validator_error(source_name, e),
    }
}

fn write_wpt_html_results_meta_atomic(
    results_root: &Path,
    meta: &WptHtmlResultsMeta,
) -> Result<(), SuiteError> {
    let path = wpt_html_results_meta_path(results_root);
    let md = render_results_meta_markdown(meta)?;
    write_markdown_atomic(&path, &md)
}

fn write_wpt_html_file_results_atomic(
    path: &Path,
    results: &WptHtmlFileResults,
) -> Result<(), SuiteError> {
    let md = render_file_results_markdown(results)?;
    write_markdown_atomic(path, &md)
}

fn load_wpt_html_results_meta(results_root: &Path) -> Result<WptHtmlResultsMeta, SuiteError> {
    let path = wpt_html_results_meta_path(results_root);
    let s = fs::read_to_string(&path)?;
    parse_results_meta_markdown(&path, &s)
}

fn load_wpt_html_file_results(path: &Path) -> Result<WptHtmlFileResults, SuiteError> {
    let s = fs::read_to_string(path)?;
    parse_file_results_markdown(path, &s)
}

fn write_markdown_atomic(path: &Path, markdown: &str) -> Result<(), SuiteError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let tmp_path = tmp_path_for(path);
    let file = fs::File::create(&tmp_path)?;
    let mut writer = std::io::BufWriter::new(file);
    writer.write_all(markdown.as_bytes())?;
    writer.flush()?;

    if path.exists() {
        fs::remove_file(path)?;
    }
    fs::rename(&tmp_path, path)?;
    Ok(())
}

fn tmp_path_for(path: &Path) -> PathBuf {
    let file_name = path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();
    path.with_file_name(format!("{file_name}.tmp"))
}

#[derive(Clone, Debug)]
struct FencedCodeBlock {
    info: String,
    content: String,
}

fn parse_markdown_fenced_code_blocks(markdown: &str) -> Result<Vec<FencedCodeBlock>, SuiteError> {
    let mut out: Vec<FencedCodeBlock> = Vec::new();
    let mut open_fence_len: Option<usize> = None;
    let mut open_info = String::new();
    let mut content = String::new();

    for line in markdown.split_inclusive('\n') {
        let trimmed = line.trim_end_matches('\n');
        if let Some(fence_len) = open_fence_len {
            if is_fence_closer(trimmed, fence_len) {
                out.push(FencedCodeBlock {
                    info: std::mem::take(&mut open_info),
                    content: std::mem::take(&mut content),
                });
                open_fence_len = None;
            } else {
                content.push_str(line);
            }
            continue;
        }

        if let Some((fence_len, info)) = parse_fence_opener(trimmed) {
            open_fence_len = Some(fence_len);
            open_info = info;
            content.clear();
        }
    }

    if open_fence_len.is_some() {
        return Err(SuiteError::InvalidManifest(
            "unclosed fenced code block".to_string(),
        ));
    }

    Ok(out)
}

fn parse_fence_opener(line: &str) -> Option<(usize, String)> {
    let bytes = line.as_bytes();
    let mut n = 0usize;
    while n < bytes.len() && bytes[n] == b'`' {
        n += 1;
    }
    if n < 3 {
        return None;
    }
    let info = line[n..].trim().to_string();
    Some((n, info))
}

fn is_fence_closer(line: &str, fence_len: usize) -> bool {
    let bytes = line.as_bytes();
    let mut n = 0usize;
    while n < bytes.len() && bytes[n] == b'`' {
        n += 1;
    }
    n == fence_len && line[n..].trim().is_empty()
}

fn block_lang(info: &str) -> &str {
    info.split_whitespace().next().unwrap_or("")
}

fn render_fenced_code_block(info: &str, content: &str) -> String {
    let fence_len = (max_backtick_run(content) + 1).max(3);
    let fence = "`".repeat(fence_len);

    let mut out = String::new();
    out.push_str(&fence);
    out.push_str(info);
    out.push('\n');
    out.push_str(content);
    if !content.ends_with('\n') {
        out.push('\n');
    }
    out.push_str(&fence);
    out.push('\n');
    out
}

fn max_backtick_run(s: &str) -> usize {
    let mut max_run = 0usize;
    let mut run = 0usize;
    for b in s.bytes() {
        if b == b'`' {
            run += 1;
            max_run = max_run.max(run);
        } else {
            run = 0;
        }
    }
    max_run
}

fn html_excerpt_bytes(bytes: &[u8], max_bytes: usize) -> (String, bool) {
    if bytes.len() <= max_bytes {
        (String::from_utf8_lossy(bytes).into_owned(), false)
    } else {
        (
            String::from_utf8_lossy(&bytes[..max_bytes]).into_owned(),
            true,
        )
    }
}

fn render_results_meta_markdown(meta: &WptHtmlResultsMeta) -> Result<String, SuiteError> {
    let json = serde_json::to_string_pretty(meta)?;
    let mut out = String::new();
    out.push_str("# WPT HTML results\n\n");
    out.push_str("Totals:\n");
    out.push_str(&format!(
        "- files: {}\n- errors: {}\n- warnings: {}\n- infos: {}\n\n",
        meta.totals.files, meta.totals.errors, meta.totals.warnings, meta.totals.infos
    ));
    out.push_str(
        "Machine-readable metadata for `html_inspector_wpt_cli wpt-html` lives in the JSON block below.\n\n",
    );
    out.push_str(&render_fenced_code_block("json", &json));
    Ok(out)
}

fn parse_results_meta_markdown(
    path: &Path,
    markdown: &str,
) -> Result<WptHtmlResultsMeta, SuiteError> {
    let blocks = parse_markdown_fenced_code_blocks(markdown)?;
    let Some(json_block) = blocks
        .iter()
        .find(|b| block_lang(&b.info).eq_ignore_ascii_case("json"))
    else {
        return Err(SuiteError::InvalidManifest(format!(
            "{}: missing ```json meta block",
            path.display()
        )));
    };
    serde_json::from_str(&json_block.content).map_err(SuiteError::from)
}

fn render_file_results_markdown(results: &WptHtmlFileResults) -> Result<String, SuiteError> {
    #[derive(Serialize)]
    struct FileHeader<'a> {
        format_version: u32,
        file: &'a str,
        #[serde(skip_serializing_if = "Option::is_none")]
        validated_html_truncated: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        validated_html_max_bytes: Option<usize>,
    }

    let header = FileHeader {
        format_version: results.format_version,
        file: &results.file,
        validated_html_truncated: results
            .validated_html
            .as_ref()
            .map(|_| results.validated_html_truncated),
        validated_html_max_bytes: results
            .validated_html
            .as_ref()
            .map(|_| WPT_HTML_RESULTS_VALIDATED_HTML_MAX_BYTES),
    };
    let header_json = serde_json::to_string_pretty(&header)?;

    let (errors, warnings, infos) = report_counts(&results.report);
    let mut out = String::new();
    out.push_str("# ");
    out.push_str(&results.file);
    out.push_str("\n\n");
    out.push_str("Counts:\n");
    out.push_str(&format!(
        "- errors: {errors}\n- warnings: {warnings}\n- infos: {infos}\n\n"
    ));
    out.push_str(&render_fenced_code_block("json", &header_json));
    out.push('\n');

    if let Some(validated_html) = results.validated_html.as_deref() {
        out.push_str("Validated HTML:\n");
        out.push_str(&render_fenced_code_block("html", validated_html));
        if results.validated_html_truncated {
            out.push_str(&format!(
                "(validated HTML truncated to first {} bytes)\n",
                WPT_HTML_RESULTS_VALIDATED_HTML_MAX_BYTES
            ));
        }
        out.push('\n');
    }

    let report_json = serde_json::to_string_pretty(&results.report)?;
    out.push_str(&render_fenced_code_block("json", &report_json));
    Ok(out)
}

fn parse_file_results_markdown(
    path: &Path,
    markdown: &str,
) -> Result<WptHtmlFileResults, SuiteError> {
    #[derive(Deserialize)]
    struct FileHeader {
        format_version: u32,
        file: String,
        #[serde(default)]
        validated_html_truncated: Option<bool>,
    }

    let blocks = parse_markdown_fenced_code_blocks(markdown)?;
    if blocks.len() != 2 && blocks.len() != 3 {
        return Err(SuiteError::InvalidManifest(format!(
            "{}: expected 2 or 3 fenced code blocks (header + [validated html] + report), got {}",
            path.display(),
            blocks.len()
        )));
    }

    if !block_lang(&blocks[0].info).eq_ignore_ascii_case("json") {
        return Err(SuiteError::InvalidManifest(format!(
            "{}: first fenced block must be ```json file header",
            path.display()
        )));
    }
    let header: FileHeader = serde_json::from_str(&blocks[0].content)?;

    let (validated_html, report_block) = if blocks.len() == 2 {
        (None, &blocks[1])
    } else if block_lang(&blocks[1].info).eq_ignore_ascii_case("html") {
        (Some(blocks[1].content.clone()), &blocks[2])
    } else {
        return Err(SuiteError::InvalidManifest(format!(
            "{}: second fenced block must be ```html validated html",
            path.display()
        )));
    };

    if !block_lang(&report_block.info).eq_ignore_ascii_case("json") {
        return Err(SuiteError::InvalidManifest(format!(
            "{}: last fenced block must be ```json report",
            path.display()
        )));
    }
    let report: Value = serde_json::from_str(&report_block.content)?;

    Ok(WptHtmlFileResults {
        format_version: header.format_version,
        file: header.file,
        validated_html,
        validated_html_truncated: header.validated_html_truncated.unwrap_or(false),
        report,
    })
}

fn report_counts(value: &Value) -> (usize, usize, usize) {
    let Some(msgs) = value.get("messages").and_then(Value::as_array) else {
        return (0, 0, 0);
    };
    let (mut errors, mut warnings, mut infos) = (0usize, 0usize, 0usize);
    for msg in msgs {
        let Some(sev) = msg.get("severity").and_then(Value::as_str) else {
            continue;
        };
        if sev.eq_ignore_ascii_case("error") {
            errors += 1;
        } else if sev.eq_ignore_ascii_case("warning") {
            warnings += 1;
        } else if sev.eq_ignore_ascii_case("info") {
            infos += 1;
        }
    }
    (errors, warnings, infos)
}

fn wpt_results_config_compatible_for_check(
    results: &WptHtmlResultsConfig,
    requested: &WptHtmlResultsConfig,
) -> bool {
    results.also_check_css == requested.also_check_css
        && results.ignore_missing_lang == requested.ignore_missing_lang
        && results.message_order == requested.message_order
        && results.severity_profile == requested.severity_profile
        // allow checking at a stricter min severity than the stored results
        && results.min_severity.rank() <= requested.min_severity.rank()
}

fn filter_report_by_min_severity(report: &Value, min_severity: WptMinSeverity) -> Value {
    let Value::Object(obj) = report else {
        return report.clone();
    };

    let Some(messages) = obj.get("messages").and_then(Value::as_array) else {
        return report.clone();
    };

    let mut out_obj = obj.clone();
    out_obj.insert(
        "messages".to_string(),
        Value::Array(
            messages
                .iter()
                .filter(|msg| msg_has_severity_at_least(msg, min_severity))
                .cloned()
                .collect(),
        ),
    );
    Value::Object(out_obj)
}

fn msg_has_severity_at_least(msg: &Value, min_severity: WptMinSeverity) -> bool {
    let Some(sev) = msg.get("severity").and_then(Value::as_str) else {
        return true;
    };
    let rank = if sev.eq_ignore_ascii_case("error") {
        WptMinSeverity::Error.rank()
    } else if sev.eq_ignore_ascii_case("warning") {
        WptMinSeverity::Warning.rank()
    } else if sev.eq_ignore_ascii_case("info") {
        WptMinSeverity::Info.rank()
    } else {
        // Unknown severity: keep it so we don't accidentally drop important data.
        return true;
    };
    rank >= min_severity.rank()
}

fn collect_markdown_files_rec(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), SuiteError> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let ft = entry.file_type()?;
        if ft.is_dir() {
            collect_markdown_files_rec(&path, out)?;
            continue;
        }
        if !ft.is_file() {
            continue;
        }
        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            out.push(path);
        }
    }
    Ok(())
}

fn is_html_file(path: &Path) -> bool {
    let Some(ext) = path.extension().and_then(|e| e.to_str()) else {
        return false;
    };
    matches!(
        ext.to_ascii_lowercase().as_str(),
        "html" | "htm" | "xhtml" | "xht"
    )
}

fn collect_html_files_rec(
    dir: &Path,
    out: &mut Vec<(String, PathBuf)>,
    wpt_root: &Path,
) -> Result<(), SuiteError> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let ft = entry.file_type()?;
        if ft.is_dir() {
            if entry.file_name().as_os_str() == std::ffi::OsStr::new(".git") {
                continue;
            }
            collect_html_files_rec(&path, out, wpt_root)?;
            continue;
        }
        if !ft.is_file() || !is_html_file(&path) {
            continue;
        }
        let rel = path.strip_prefix(wpt_root).unwrap_or(&path).to_path_buf();
        out.push((path_to_slash(&rel), path));
    }
    Ok(())
}

fn path_to_slash(path: &Path) -> String {
    let mut out = String::new();
    for c in path.components() {
        let Component::Normal(part) = c else {
            continue;
        };
        if !out.is_empty() {
            out.push('/');
        }
        out.push_str(&part.to_string_lossy());
    }
    out
}

fn safe_rel_path_from_slash(path: &str) -> Result<PathBuf, SuiteError> {
    let rel = Path::new(path);
    let mut out = PathBuf::new();
    for c in rel.components() {
        match c {
            Component::Normal(part) => out.push(part),
            _ => {
                return Err(SuiteError::InvalidManifest(format!(
                    "invalid relative path: {path}"
                )));
            }
        }
    }
    Ok(out)
}

fn run<I, WOut, WErr>(
    mut args: I,
    _stdout: &mut WOut,
    stderr: &mut WErr,
) -> Result<(), Box<dyn std::error::Error>>
where
    I: Iterator<Item = String>,
    WOut: Write,
    WErr: Write,
{
    let cmd = args.next();

    match cmd.as_deref().unwrap_or("help") {
        "wpt-html" => {
            let root = workspace_root();
            let mut wpt_root = root.join("fixtures").join("wpt");
            let mut results_root = root.join("fixtures").join("wpt_html_results");

            let mut cfg = WptHtmlResultsConfig::default();
            let mut write_results = false;
            let mut strict = false;
            let mut max_failures: Option<usize> = None;
            let mut id_contains: Option<String> = None;
            let mut print_failures: usize = 20;

            while let Some(arg) = args.next() {
                match arg.as_str() {
                    "--wpt-root" => {
                        wpt_root = PathBuf::from(args.next().ok_or("missing value for --wpt-root")?)
                    }
                    "--results" => {
                        results_root =
                            PathBuf::from(args.next().ok_or("missing value for --results")?)
                    }
                    "--write" => write_results = true,
                    "--strict" => strict = true,
                    "--also-check-css" => cfg.also_check_css = true,
                    "--ignore-missing-lang" => cfg.ignore_missing_lang = true,
                    "--min-severity" => {
                        cfg.min_severity = WptMinSeverity::parse_cli(
                            &args.next().ok_or("missing value for --min-severity")?,
                        )?
                    }
                    "--message-order" => {
                        cfg.message_order = WptMessageOrder::parse_cli(
                            &args.next().ok_or("missing value for --message-order")?,
                        )?
                    }
                    "--severity-profile" => {
                        cfg.severity_profile = WptSeverityProfile::parse_cli(
                            &args.next().ok_or("missing value for --severity-profile")?,
                        )?
                    }
                    "--max-failures" => {
                        max_failures = Some(
                            args.next()
                                .ok_or("missing value for --max-failures")?
                                .parse()?,
                        )
                    }
                    "--id-contains" => {
                        id_contains = Some(args.next().ok_or("missing value for --id-contains")?)
                    }
                    "--print-failures" => {
                        print_failures = args
                            .next()
                            .ok_or("missing value for --print-failures")?
                            .parse()?
                    }
                    other => return Err(format!("unknown arg: {other}").into()),
                }
            }

            let wpt_root = resolve_wpt_repo_root(&wpt_root)?;
            let wpt_commit = git_head_commit(&wpt_root)?;

            if write_results {
                let summary =
                    write_wpt_html_results_tree(&wpt_root, &wpt_commit, &results_root, &cfg)?;
                writeln!(
                    stderr,
                    "wpt-html wrote {} file(s) under {} (wpt_commit={})",
                    summary.files_written,
                    results_root.display(),
                    wpt_commit
                )?;
                return Ok(());
            }

            let options = WptHtmlCheckOptions {
                id_contains,
                max_failures,
            };
            let (summary, failures) =
                check_wpt_html_results_tree(&wpt_root, &wpt_commit, &results_root, &cfg, options)?;

            writeln!(
                stderr,
                "wpt-html summary: files={} matched={} failed={}",
                summary.files_total, summary.files_matched, summary.files_failed
            )?;
            if !failures.is_empty() {
                writeln!(stderr, "failures (showing up to {print_failures}):")?;
                for f in failures.iter().take(print_failures) {
                    match &f.kind {
                        WptHtmlFailureKind::MissingResultsFile => {
                            writeln!(stderr, "  {}: missing results file", f.id)?;
                        }
                        WptHtmlFailureKind::UnexpectedResultsFile => {
                            writeln!(stderr, "  {}: unexpected results file", f.id)?;
                        }
                        WptHtmlFailureKind::InvalidResultsFile { message } => {
                            writeln!(stderr, "  {}: invalid results file: {}", f.id, message)?;
                        }
                        WptHtmlFailureKind::ReportMismatch { expected, actual } => {
                            let (exp_e, exp_w, exp_i) = report_counts(expected);
                            let (act_e, act_w, act_i) = report_counts(actual);
                            match cfg.min_severity {
                                WptMinSeverity::Error => {
                                    writeln!(
                                        stderr,
                                        "  {}: expected errors={} got errors={}",
                                        f.id, exp_e, act_e
                                    )?;
                                }
                                WptMinSeverity::Warning => {
                                    writeln!(
                                        stderr,
                                        "  {}: expected errors={} warnings={} got errors={} warnings={}",
                                        f.id, exp_e, exp_w, act_e, act_w
                                    )?;
                                }
                                WptMinSeverity::Info => {
                                    writeln!(
                                        stderr,
                                        "  {}: expected errors={} warnings={} infos={} got errors={} warnings={} infos={}",
                                        f.id, exp_e, exp_w, exp_i, act_e, act_w, act_i
                                    )?;
                                }
                            }

                            let html_path = safe_rel_path_from_slash(&f.file)
                                .map(|rel| wpt_root.join(rel))
                                .ok();
                            if let Some(html_path) = html_path {
                                match fs::read(&html_path) {
                                    Ok(bytes) => {
                                        let (html, truncated) = html_excerpt_bytes(
                                            &bytes,
                                            WPT_HTML_RESULTS_VALIDATED_HTML_MAX_BYTES,
                                        );
                                        writeln!(stderr, "  html: {}", html_path.display())?;
                                        write!(
                                            stderr,
                                            "{}",
                                            render_fenced_code_block("html", &html)
                                        )?;
                                        if truncated {
                                            writeln!(
                                                stderr,
                                                "  (html truncated to first {} bytes)",
                                                WPT_HTML_RESULTS_VALIDATED_HTML_MAX_BYTES
                                            )?;
                                        }
                                    }
                                    Err(e) => {
                                        writeln!(
                                            stderr,
                                            "  html: {} (failed to read: {})",
                                            html_path.display(),
                                            e
                                        )?;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if strict && (!failures.is_empty() || summary.files_failed != 0) {
                return Err(format!(
                    "wpt-html had failures: files_failed={}",
                    summary.files_failed
                )
                .into());
            }
            Ok(())
        }
        _ => {
            writeln!(stderr, "usage:")?;
            writeln!(
                stderr,
                "  html_inspector_wpt_cli wpt-html [--wpt-root DIR] [--results DIR] [--write] [--strict] [--also-check-css] [--ignore-missing-lang] [--min-severity error|warning|info] [--message-order emit|by-span|vnu] [--severity-profile conformance|risk] [--max-failures N] [--id-contains STR] [--print-failures N]"
            )?;
            Ok(())
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = std::io::stdout();
    let mut stderr = std::io::stderr();
    run(env::args().skip(1), &mut stdout, &mut stderr)
}

#[cfg(test)]
mod tests {
    use super::{
        collect_wpt_html_files, parse_file_results_markdown, parse_results_meta_markdown,
        render_file_results_markdown, render_results_meta_markdown, safe_rel_path_from_slash,
        WptHtmlFileResults, WptHtmlResultsConfig, WptHtmlResultsMeta, WptHtmlResultsTotals,
        WPT_HTML_RESULTS_FORMAT_VERSION,
    };
    use serde_json::json;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn safe_rel_path_from_slash_rejects_traversal() {
        assert!(safe_rel_path_from_slash("../x").is_err());
        assert!(safe_rel_path_from_slash("/abs").is_err());
    }

    #[test]
    fn results_meta_markdown_round_trips() {
        let meta = WptHtmlResultsMeta {
            format_version: WPT_HTML_RESULTS_FORMAT_VERSION,
            wpt_commit: "abc123".to_string(),
            config: WptHtmlResultsConfig::default(),
            totals: WptHtmlResultsTotals {
                files: 1,
                errors: 2,
                warnings: 3,
                infos: 4,
            },
        };
        let md = render_results_meta_markdown(&meta).unwrap();
        let parsed = parse_results_meta_markdown(std::path::Path::new("meta.md"), &md).unwrap();
        assert_eq!(parsed, meta);
    }

    #[test]
    fn file_results_markdown_round_trips_and_handles_backticks() {
        let results = WptHtmlFileResults {
            format_version: WPT_HTML_RESULTS_FORMAT_VERSION,
            file: "x.html".to_string(),
            validated_html: Some("<!doctype html>\n<p>```</p>\n".to_string()),
            validated_html_truncated: false,
            report: json!({
                "source_name": "x.html",
                "messages": [{
                    "code": "x",
                    "severity": "Error",
                    "category": "Html",
                    "message": "```",
                    "span": null,
                    "notes": [],
                    "order": 0
                }]
            }),
        };

        let md = render_file_results_markdown(&results).unwrap();
        let parsed = parse_file_results_markdown(std::path::Path::new("x.md"), &md).unwrap();
        assert_eq!(parsed, results);
    }

    #[test]
    fn file_results_markdown_parses_legacy_two_block_format() {
        let results = WptHtmlFileResults {
            format_version: WPT_HTML_RESULTS_FORMAT_VERSION,
            file: "x.html".to_string(),
            validated_html: None,
            validated_html_truncated: false,
            report: json!({
                "source_name": "x.html",
                "messages": []
            }),
        };

        let md = render_file_results_markdown(&results).unwrap();
        let parsed = parse_file_results_markdown(std::path::Path::new("x.md"), &md).unwrap();
        assert_eq!(parsed, results);
    }

    #[test]
    fn collect_wpt_html_files_only_scans_html_dir() {
        let uniq = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let tmp = std::env::temp_dir().join(format!(
            "html_inspector_wpt_cli_collect_wpt_html_files_{uniq}_{}",
            std::process::id()
        ));
        std::fs::create_dir_all(tmp.join("html")).unwrap();
        std::fs::create_dir_all(tmp.join("other")).unwrap();
        std::fs::write(tmp.join("html").join("in.html"), "<!doctype html>").unwrap();
        std::fs::write(tmp.join("other").join("out.html"), "<!doctype html>").unwrap();

        let mut files: Vec<(String, std::path::PathBuf)> = Vec::new();
        collect_wpt_html_files(&tmp, &mut files).unwrap();
        files.sort_by(|a, b| a.0.cmp(&b.0));

        assert_eq!(files.len(), 1);
        assert_eq!(files[0].0, "html/in.html");

        std::fs::remove_dir_all(&tmp).unwrap();
    }
}
