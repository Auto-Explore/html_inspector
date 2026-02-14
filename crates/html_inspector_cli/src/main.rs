use std::env;

use html_inspector::{Config, InputFormat, Severity};
use html_inspector_html::HtmlEventSource;
use html_inspector_rules_aria::pack_aria;
use html_inspector_rules_css::pack_css_checks;
use html_inspector_rules_html::pack_html_conformance;
use html_inspector_rules_i18n::pack_i18n;

mod server;
mod util;

fn parse_css_cli_options<I: Iterator<Item = String>>(
    args: &mut I,
) -> Result<(bool, css_inspector::Config), Box<dyn std::error::Error>> {
    let mut allow_network = false;
    // Match vnu.jar defaults (Assertions.java): css3svg, medium=all, warningLevel=-1.
    let mut config = css_inspector::Config {
        profile: Some("css3svg".to_string()),
        medium: Some("all".to_string()),
        warning: Some("-1".to_string()),
    };
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--allow-network" => allow_network = true,
            "--profile" => config.profile = Some(args.next().ok_or("missing value for --profile")?),
            "--medium" => config.medium = Some(args.next().ok_or("missing value for --medium")?),
            "--warning" => config.warning = Some(args.next().ok_or("missing value for --warning")?),
            other => return Err(format!("unknown arg: {other}").into()),
        }
    }
    Ok((allow_network, config))
}

fn validate_css_path_or_uri(
    path_or_uri: &str,
    config: &css_inspector::Config,
    allow_network: bool,
) -> Result<css_inspector::Report, Box<dyn std::error::Error>> {
    let report = if path_or_uri.contains("://") {
        let fetcher = css_inspector::StdFetcher {
            allow_network,
            ..css_inspector::StdFetcher::default()
        };
        css_inspector::validate_css_uri_with_fetcher(path_or_uri, config, &fetcher)?
    } else {
        let css = std::fs::read_to_string(path_or_uri)?;
        css_inspector::validate_css_text(&css, config)?
    };
    Ok(report)
}

