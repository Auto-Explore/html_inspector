use std::path::{Path, PathBuf};
use std::process::Command;

use html_inspector::{Config, InputFormat, MessageOrder};
use html_inspector_html::HtmlEventSource;
use html_inspector_rules_aria::pack_aria;
use html_inspector_rules_css::pack_css_checks;
use html_inspector_rules_html::pack_html_conformance;
use html_inspector_rules_i18n::pack_i18n;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use thiserror::Error;

type CompareCounts = (
    FxHashMap<String, Counts>,
    FxHashMap<(String, String), u64>,
    FxHashMap<(String, String), u64>,
);

#[derive(Debug, Error)]
enum CompareError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("java invocation failed: {0}")]
    Java(String),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
struct Counts {
    errors: u64,
    warnings: u64,
    infos: u64,
    css_errors: u64,
}

impl Counts {
    fn add(&mut self, other: Counts) {
        self.errors += other.errors;
        self.warnings += other.warnings;
        self.infos += other.infos;
        self.css_errors += other.css_errors;
    }
}

#[derive(Debug, Deserialize)]
struct VnuJson {
    #[serde(default)]
    messages: Vec<VnuMessage>,
}

#[derive(Debug, Deserialize)]
struct VnuMessage {
    #[serde(rename = "type")]
    ty: String,
    #[serde(rename = "subType")]
    #[serde(default)]
    sub_type: Option<String>,
    #[serde(default)]
    url: Option<String>,
    #[serde(default)]
    message: String,
}

#[derive(Clone, Debug, Serialize)]
struct TopMessage {
    severity: String,
    count: u64,
    message: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let mut root: Option<PathBuf> = None;
    let mut limit: Option<usize> = None;
    let mut vnu_jar: Option<PathBuf> = None;
    let mut out_path: Option<PathBuf> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--root" => {
                root = Some(PathBuf::from(
                    args.next().ok_or("missing value for --root")?,
                ))
            }
            "--limit" => limit = Some(args.next().ok_or("missing value for --limit")?.parse()?),
            "--vnu-jar" => {
                vnu_jar = Some(PathBuf::from(
                    args.next().ok_or("missing value for --vnu-jar")?,
                ))
            }
            "--out" => {
                out_path = Some(PathBuf::from(args.next().ok_or("missing value for --out")?))
            }
            other => return Err(format!("unknown arg: {other}").into()),
        }
    }

    let root = root.ok_or(
        "usage: html_inspector_corpus_compare --root <dir> [--limit N] [--vnu-jar PATH] [--out PATH]",
    )?;
    let root = std::fs::canonicalize(&root)?;

    let vnu_jar = vnu_jar.unwrap_or_else(|| repo_root().join("build").join("dist").join("vnu.jar"));

    let corpus = collect_html_files(&root, limit)?;
    eprintln!("corpus: root={} files={}", root.display(), corpus.len());

    let (vnu_counts, vnu_freq_html, vnu_freq_css) =
        run_vnu_jar_on_dir(&vnu_jar, &root, corpus.len())?;
    eprintln!("vnu.jar: files-with-messages={}", vnu_counts.len());

    let (rust_counts, rust_freq_html, rust_freq_css) = run_rust_on_files(&corpus)?;
    eprintln!("rust: files-validated={}", rust_counts.len());

    let mut mismatches_full = Vec::with_capacity(corpus.len());
    let mut mismatches_html_only = Vec::with_capacity(corpus.len());

    let mut totals_vnu_full = Counts::default();
    let mut totals_rust_full = Counts::default();
    let mut totals_vnu_html_only = Counts::default();
    let mut totals_rust_html_only = Counts::default();

    for p in &corpus {
        let k = p.to_string_lossy().to_string();
        let vnu = vnu_counts.get(&k).copied().unwrap_or_default();
        let rust = rust_counts.get(&k).copied().unwrap_or_default();

        totals_vnu_full.add(vnu);
        totals_rust_full.add(rust);

        let vnu_html_only = Counts {
            errors: vnu.errors.saturating_sub(vnu.css_errors),
            warnings: vnu.warnings,
            infos: vnu.infos,
            css_errors: 0,
        };
        let rust_html_only = Counts {
            errors: rust.errors.saturating_sub(rust.css_errors),
            warnings: rust.warnings,
            infos: rust.infos,
            css_errors: 0,
        };
        totals_vnu_html_only.add(vnu_html_only);
        totals_rust_html_only.add(rust_html_only);

        if vnu != rust {
            mismatches_full.push((k.clone(), vnu, rust));
        }
        if vnu_html_only.errors != rust_html_only.errors
            || vnu_html_only.warnings != rust_html_only.warnings
            || vnu_html_only.infos != rust_html_only.infos
        {
            mismatches_html_only.push((k.clone(), vnu_html_only, rust_html_only));
        }
    }

    let summary = serde_json::json!({
        "root": root.to_string_lossy(),
        "files": corpus.len(),
        "vnu": {
            "full": totals_vnu_full,
            "html_only": totals_vnu_html_only,
            "top_messages_html": top_messages(&vnu_freq_html, 50),
            "top_messages_css": top_messages(&vnu_freq_css, 50),
        },
        "rust": {
            "full": totals_rust_full,
            "html_only": totals_rust_html_only,
            "top_messages_html": top_messages(&rust_freq_html, 50),
            "top_messages_css": top_messages(&rust_freq_css, 50),
        },
        "mismatches": {
            "full": mismatches_full.len(),
            "html_only": mismatches_html_only.len(),
            "examples_full": mismatches_full.iter().take(50).map(|(p, a, b)| serde_json::json!({
                "path": p,
                "vnu": a,
                "rust": b,
            })).collect::<Vec<_>>(),
            "examples_html_only": mismatches_html_only.iter().take(50).map(|(p, a, b)| serde_json::json!({
                "path": p,
                "vnu": a,
                "rust": b,
            })).collect::<Vec<_>>(),
        }
    });

    if let Some(out) = out_path {
        std::fs::write(&out, serde_json::to_vec_pretty(&summary)?)?;
        eprintln!("wrote {}", out.display());
    } else {
        println!("{}", serde_json::to_string_pretty(&summary)?);
    }

    Ok(())
}

