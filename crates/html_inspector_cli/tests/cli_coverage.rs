use std::path::PathBuf;
use std::process::Command;

fn cli() -> Command {
    Command::new(env!("CARGO_BIN_EXE_html_inspector_cli"))
}

fn temp_file_path(name: &str) -> PathBuf {
    let mut p = std::env::temp_dir();
    p.push(format!(
        "html_inspector_cli_test_{}_{}_{}",
        name,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    p
}

#[test]
fn no_args_prints_usage() {
    let out = cli().output().unwrap();
    assert!(out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("usage:"), "stderr was: {stderr}");
}

#[test]
fn file_command_outputs_json() {
    let path = temp_file_path("file");
    std::fs::write(&path, b"<!doctype html><p>hi</p>").unwrap();
    let out = cli()
        .arg("file")
        .arg(path.to_string_lossy().to_string())
        .arg("html")
        .output()
        .unwrap();
    let _ = std::fs::remove_file(&path);

    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let v: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(
        v.get("source_name").and_then(|v| v.as_str()),
        Some(path.to_string_lossy().as_ref())
    );
    assert!(v.get("messages").and_then(|v| v.as_array()).is_some());
}

#[test]
fn file_command_rejects_unknown_format() {
    let path = temp_file_path("badfmt");
    std::fs::write(&path, b"<!doctype html><p>hi</p>").unwrap();
    let out = cli()
        .arg("file")
        .arg(path.to_string_lossy().to_string())
        .arg("wat")
        .output()
        .unwrap();
    let _ = std::fs::remove_file(&path);

    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("unknown format"), "stderr was: {stderr}");
}

#[test]
fn css_command_outputs_json() {
    let path = temp_file_path("css");
    std::fs::write(&path, b"body { color: red }").unwrap();
    let out = cli()
        .arg("css")
        .arg(path.to_string_lossy().to_string())
        .output()
        .unwrap();
    let _ = std::fs::remove_file(&path);

    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let v: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(v.get("errors").and_then(|v| v.as_u64()), Some(0));
    assert!(v.get("warnings").and_then(|v| v.as_u64()).is_some());
    assert!(v.get("messages").and_then(|v| v.as_array()).is_some());
}

#[test]
fn check_command_uses_css_for_css_extension() {
    let mut path = temp_file_path("check_css");
    path.set_extension("css");
    std::fs::write(&path, b"body { color: red }").unwrap();
    let out = cli()
        .arg("check")
        .arg(path.to_string_lossy().to_string())
        .output()
        .unwrap();
    let _ = std::fs::remove_file(&path);

    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let v: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert!(v.get("errors").is_some(), "stdout was: {stdout}");
    assert!(v.get("warnings").is_some(), "stdout was: {stdout}");
}

#[test]
fn check_command_uses_css_for_uppercase_css_extension() {
    let mut path = temp_file_path("check_css_uppercase");
    path.set_extension("CSS");
    std::fs::write(&path, b"body { color: red }").unwrap();
    let out = cli()
        .arg("check")
        .arg(path.to_string_lossy().to_string())
        .output()
        .unwrap();
    let _ = std::fs::remove_file(&path);

    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let v: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert!(v.get("errors").is_some(), "stdout was: {stdout}");
    assert!(v.get("warnings").is_some(), "stdout was: {stdout}");
}

#[test]
fn check_command_accepts_options_without_explicit_format() {
    let mut path = temp_file_path("check_html_css_opt");
    path.set_extension("html");
    std::fs::write(
        &path,
        b"<!doctype html><html lang=en><head><meta charset=utf-8><title>t</title></head><body><p style=\"/*\">hi</p></body></html>",
    )
    .unwrap();

    let out = cli()
        .arg("check")
        .arg(path.to_string_lossy().to_string())
        .arg("--also-check-css")
        .output()
        .unwrap();
    let _ = std::fs::remove_file(&path);

    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let v: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let has_css_error = v
        .get("messages")
        .and_then(|v| v.as_array())
        .into_iter()
        .flatten()
        .any(|m| m.get("code").and_then(|v| v.as_str()) == Some("html.css.error"));
    assert!(has_css_error, "stdout was: {stdout}");
}

#[test]
fn check_command_rejects_format_after_options() {
    let mut path = temp_file_path("check_html_css_opt_late_fmt");
    path.set_extension("html");
    std::fs::write(
        &path,
        b"<!doctype html><html lang=en><head><meta charset=utf-8><title>t</title></head><body><p>ok</p></body></html>",
    )
    .unwrap();

    let out = cli()
        .arg("check")
        .arg(path.to_string_lossy().to_string())
        .arg("--also-check-css")
        .arg("html")
        .output()
        .unwrap();
    let _ = std::fs::remove_file(&path);

    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("unknown arg"), "stderr was: {stderr}");
}
