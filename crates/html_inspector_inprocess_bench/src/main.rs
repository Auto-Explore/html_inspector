use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};

use html_inspector::{Config, InputFormat, MessageOrder, Report};
use html_inspector_html::HtmlEventSource;
use html_inspector_rules_aria::pack_aria;
use html_inspector_rules_css::pack_css_checks;
use html_inspector_rules_html::pack_html_conformance;
use html_inspector_rules_i18n::pack_i18n;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("workspace structure changed")
        .to_path_buf()
}

fn collect_html_files(dir: &Path, recursive: bool) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let Ok(rd) = std::fs::read_dir(dir) else {
        return out;
    };
    for e in rd.flatten() {
        let Ok(ft) = e.file_type() else { continue };
        let p = e.path();
        if ft.is_dir() {
            if recursive {
                out.extend(collect_html_files(&p, true));
            }
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
        if matches!(ext.as_str(), "html" | "htm" | "xhtml") {
            out.push(p);
        }
    }
    out.sort();
    out
}

fn arg_value(args: &[String], name: &str) -> Option<String> {
    args.iter()
        .position(|a| a == name)
        .and_then(|i| args.get(i + 1))
        .cloned()
}

fn arg_flag(args: &[String], name: &str) -> bool {
    args.iter().any(|a| a == name)
}

fn parse_message_order(s: &str) -> Option<MessageOrder> {
    match s {
        "emit" => Some(MessageOrder::Emit),
        "span" => Some(MessageOrder::BySpan),
        "vnu" => Some(MessageOrder::Vnu),
        _ => None,
    }
}

#[derive(Clone)]
struct Doc {
    source_name: String,
    format: InputFormat,
    bytes: Arc<Vec<u8>>,
    base_uri: Option<String>,
}

fn detect_format(p: &Path) -> InputFormat {
    match p.extension().and_then(|s| s.to_str()) {
        Some(ext)
            if ext.eq_ignore_ascii_case("xhtml")
                || ext.eq_ignore_ascii_case("xht")
                || ext.eq_ignore_ascii_case("xml") =>
        {
            InputFormat::Xhtml
        }
        _ => InputFormat::Html,
    }
}

type RunOnceTotals = (usize, usize, usize);
type RunOnceOutput = (Duration, usize, RunOnceTotals);

fn run_once(docs: &[Doc], config: &Config) -> Result<RunOnceOutput, String> {
    let start = Instant::now();
    let mut total_msgs = 0usize;
    let mut total_errors = 0usize;
    let mut total_warnings = 0usize;
    let mut total_infos = 0usize;

    for doc in docs {
        let source = HtmlEventSource::from_shared_bytes(
            doc.source_name.clone(),
            doc.format,
            doc.bytes.clone(),
        )
        .map_err(|e| e.to_string())?;
        let rules = pack_html_conformance()
            .merge(pack_aria())
            .merge(pack_i18n())
            .merge(pack_css_checks());
        let mut config = config.clone();
        config.base_uri = doc.base_uri.clone();

        let report: Report =
            html_inspector::validate_events(source, rules, config).map_err(|e| e.to_string())?;
        total_msgs += report.messages.len();
        let (errors, warnings, infos) = report.counts();
        total_errors += errors;
        total_warnings += warnings;
        total_infos += infos;
    }

    Ok((
        start.elapsed(),
        total_msgs,
        (total_errors, total_warnings, total_infos),
    ))
}

#[cfg(test)]
mod tests {
    use super::detect_format;
    use html_inspector::InputFormat;
    use std::path::Path;

    #[test]
    fn detect_format_is_ascii_case_insensitive_without_allocating() {
        assert_eq!(detect_format(Path::new("index.html")), InputFormat::Html);
        assert_eq!(detect_format(Path::new("index.XHTML")), InputFormat::Xhtml);
        assert_eq!(detect_format(Path::new("index.xht")), InputFormat::Xhtml);
        assert_eq!(detect_format(Path::new("index.XML")), InputFormat::Xhtml);
        assert_eq!(detect_format(Path::new("index")), InputFormat::Html);
    }
}