fn parse_min_severity(v: &str) -> Result<Severity, Box<dyn std::error::Error>> {
    match v {
        "error" => Ok(Severity::Error),
        "warning" => Ok(Severity::Warning),
        "info" => Ok(Severity::Info),
        other => Err(format!(
            "invalid value for --min-severity: {other} (expected: error|warning|info)"
        )
        .into()),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1).peekable();
    let mut min_severity = Severity::Warning;
    while let Some(arg) = args.peek().cloned() {
        match arg.as_str() {
            "--min-severity" => {
                let _ = args.next();
                let v = args.next().ok_or("missing value for --min-severity")?;
                min_severity = parse_min_severity(&v)?;
            }
            _ if arg.starts_with("--min-severity=") => {
                let _ = args.next();
                let (_, v) = arg.split_once('=').ok_or("invalid --min-severity")?;
                min_severity = parse_min_severity(v)?;
            }
            _ => break,
        }
    }

    let cmd = args.next().unwrap_or_else(|| "help".to_string());

    match cmd.as_str() {
        "check" => {
            let path_or_uri = args
                .next()
                .ok_or("usage: html_inspector_cli check <path-or-uri>")?;
            if util::ends_with_ascii_ci(&path_or_uri, ".css") {
                let (allow_network, config) = parse_css_cli_options(&mut args)?;
                let mut report = validate_css_path_or_uri(&path_or_uri, &config, allow_network)?;
                if matches!(min_severity, Severity::Error) {
                    report
                        .messages
                        .retain(|m| matches!(m.severity, css_inspector::Severity::Error));
                }
                report.errors = report
                    .messages
                    .iter()
                    .filter(|m| matches!(m.severity, css_inspector::Severity::Error))
                    .count();
                report.warnings = report
                    .messages
                    .iter()
                    .filter(|m| matches!(m.severity, css_inspector::Severity::Warning))
                    .count();
                println!("{}", serde_json::to_string_pretty(&report)?);
                Ok(())
            } else {
                let mut also_check_css = false;
                let mut csp_header: Option<String> = None;
                let format = match args.peek().map(String::as_str) {
                    Some("xhtml") => {
                        args.next();
                        InputFormat::Xhtml
                    }
                    Some("html") => {
                        args.next();
                        InputFormat::Html
                    }
                    _ => InputFormat::Html,
                };
                while let Some(arg) = args.next() {
                    match arg.as_str() {
                        "--also-check-css" => also_check_css = true,
                        "--csp-header" => {
                            csp_header = Some(args.next().ok_or("missing value for --csp-header")?)
                        }
                        other => return Err(format!("unknown arg: {other}").into()),
                    }
                }
                let bytes = std::fs::read(&path_or_uri)?;
                let rules = pack_html_conformance()
                    .merge(pack_aria())
                    .merge(pack_i18n());
                let rules = rules.merge(pack_css_checks());
                let base_uri = std::fs::canonicalize(&path_or_uri)
                    .ok()
                    .map(|p| format!("file://{}", p.to_string_lossy()));
                let source = HtmlEventSource::from_bytes(path_or_uri, format, bytes)?;
                let report = html_inspector::validate_events(
                    source,
                    rules,
                    Config {
                        also_check_css,
                        min_severity,
                        base_uri,
                        csp_header,
                        ..Config::default()
                    },
                )?;
                println!("{}", serde_json::to_string_pretty(&report)?);
                Ok(())
            }
        }
        "file" => {
            let path = args.next().ok_or("usage: html_inspector_cli file <path>")?;
            let mut also_check_css = false;
            let mut csp_header: Option<String> = None;
            let mut format: Option<InputFormat> = None;
            while let Some(arg) = args.next() {
                match arg.as_str() {
                    "html" => format = Some(InputFormat::Html),
                    "xhtml" => format = Some(InputFormat::Xhtml),
                    "--also-check-css" => also_check_css = true,
                    "--csp-header" => {
                        csp_header = Some(args.next().ok_or("missing value for --csp-header")?)
                    }
                    other if other.starts_with("--") => {
                        return Err(format!("unknown arg: {other}").into());
                    }
                    other => return Err(format!("unknown format: {other}").into()),
                }
            }
            let format = format.unwrap_or(InputFormat::Html);
            let bytes = std::fs::read(&path)?;
            let rules = pack_html_conformance()
                .merge(pack_aria())
                .merge(pack_i18n())
                .merge(pack_css_checks());
            let base_uri = std::fs::canonicalize(&path)
                .ok()
                .map(|p| format!("file://{}", p.to_string_lossy()));
            let source = HtmlEventSource::from_bytes(path, format, bytes)?;
            let report = html_inspector::validate_events(
                source,
                rules,
                Config {
                    also_check_css,
                    min_severity,
                    base_uri,
                    csp_header,
                    ..Config::default()
                },
            )?;
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        "css" => {
            let path_or_uri = args
                .next()
                .ok_or("usage: html_inspector_cli css <path-or-uri> [--profile P] [--medium M] [--warning N] [--allow-network]")?;
            let (allow_network, config) = parse_css_cli_options(&mut args)?;
            let mut report = validate_css_path_or_uri(&path_or_uri, &config, allow_network)?;
            if matches!(min_severity, Severity::Error) {
                report
                    .messages
                    .retain(|m| matches!(m.severity, css_inspector::Severity::Error));
            }
            report.errors = report
                .messages
                .iter()
                .filter(|m| matches!(m.severity, css_inspector::Severity::Error))
                .count();
            report.warnings = report
                .messages
                .iter()
                .filter(|m| matches!(m.severity, css_inspector::Severity::Warning))
                .count();
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        "serve" => {
            let mut bind: std::net::IpAddr = "127.0.0.1".parse().unwrap();
            let mut port: u16 = 8888;
            let mut max_bytes: usize = 20 * 1024 * 1024;

            match args.next().as_deref() {
                Some(v) if !v.starts_with("--") => port = v.parse()?,
                Some(v) if v.starts_with("--") => {
                    // Push back by handling below.
                    match v {
                        "--bind" => {
                            bind = args.next().ok_or("missing value for --bind")?.parse()?
                        }
                        "--max-bytes" => {
                            max_bytes = args
                                .next()
                                .ok_or("missing value for --max-bytes")?
                                .parse()?
                        }
                        other => return Err(format!("unknown arg: {other}").into()),
                    }
                }
                None => {}
                _ => {}
            }

            while let Some(arg) = args.next() {
                match arg.as_str() {
                    "--bind" => bind = args.next().ok_or("missing value for --bind")?.parse()?,
                    "--max-bytes" => {
                        max_bytes = args
                            .next()
                            .ok_or("missing value for --max-bytes")?
                            .parse()?
                    }
                    other => return Err(format!("unknown arg: {other}").into()),
                }
            }

            server::serve(bind, port, max_bytes, min_severity)?;
            Ok(())
        }
        _ => {
            eprintln!("usage:");
            eprintln!(
                "  html_inspector_cli [--min-severity error|warning|info] <command> [args...]"
            );
            eprintln!("  html_inspector_cli check <path-or-uri> [html|xhtml]");
            eprintln!("  html_inspector_cli file <path> [html|xhtml] [--also-check-css]");
            eprintln!(
                "  html_inspector_cli css <path-or-uri> [--profile P] [--medium M] [--warning N] [--allow-network]"
            );
            eprintln!("  html_inspector_cli serve [port] [--bind IP] [--max-bytes N]");
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_css_cli_options, validate_css_path_or_uri};

    #[test]
    fn parse_css_cli_options_defaults_match_vnu() {
        let mut args = Vec::<String>::new().into_iter();
        let (allow_network, cfg) = parse_css_cli_options(&mut args).unwrap();
        assert!(!allow_network);
        assert_eq!(cfg.profile.as_deref(), Some("css3svg"));
        assert_eq!(cfg.medium.as_deref(), Some("all"));
        assert_eq!(cfg.warning.as_deref(), Some("-1"));
    }

    #[test]
    fn parse_css_cli_options_parses_flags_and_values() {
        let mut args = vec![
            "--profile".to_string(),
            "p".to_string(),
            "--medium".to_string(),
            "m".to_string(),
            "--warning".to_string(),
            "w".to_string(),
            "--allow-network".to_string(),
        ]
        .into_iter();
        let (allow_network, cfg) = parse_css_cli_options(&mut args).unwrap();
        assert!(allow_network);
        assert_eq!(cfg.profile.as_deref(), Some("p"));
        assert_eq!(cfg.medium.as_deref(), Some("m"));
        assert_eq!(cfg.warning.as_deref(), Some("w"));
    }

    #[test]
    fn parse_css_cli_options_rejects_unknown_args() {
        let mut args = vec!["--nope".to_string()].into_iter();
        let err = parse_css_cli_options(&mut args).unwrap_err();
        assert!(err.to_string().contains("unknown arg"));
    }

    #[test]
    fn parse_css_cli_options_requires_values_for_options() {
        let mut args = vec!["--profile".to_string()].into_iter();
        let err = parse_css_cli_options(&mut args).unwrap_err();
        assert!(err.to_string().contains("missing value for --profile"));
    }

    #[test]
    fn validate_css_path_or_uri_reads_files_when_no_scheme() {
        let dir = std::env::temp_dir();
        let path = dir.join(format!(
            "html_inspector_css_validate_{}_{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::write(&path, b"body { color: red }").unwrap();

        let cfg = css_inspector::Config::default();
        let report =
            validate_css_path_or_uri(path.to_string_lossy().as_ref(), &cfg, false).unwrap();
        let _ = std::fs::remove_file(&path);

        assert_eq!(report.errors, 0, "report was: {report:?}");
    }
}