fn collect_html_files(root: &Path, limit: Option<usize>) -> Result<Vec<PathBuf>, CompareError> {
    let mut out = limit.map_or_else(Vec::new, Vec::with_capacity);
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir)? {
            let entry = entry?;
            let p = entry.path();
            let ft = entry.file_type()?;
            if ft.is_dir() {
                stack.push(p);
                continue;
            }
            if !ft.is_file() {
                continue;
            }
            let ext = p
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_ascii_lowercase();
            if matches!(ext.as_str(), "html" | "htm" | "xhtml" | "xht") {
                out.push(std::fs::canonicalize(&p).unwrap_or(p));
                if let Some(l) = limit
                    && out.len() >= l
                {
                    return Ok(out);
                }
            }
        }
    }
    out.sort();
    Ok(out)
}

fn run_vnu_jar_on_dir(
    jar: &Path,
    root: &Path,
    estimated_files: usize,
) -> Result<CompareCounts, CompareError> {
    let out = Command::new("java")
        .arg("-jar")
        .arg(jar)
        .arg("--format")
        .arg("json")
        .arg("--stdout")
        .arg("--skip-non-html")
        .arg("--html")
        .arg(root)
        .output()?;

    // vnu.jar uses exit codes for validation findings. Prefer parsing JSON output whenever possible.
    let parsed: VnuJson = serde_json::from_slice(&out.stdout).map_err(|e| {
        let stderr = String::from_utf8_lossy(&out.stderr).to_string();
        CompareError::Java(format!(
            "failed to parse vnu.jar output as JSON (exit={:?}): {e}; stderr={stderr}",
            out.status.code()
        ))
    })?;

    let mut map: FxHashMap<String, Counts> =
        FxHashMap::with_capacity_and_hasher(estimated_files, Default::default());
    let mut freq_html: FxHashMap<(String, String), u64> = FxHashMap::default();
    let mut freq_css: FxHashMap<(String, String), u64> = FxHashMap::default();
    for m in parsed.messages {
        let Some(url) = m.url.as_deref() else {
            continue;
        };
        let Some(path) = url_to_path(url) else {
            continue;
        };
        if !path.starts_with(root) {
            continue;
        }
        let key = path.to_string_lossy().to_string();
        let is_css = m.message.starts_with("CSS:");
        let counts = map.entry(key).or_default();
        let sev = classify_vnu(&m.ty, m.sub_type.as_deref());
        match sev {
            SeverityClass::Error => counts.errors += 1,
            SeverityClass::Warning => counts.warnings += 1,
            SeverityClass::Info => counts.infos += 1,
        }
        if is_css && matches!(sev, SeverityClass::Error) {
            counts.css_errors += 1;
        }
        let sev_str = match sev {
            SeverityClass::Error => "error",
            SeverityClass::Warning => "warning",
            SeverityClass::Info => "info",
        }
        .to_string();
        if is_css {
            *freq_css.entry((sev_str, m.message)).or_default() += 1;
        } else {
            *freq_html.entry((sev_str, m.message)).or_default() += 1;
        }
    }
    Ok((map, freq_html, freq_css))
}