fn stats(v: &[Duration]) -> Option<(Duration, Duration, Duration)> {
    if v.is_empty() {
        return None;
    }
    let mut v = v.to_vec();
    v.sort();
    let min = v[0];
    let med = v[v.len() / 2];
    let sum: Duration = v.iter().copied().fold(Duration::from_secs(0), |a, b| a + b);
    let mean = sum / (v.len() as u32);
    Some((min, med, mean))
}

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if arg_flag(&args, "--help") {
        eprintln!(
            "usage: html_inspector_inprocess_bench [--iterations N] [--warmup N] [--html-dir PATH] [--recursive] [--limit-files N]\n\
             \n\
             Runs the Rust validator fully in-process over a corpus of HTML/XHTML files.\n\
             This avoids the HTTP server + client overhead and exercises all scanners/checks.\n\
             \n\
             Options:\n\
             - --also-check-css / --no-css                 (default: on)\n\
             - --message-order emit|span|vnu                (default: vnu)\n\
             - --iterations N                               (default: 10)\n\
             - --warmup N                                   (default: 1)\n\
             - --limit-files N                              (default: all)\n\
             - --html-dir PATH                              (default: tests/html)\n\
             - --recursive                                  (default: on)"
        );
        return Ok(());
    }

    let iterations: usize = arg_value(&args, "--iterations")
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);
    let warmup: usize = arg_value(&args, "--warmup")
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);
    let limit_files: Option<usize> = arg_value(&args, "--limit-files").and_then(|s| s.parse().ok());
    let mut also_check_css = true;
    let message_order_arg = arg_value(&args, "--message-order");
    let message_order = message_order_arg.as_deref().unwrap_or("vnu");
    let message_order = parse_message_order(message_order).ok_or_else(|| {
        format!(
            "unsupported --message-order {}; use emit|span|vnu",
            message_order
        )
    })?;

    if arg_flag(&args, "--no-css") {
        also_check_css = false;
    }
    if arg_flag(&args, "--also-check-css") {
        also_check_css = true;
    }

    let root = arg_value(&args, "--html-dir")
        .map(PathBuf::from)
        .unwrap_or_else(|| repo_root().join("tests").join("html"));
    let recursive = !arg_flag(&args, "--no-recursive");

    let mut files = collect_html_files(&root, recursive);
    if let Some(n) = limit_files {
        files.truncate(n);
    }
    if files.is_empty() {
        return Err(format!("no html files found in {}", root.display()));
    }

    let mut docs: Vec<Doc> = Vec::with_capacity(files.len());
    let mut total_bytes = 0usize;
    for p in &files {
        let bytes = std::fs::read(p).unwrap_or_default();
        total_bytes += bytes.len();
        let base_uri = std::fs::canonicalize(p)
            .ok()
            .map(|abs| format!("file://{}", abs.to_string_lossy()));
        docs.push(Doc {
            source_name: p.to_string_lossy().to_string(),
            format: detect_format(p),
            bytes: Arc::new(bytes),
            base_uri,
        });
    }

    let config = Config {
        message_order,
        also_check_css,
        ..Config::default()
    };

    println!("corpus: files={} bytes={}", files.len(), total_bytes);
    println!(
        "config: message_order={:?} also_check_css={}",
        message_order, also_check_css
    );

    for _ in 0..warmup {
        let _ = run_once(&docs, &config)?;
    }

    let mut times: Vec<Duration> = Vec::new();
    let mut last_totals = None;
    for _ in 0..iterations {
        let (d, msgs, counts) = run_once(&docs, &config)?;
        times.push(d);
        last_totals = Some((msgs, counts));
    }

    if let Some((min, med, mean)) = stats(&times) {
        println!("rust: min={:?} med={:?} mean={:?}", min, med, mean);
    }
    if let Some((msgs, (e, w, i))) = last_totals {
        println!(
            "last_run: messages={} errors={} warnings={} infos={}",
            msgs, e, w, i
        );
    }

    Ok(())
}

fn main() -> std::process::ExitCode {
    match run() {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            std::process::ExitCode::from(2)
        }
    }
}