#[derive(Clone, Copy, Debug)]
enum SeverityClass {
    Error,
    Warning,
    Info,
}

fn classify_vnu(ty: &str, sub_type: Option<&str>) -> SeverityClass {
    match ty {
        "error" | "non-document-error" => SeverityClass::Error,
        "info" => match sub_type {
            Some("warning") => SeverityClass::Warning,
            _ => SeverityClass::Info,
        },
        "warning" => SeverityClass::Warning,
        _ => SeverityClass::Info,
    }
}

fn url_to_path(url: &str) -> Option<PathBuf> {
    if let Some(rest) = url.strip_prefix("file:") {
        let mut p = rest;
        // file:/abs/path or file:///abs/path or file://localhost/abs/path
        if p.starts_with("///") {
            p = &p[2..];
        } else if p.starts_with("//localhost/") {
            p = &p["//localhost".len()..];
        }
        // strip any query/fragment just in case
        let p = p.split(['?', '#']).next().unwrap_or(p);
        return Some(PathBuf::from(p));
    }
    None
}

fn run_rust_on_files(files: &[PathBuf]) -> Result<CompareCounts, CompareError> {
    let mut out: FxHashMap<String, Counts> =
        FxHashMap::with_capacity_and_hasher(files.len(), Default::default());
    let mut freq_html: FxHashMap<(String, String), u64> = FxHashMap::default();
    let mut freq_css: FxHashMap<(String, String), u64> = FxHashMap::default();
    for p in files {
        let bytes = std::fs::read(p)?;
        let ext = p
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        let format = if matches!(ext.as_str(), "xhtml" | "xht") {
            InputFormat::Xhtml
        } else {
            InputFormat::Html
        };
        let source_name = p.to_string_lossy().to_string();
        let source =
            HtmlEventSource::from_bytes(source_name.clone(), format, bytes).map_err(|e| {
                CompareError::Java(format!("HtmlEventSource failed for {source_name}: {e}"))
            })?;
        let rules = pack_html_conformance()
            .merge(pack_aria())
            .merge(pack_i18n())
            .merge(pack_css_checks());
        let base_uri = Some(format!("file://{}", source_name));
        let report = html_inspector::validate_events(
            source,
            rules,
            Config {
                message_order: MessageOrder::Vnu,
                also_check_css: true,
                base_uri,
                css_profile: Some("css3svg".to_string()),
                css_medium: Some("all".to_string()),
                css_warning: None,
                ..Config::default()
            },
        )
        .map_err(|e| {
            CompareError::Java(format!("validate_events failed for {source_name}: {e}"))
        })?;

        let mut counts = Counts::default();
        for m in report.messages {
            match m.severity {
                html_inspector::Severity::Error => counts.errors += 1,
                html_inspector::Severity::Warning => counts.warnings += 1,
                html_inspector::Severity::Info => counts.infos += 1,
            }
            if m.code == "html.css.error" {
                counts.css_errors += 1;
            }

            let sev_str = match m.severity {
                html_inspector::Severity::Error => "error",
                html_inspector::Severity::Warning => "warning",
                html_inspector::Severity::Info => "info",
            }
            .to_string();
            if m.code == "html.css.error" {
                *freq_css.entry((sev_str, m.message)).or_default() += 1;
            } else {
                *freq_html.entry((sev_str, m.message)).or_default() += 1;
            }
        }
        out.insert(source_name, counts);
    }
    Ok((out, freq_html, freq_css))
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .expect("workspace structure changed")
        .to_path_buf()
}

fn top_messages(freq: &FxHashMap<(String, String), u64>, limit: usize) -> Vec<TopMessage> {
    let mut v: Vec<TopMessage> = freq
        .iter()
        .map(|((sev, msg), count)| TopMessage {
            severity: sev.clone(),
            count: *count,
            message: msg.clone(),
        })
        .collect();
    v.sort_by(|a, b| {
        b.count
            .cmp(&a.count)
            .then_with(|| a.severity.cmp(&b.severity))
    });
    v.truncate(limit);
    v
}
