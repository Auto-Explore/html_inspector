use std::io::{BufRead, BufReader, Read, Write};
use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream};
use std::thread;

use base64::Engine as _;
use rustc_hash::FxHashMap;

use html_inspector_core::{Config, InputFormat, MessageOrder, Report, Severity};
use html_inspector_html::HtmlEventSource;
use html_inspector_rules_aria::pack_aria;
use html_inspector_rules_css::pack_css_checks;
use html_inspector_rules_html::pack_html_conformance;
use html_inspector_rules_i18n::pack_i18n;

use crate::util::ends_with_ascii_ci;

const MAX_HEADER_BYTES: usize = 64 * 1024;
const MAX_UI_BODY_BYTES: usize = 20 * 1024 * 1024;

#[inline]
fn trim_crlf(s: &str) -> &str {
    s.trim_end_matches(['\r', '\n'])
}

#[derive(serde::Deserialize)]
struct UiRequest {
    content: String,
    #[serde(default)]
    document_content_type: Option<String>,
    #[serde(default)]
    doc: Option<String>,
    #[serde(default)]
    parser: Option<String>,
    #[serde(default)]
    level: Option<String>,
    #[serde(default)]
    also_check_css: bool,
    #[serde(default)]
    csp_header: Option<String>,
    #[serde(default)]
    java_base_url: Option<String>,
    #[serde(default)]
    java_use_basic_auth: bool,
    #[serde(default)]
    java_basic_auth_user: Option<String>,
    #[serde(default)]
    java_basic_auth_password: Option<String>,
}

#[derive(serde::Serialize)]
struct UiRustResponse {
    report: Report,
    gnu: String,
}

#[derive(serde::Serialize)]
struct CompareResponse {
    rust: UiRustResponse,
    java: Option<JavaResponse>,
    diff: DiffSummary,
}

#[derive(serde::Serialize)]
struct JavaResponse {
    status: u16,
    content_type: Option<String>,
    body_json: Option<serde_json::Value>,
    body_text: Option<String>,
    error: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize)]
struct NormalizedMessage {
    #[serde(rename = "type")]
    type_: String,
    message: String,
    first_line: Option<u32>,
    first_col: Option<u32>,
}

#[derive(Default, serde::Serialize)]
struct DiffSummary {
    rust_counts: FxHashMap<String, usize>,
    java_counts: FxHashMap<String, usize>,
    only_in_rust: Vec<NormalizedMessage>,
    only_in_java: Vec<NormalizedMessage>,
}

pub fn serve(
    bind: IpAddr,
    port: u16,
    max_body_bytes: usize,
    default_min_severity: Severity,
) -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::new(bind, port);
    let listener = TcpListener::bind(addr)?;
    eprintln!("html_inspector server listening on http://{addr}/");

    for stream in listener.incoming() {
        let Ok(stream) = stream else { continue };
        let _ = stream.set_nodelay(true);
        thread::spawn(move || {
            let _ = handle_connection(stream, max_body_bytes, default_min_severity);
        });
    }

    Ok(())
}

fn handle_connection(
    mut stream: TcpStream,
    max_body_bytes: usize,
    default_min_severity: Severity,
) -> std::io::Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let req = match read_request(&mut reader, max_body_bytes) {
        Ok(r) => r,
        Err(e) => {
            let status = if e.contains("body too large") {
                413
            } else {
                400
            };
            write_response(
                &mut stream,
                status,
                "text/plain; charset=utf-8",
                format!("bad request: {e}\n").as_bytes(),
            )?;
            return Ok(());
        }
    };

    let (path, query) = split_path_query(&req.target);

    if req.method == "GET" && path == "/health" {
        write_response(&mut stream, 200, "text/plain; charset=utf-8", b"ok\n")?;
        return Ok(());
    }

    if req.method == "GET" && path == "/" {
        let body = ui_html().into_bytes();
        write_response(&mut stream, 200, "text/html; charset=utf-8", &body)?;
        return Ok(());
    }

    if req.method == "GET" {
        write_response(
            &mut stream,
            200,
            "text/plain; charset=utf-8",
            b"html_inspector server\n\nPOST /?out=json|gnu&parser=html|xml&level=error|warning&doc=name\n",
        )?;
        return Ok(());
    }

    if req.method != "POST" {
        write_response(
            &mut stream,
            405,
            "text/plain; charset=utf-8",
            b"method not allowed\n",
        )?;
        return Ok(());
    }

    if path == "/api/rust" {
        let resp = match handle_ui_rust(&req, default_min_severity) {
            Ok(r) => r,
            Err(e) => {
                write_response(
                    &mut stream,
                    400,
                    "application/json; charset=utf-8",
                    serde_json::json!({ "error": e }).to_string().as_bytes(),
                )?;
                return Ok(());
            }
        };
        let body = serde_json::to_vec(&resp).unwrap_or_else(|e| {
            serde_json::json!({ "error": format!("json error: {e}") })
                .to_string()
                .into_bytes()
        });
        write_response(&mut stream, 200, "application/json; charset=utf-8", &body)?;
        return Ok(());
    }

    if path == "/api/compare" {
        let resp = match handle_ui_compare(&req, default_min_severity) {
            Ok(r) => r,
            Err(e) => {
                write_response(
                    &mut stream,
                    400,
                    "application/json; charset=utf-8",
                    serde_json::json!({ "error": e }).to_string().as_bytes(),
                )?;
                return Ok(());
            }
        };
        let body = serde_json::to_vec(&resp).unwrap_or_else(|e| {
            serde_json::json!({ "error": format!("json error: {e}") })
                .to_string()
                .into_bytes()
        });
        write_response(&mut stream, 200, "application/json; charset=utf-8", &body)?;
        return Ok(());
    }

    let params = Params::from_query_and_headers(query, &req.headers);
    let out = params.out.as_deref().unwrap_or("gnu");
    let content_type = req
        .headers
        .get("content-type")
        .map(String::as_str)
        .unwrap_or("");

    let (format, doc_name) = detect_input_format_and_name(&params, content_type);
    let mut report =
        match validate_html_bytes(doc_name, format, req.body, &params, default_min_severity) {
            Ok(r) => r,
            Err(e) => {
                write_response(
                    &mut stream,
                    400,
                    "text/plain; charset=utf-8",
                    format!("invalid input: {e}\n").as_bytes(),
                )?;
                return Ok(());
            }
        };

    apply_level_filter(&mut report, params.level.as_deref());

    match out {
        "json" => {
            let body = serde_json::to_vec(&report)
                .unwrap_or_else(|e| format!("json error: {e}\n").into_bytes());
            write_response(&mut stream, 200, "application/json; charset=utf-8", &body)?;
        }
        "gnu" | "text" => {
            let body = format_gnu(&report).into_bytes();
            write_response(&mut stream, 200, "text/plain; charset=utf-8", &body)?;
        }
        _ => {
            write_response(
                &mut stream,
                400,
                "text/plain; charset=utf-8",
                b"unsupported out; use out=json or out=gnu\n",
            )?;
        }
    }

    Ok(())
}

struct HttpRequest {
    method: String,
    target: String,
    headers: FxHashMap<String, String>,
    body: Vec<u8>,
}

struct HttpRequestHead {
    method: String,
    target: String,
    headers: FxHashMap<String, String>,
    expect_continue: bool,
}

fn read_request(
    reader: &mut BufReader<TcpStream>,
    max_body_bytes: usize,
) -> Result<HttpRequest, String> {
    let head = read_request_head(reader)?;

    if head.expect_continue {
        let stream = reader.get_mut();
        let _ = stream.write_all(b"HTTP/1.1 100 Continue\r\n\r\n");
        let _ = stream.flush();
    }

    let body = read_request_body(reader, &head.headers, max_body_bytes)?;
    Ok(HttpRequest {
        method: head.method,
        target: head.target,
        headers: head.headers,
        body,
    })
}

#[cfg(test)]
fn read_request_from<R: BufRead>(
    reader: &mut R,
    max_body_bytes: usize,
) -> Result<HttpRequest, String> {
    let head = read_request_head(reader)?;
    let body = read_request_body(reader, &head.headers, max_body_bytes)?;
    Ok(HttpRequest {
        method: head.method,
        target: head.target,
        headers: head.headers,
        body,
    })
}

fn read_request_head<R: BufRead>(reader: &mut R) -> Result<HttpRequestHead, String> {
    let mut first = String::new();
    reader
        .read_line(&mut first)
        .map_err(|e| format!("read request line: {e}"))?;
    if first.is_empty() {
        return Err("empty request".to_string());
    }
    let first = trim_crlf(&first);
    let mut parts = first.split_whitespace();
    let method = parts.next().ok_or("missing method")?.to_string();
    let target = parts.next().ok_or("missing request target")?.to_string();
    let _http_version = parts.next().ok_or("missing http version")?;

    let mut headers: FxHashMap<String, String> = FxHashMap::default();
    let mut header_bytes = first.len() + 2;
    let mut line = String::new();
    loop {
        line.clear();
        reader
            .read_line(&mut line)
            .map_err(|e| format!("read header: {e}"))?;
        if line.is_empty() {
            return Err("unexpected EOF while reading headers".to_string());
        }
        header_bytes = header_bytes.saturating_add(line.len());
        if header_bytes > MAX_HEADER_BYTES {
            return Err("headers too large".to_string());
        }
        let line_trimmed = trim_crlf(&line);
        if line_trimmed.is_empty() {
            break;
        }
        let (name, value) = line_trimmed
            .split_once(':')
            .ok_or_else(|| format!("invalid header line: {line_trimmed}"))?;
        headers.insert(name.trim().to_ascii_lowercase(), value.trim().to_string());
    }

    let expect_continue = headers
        .get("expect")
        .is_some_and(|v| v.eq_ignore_ascii_case("100-continue"));

    Ok(HttpRequestHead {
        method,
        target,
        headers,
        expect_continue,
    })
}

fn read_request_body<R: BufRead>(
    reader: &mut R,
    headers: &FxHashMap<String, String>,
    max_body_bytes: usize,
) -> Result<Vec<u8>, String> {
    let is_chunked = transfer_encoding_is_chunked(headers);
    let content_length = headers
        .get("content-length")
        .and_then(|v| v.parse::<usize>().ok());

    match (is_chunked, content_length) {
        (true, _) => read_chunked_body(reader, max_body_bytes),
        (false, Some(len)) => {
            if len > max_body_bytes {
                return Err("body too large".to_string());
            }
            let mut body = vec![0u8; len];
            reader
                .read_exact(&mut body)
                .map_err(|e| format!("read body: {e}"))?;
            Ok(body)
        }
        (false, None) => Ok(Vec::new()),
    }
}

#[inline]
fn transfer_encoding_is_chunked(headers: &FxHashMap<String, String>) -> bool {
    headers
        .get("transfer-encoding")
        .is_some_and(|v| v.eq_ignore_ascii_case("chunked"))
}

fn read_chunked_body<R: BufRead>(reader: &mut R, max_body_bytes: usize) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();
    let mut line = String::new();
    loop {
        line.clear();
        reader
            .read_line(&mut line)
            .map_err(|e| format!("read chunk size: {e}"))?;
        if line.is_empty() {
            return Err("unexpected EOF while reading chunk size".to_string());
        }
        let size_line = trim_crlf(&line);
        let size_hex = size_line
            .split_once(';')
            .map_or(size_line, |(hex, _ext)| hex);
        let size = usize::from_str_radix(size_hex.trim(), 16)
            .map_err(|_| format!("invalid chunk size: {size_line}"))?;
        if size == 0 {
            // Consume trailing CRLF after the 0-size chunk plus any trailers.
            loop {
                line.clear();
                reader
                    .read_line(&mut line)
                    .map_err(|e| format!("read chunk trailer: {e}"))?;
                if line.is_empty() {
                    return Err("unexpected EOF while reading chunk trailer".to_string());
                }
                if trim_crlf(&line).is_empty() {
                    break;
                }
            }
            break;
        }
        if out.len().saturating_add(size) > max_body_bytes {
            return Err("body too large".to_string());
        }
        let prev_len = out.len();
        out.resize(prev_len + size, 0u8);
        reader
            .read_exact(&mut out[prev_len..])
            .map_err(|e| format!("read chunk: {e}"))?;

        // Consume CRLF after chunk.
        let mut crlf = [0u8; 2];
        reader
            .read_exact(&mut crlf)
            .map_err(|e| format!("read chunk CRLF: {e}"))?;
        if crlf != [b'\r', b'\n'] {
            return Err("invalid chunk terminator".to_string());
        }
    }
    Ok(out)
}

fn split_path_query(target: &str) -> (&str, &str) {
    // Handle origin-form (`/path?query`) and absolute-form (`http://host/path?query`) targets.
    let t = if let Some(rest) = target
        .strip_prefix("http://")
        .or_else(|| target.strip_prefix("https://"))
    {
        match rest.find('/') {
            Some(idx) => &rest[idx..],
            None => "/",
        }
    } else {
        target
    };
    t.split_once('?').unwrap_or((t, ""))
}

#[derive(Default, Debug, Clone)]
struct Params {
    out: Option<String>,
    level: Option<String>,
    parser: Option<String>,
    doc: Option<String>,
    also_check_css: bool,
    csp_header: Option<String>,
}

impl Params {
    fn parse(query: &str) -> Self {
        let mut p = Params::default();
        for (k, v) in parse_query(query) {
            match k.as_str() {
                "out" => p.out = Some(v),
                "level" => p.level = Some(v),
                "parser" => p.parser = Some(v),
                "doc" => p.doc = Some(v),
                "also_check_css" => p.also_check_css = is_truthy(&v),
                "csp_header" => p.csp_header = Some(v),
                _ => {}
            }
        }
        p
    }

    fn from_query_and_headers(query: &str, headers: &FxHashMap<String, String>) -> Self {
        let mut p = Self::parse(query);
        if p.csp_header.is_none() {
            p.csp_header = headers.get("content-security-policy").cloned();
        }
        p
    }
}

fn is_truthy(s: &str) -> bool {
    matches!(s, "1" | "true" | "yes")
}

fn parse_query(query: &str) -> impl Iterator<Item = (String, String)> + '_ {
    query.split('&').filter_map(|pair| {
        if pair.is_empty() {
            return None;
        }
        let (k, v) = pair.split_once('=').unwrap_or((pair, ""));
        let k = percent_decode(k).ok()?;
        let v = percent_decode(v).ok()?;
        Some((k, v))
    })
}

fn percent_decode(s: &str) -> Result<String, ()> {
    css_inspector::url_decode_plus(s).map_err(|_| ())
}

fn detect_input_format_and_name(params: &Params, content_type: &str) -> (InputFormat, String) {
    let parser = params.parser.as_deref().unwrap_or("");
    let doc = params.doc.as_deref().unwrap_or("[stdin]");

    let is_xhtml = matches!(parser, "xml" | "xmldtd")
        || content_type.contains("application/xhtml+xml")
        || content_type.contains("application/xml")
        || content_type.contains("text/xml")
        || ends_with_ascii_ci(doc, ".xhtml")
        || ends_with_ascii_ci(doc, ".xht")
        || ends_with_ascii_ci(doc, ".xml");
    let format = if is_xhtml {
        InputFormat::Xhtml
    } else {
        InputFormat::Html
    };

    (format, doc.to_owned())
}

fn ui_html() -> String {
    // Intentionally simple, single-file HTML with inline JS.
    r#"<!doctype html>
<meta charset="utf-8">
<title>html_inspector (Rust) — Compare with Java</title>
<style>
  body { font-family: system-ui, sans-serif; margin: 16px; max-width: 1100px; }
  textarea { width: 100%; height: 240px; font-family: ui-monospace, SFMono-Regular, Menlo, monospace; }
  input[type="text"] { width: 100%; }
  .row { display: flex; gap: 16px; }
  .col { flex: 1; }
  pre { background: #f6f8fa; padding: 12px; overflow: auto; }
  .small { font-size: 12px; color: #555; }
  .err { color: #b00020; }
  button { padding: 8px 12px; }
</style>
<h1>html_inspector (Rust)</h1>
<p class="small">Paste HTML/XHTML below, then validate with Rust or compare with a running Java vnu server.</p>

<div class="row">
  <div class="col">
    <label>Document</label>
    <textarea id="content" spellcheck="false"><!DOCTYPE html><html lang="en"><head><title>Test</title></head><body><p>Hello</p></body></html></textarea>
    <div class="row">
      <div class="col">
        <label>Upload file</label>
        <input id="file" type="file">
      </div>
      <div class="col">
        <label>Doc name (optional)</label>
        <input id="doc" type="text" placeholder="stdin.html">
      </div>
    </div>
    <div class="row">
      <div class="col">
        <label>Parser</label>
        <select id="parser">
          <option value="html">html</option>
          <option value="xml">xml</option>
        </select>
      </div>
      <div class="col">
        <label>Document Content-Type (optional)</label>
        <input id="ctype" type="text" placeholder="text/html; charset=utf-8">
      </div>
    </div>
    <div class="row">
      <div class="col">
        <label><input id="alsoCss" type="checkbox"> also_check_css</label><br>
      </div>
      <div class="col">
        <label>Java base URL (optional)</label>
        <input id="java" type="text" value="http://127.0.0.1:8889/" placeholder="http://127.0.0.1:8889/">
        <div class="small">Start Java server separately (e.g. vnu.jar --http 8889).</div>
        <label style="display:block; margin-top: 8px;"><input id="javaAuth" type="checkbox"> Use basic authentication</label>
        <div id="javaAuthFields" style="display:none; margin-top: 8px;">
          <div class="row">
            <div class="col">
              <label>Username</label>
              <input id="javaUser" type="text" autocomplete="username">
            </div>
            <div class="col">
              <label>Password</label>
              <input id="javaPass" type="password" autocomplete="current-password">
            </div>
          </div>
        </div>
      </div>
    </div>
    <div style="margin-top: 12px;">
      <button id="btnRust">Validate (Rust)</button>
      <button id="btnCompare">Compare (Rust vs Java)</button>
      <span id="status" class="small"></span>
    </div>
  </div>

  <div class="col">
    <h2>Summary</h2>
    <pre id="summary"></pre>
    <h2>Rust</h2>
    <pre id="rust"></pre>
    <h2>Java</h2>
    <pre id="javaOut"></pre>
  </div>
</div>

<script>
  const el = (id) => document.getElementById(id);
  el('file').addEventListener('change', async () => {
    const f = el('file').files?.[0];
    if (!f) return;
    el('content').value = await f.text();
    el('doc').value = f.name;
  });

  function updateJavaAuthFields() {
    const enabled = el('javaAuth').checked;
    el('javaAuthFields').style.display = enabled ? 'block' : 'none';
    el('javaUser').disabled = !enabled;
    el('javaPass').disabled = !enabled;
  }
  el('javaAuth').addEventListener('change', updateJavaAuthFields);
  updateJavaAuthFields();

  function buildReq() {
    const useBasicAuth = el('javaAuth').checked;
    const javaBaseUrl = el('java').value.trim();
    return {
      content: el('content').value,
      doc: el('doc').value || null,
      parser: el('parser').value,
      document_content_type: el('ctype').value || null,
      also_check_css: el('alsoCss').checked,
      java_base_url: javaBaseUrl ? javaBaseUrl : null,
      java_use_basic_auth: useBasicAuth,
      java_basic_auth_user: useBasicAuth ? (el('javaUser').value || null) : null,
      java_basic_auth_password: useBasicAuth ? (el('javaPass').value || null) : null,
    };
  }

  async function postJson(path, obj) {
    const res = await fetch(path, {
      method: 'POST',
      headers: { 'content-type': 'application/json; charset=utf-8' },
      body: JSON.stringify(obj),
    });
    const text = await res.text();
    let json;
    try { json = JSON.parse(text); } catch (e) { json = { error: 'invalid json', raw: text }; }
    return { ok: res.ok, status: res.status, json };
  }

  function pretty(x) { return JSON.stringify(x, null, 2); }

  el('btnRust').addEventListener('click', async () => {
    el('status').textContent = 'Running...';
    const { ok, json } = await postJson('/api/rust', buildReq());
    el('status').textContent = ok ? 'OK' : 'Error';
    el('summary').textContent = '';
    el('rust').textContent = pretty(json);
    el('javaOut').textContent = '';
  });

  el('btnCompare').addEventListener('click', async () => {
    const req = buildReq();
    if (!req.java_base_url) {
      el('status').textContent = 'Set Java base URL to compare.';
      return;
    }
    el('status').textContent = 'Running...';
    const { ok, json } = await postJson('/api/compare', req);
    el('status').textContent = ok ? 'OK' : 'Error';
    el('summary').textContent = pretty(json.diff || json);
    el('rust').textContent = pretty(json.rust || {});
    el('javaOut').textContent = pretty(json.java || {});
  });
</script>
"#
    .to_string()
}

fn handle_ui_rust(
    req: &HttpRequest,
    default_min_severity: Severity,
) -> Result<UiRustResponse, String> {
    if req.body.len() > MAX_UI_BODY_BYTES {
        return Err("body too large".to_string());
    }
    let ui: UiRequest =
        serde_json::from_slice(&req.body).map_err(|e| format!("invalid json: {e}"))?;
    validate_with_params(&ui, default_min_severity)
        .map(|(report, gnu)| UiRustResponse { report, gnu })
}

fn handle_ui_compare(
    req: &HttpRequest,
    default_min_severity: Severity,
) -> Result<CompareResponse, String> {
    if req.body.len() > MAX_UI_BODY_BYTES {
        return Err("body too large".to_string());
    }
    let ui: UiRequest =
        serde_json::from_slice(&req.body).map_err(|e| format!("invalid json: {e}"))?;

    let (rust_report, rust_gnu) = validate_with_params(&ui, default_min_severity)?;
    let rust_resp = UiRustResponse {
        report: rust_report,
        gnu: rust_gnu,
    };

    let java = ui
        .java_base_url
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|base| call_java_validator(base, &ui));

    let diff = diff_reports(
        &rust_resp.report,
        java.as_ref().and_then(|j| j.body_json.as_ref()),
    );

    Ok(CompareResponse {
        rust: rust_resp,
        java,
        diff,
    })
}

fn validate_with_params(
    ui: &UiRequest,
    default_min_severity: Severity,
) -> Result<(Report, String), String> {
    let params = Params {
        out: Some("json".to_string()),
        level: ui.level.clone(),
        parser: ui.parser.clone(),
        doc: ui.doc.clone(),
        also_check_css: ui.also_check_css,
        csp_header: ui.csp_header.clone().filter(|s| !s.trim().is_empty()),
    };

    let content_type = ui.document_content_type.as_deref().unwrap_or("");
    let (format, doc_name) = detect_input_format_and_name(&params, content_type);

    let mut report = validate_html_bytes(
        doc_name,
        format,
        ui.content.as_bytes().to_vec(),
        &params,
        default_min_severity,
    )
    .map_err(|e| format!("invalid input: {e}"))?;
    apply_level_filter(&mut report, params.level.as_deref());
    let gnu = format_gnu(&report);
    Ok((report, gnu))
}

fn call_java_validator(java_base_url: &str, ui: &UiRequest) -> JavaResponse {
    let mut out = JavaResponse {
        status: 0,
        content_type: None,
        body_json: None,
        body_text: None,
        error: None,
    };

    let (host, port, base_path) = match parse_http_base_url(java_base_url) {
        Ok(v) => v,
        Err(e) => {
            out.error = Some(e);
            return out;
        }
    };

    let doc = ui
        .doc
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or("stdin");
    let parser = ui.parser.as_deref().unwrap_or("html");

    let path = if base_path.is_empty() {
        "/".to_string()
    } else {
        base_path
    };
    let q = format!(
        "out=json&parser={}&doc={}",
        url_encode(parser),
        url_encode(doc)
    );
    let full_path = if path.contains('?') {
        format!("{path}&{q}")
    } else {
        format!("{path}?{q}")
    };

    let body = ui.content.as_bytes();
    let doc_ct = ui
        .document_content_type
        .as_deref()
        .unwrap_or("text/html; charset=utf-8");

    let mut extra_headers: Vec<(String, String)> = Vec::new();
    if ui.java_use_basic_auth {
        let user = ui
            .java_basic_auth_user
            .as_deref()
            .unwrap_or("")
            .trim()
            .to_string();
        let pass = ui
            .java_basic_auth_password
            .as_deref()
            .unwrap_or("")
            .to_string();
        if user.is_empty() {
            out.error = Some("java_use_basic_auth is set but java_basic_auth_user is empty".into());
            return out;
        }
        extra_headers.push((
            "Authorization".to_string(),
            basic_auth_header_value(&user, &pass),
        ));
    }

    match http_post(
        &host,
        port,
        &full_path,
        doc_ct,
        &extra_headers,
        body,
        MAX_UI_BODY_BYTES,
    ) {
        Ok(resp) => {
            out.status = resp.status;
            out.content_type = resp.headers.get("content-type").cloned();
            let text = String::from_utf8(resp.body.clone()).ok();
            out.body_text = text.clone();
            out.body_json = text.and_then(|t| serde_json::from_str(&t).ok());
        }
        Err(e) => out.error = Some(e),
    }

    out
}

fn diff_reports(rust: &Report, java_json: Option<&serde_json::Value>) -> DiffSummary {
    let rust_msgs: Vec<NormalizedMessage> = rust
        .messages
        .iter()
        .map(|m| NormalizedMessage {
            type_: match m.severity {
                Severity::Error => "error",
                Severity::Warning => "warning",
                Severity::Info => "info",
            }
            .to_string(),
            message: m.message.clone(),
            first_line: m.span.map(|s| s.line),
            first_col: m.span.map(|s| s.col),
        })
        .collect();

    let java_msgs: Vec<NormalizedMessage> = java_json
        .and_then(|v| v.get("messages"))
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|m| {
                    let msg = m.get("message")?.as_str()?.to_string();
                    let type_ = m
                        .get("type")
                        .and_then(|t| t.as_str())
                        .unwrap_or("info")
                        .to_string();
                    let first_line = m
                        .get("firstLine")
                        .and_then(|n| n.as_u64())
                        .map(|n| n as u32);
                    let first_col = m
                        .get("firstColumn")
                        .and_then(|n| n.as_u64())
                        .map(|n| n as u32);
                    Some(NormalizedMessage {
                        type_,
                        message: msg,
                        first_line,
                        first_col,
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    let mut out = DiffSummary::default();
    for m in &rust_msgs {
        *out.rust_counts.entry(m.type_.clone()).or_insert(0) += 1;
    }
    for m in &java_msgs {
        *out.java_counts.entry(m.type_.clone()).or_insert(0) += 1;
    }

    // Best-effort diff on (type, line, col, message). For large docs, only keep a small sample.
    let mut rust_set: std::collections::BTreeSet<(String, u32, u32, String)> =
        std::collections::BTreeSet::new();
    for m in &rust_msgs {
        rust_set.insert((
            m.type_.clone(),
            m.first_line.unwrap_or(0),
            m.first_col.unwrap_or(0),
            m.message.clone(),
        ));
    }
    let mut java_set: std::collections::BTreeSet<(String, u32, u32, String)> =
        std::collections::BTreeSet::new();
    for m in &java_msgs {
        java_set.insert((
            m.type_.clone(),
            m.first_line.unwrap_or(0),
            m.first_col.unwrap_or(0),
            m.message.clone(),
        ));
    }

    for m in &rust_msgs {
        let k = (
            m.type_.clone(),
            m.first_line.unwrap_or(0),
            m.first_col.unwrap_or(0),
            m.message.clone(),
        );
        if !java_set.contains(&k) {
            out.only_in_rust.push(m.clone());
            if out.only_in_rust.len() >= 20 {
                break;
            }
        }
    }
    for m in &java_msgs {
        let k = (
            m.type_.clone(),
            m.first_line.unwrap_or(0),
            m.first_col.unwrap_or(0),
            m.message.clone(),
        );
        if !rust_set.contains(&k) {
            out.only_in_java.push(m.clone());
            if out.only_in_java.len() >= 20 {
                break;
            }
        }
    }

    out
}

fn url_encode(s: &str) -> String {
    // Minimal x-www-form-urlencoded encoding for query params.
    let mut out = String::new();
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char);
            }
            b' ' => out.push_str("%20"),
            other => out.push_str(&format!("%{:02X}", other)),
        }
    }
    out
}

fn parse_http_base_url(s: &str) -> Result<(String, u16, String), String> {
    let s = s.trim();
    let rest = s
        .strip_prefix("http://")
        .ok_or("java_base_url must start with http://".to_string())?;
    let (authority, path) = match rest.find('/') {
        Some(idx) => (&rest[..idx], &rest[idx..]),
        None => (rest, "/"),
    };
    let (host, port) = match authority.split_once(':') {
        Some((h, p)) => {
            let port = p
                .parse::<u16>()
                .map_err(|_| "invalid port in java_base_url".to_string())?;
            (h.to_string(), port)
        }
        None => (authority.to_string(), 80),
    };
    if host.is_empty() {
        return Err("missing host in java_base_url".to_string());
    }
    Ok((host, port, path.to_string()))
}

struct HttpResponse {
    status: u16,
    headers: FxHashMap<String, String>,
    body: Vec<u8>,
}

fn http_post(
    host: &str,
    port: u16,
    path_and_query: &str,
    content_type: &str,
    extra_headers: &[(String, String)],
    body: &[u8],
    max_body_bytes: usize,
) -> Result<HttpResponse, String> {
    let addr = format!("{host}:{port}");
    let mut stream = TcpStream::connect(&addr).map_err(|e| format!("connect {addr}: {e}"))?;
    let mut req = String::new();
    req.push_str(&format!("POST {} HTTP/1.1\r\n", path_and_query));
    req.push_str(&format!("Host: {}\r\n", host));
    // vnu.jar's Jetty server rejects requests without User-Agent.
    if !extra_headers
        .iter()
        .any(|(name, _)| name.eq_ignore_ascii_case("user-agent"))
    {
        req.push_str(&format!(
            "User-Agent: html_inspector_cli/{}\r\n",
            env!("CARGO_PKG_VERSION")
        ));
    }
    for (name, value) in extra_headers {
        if name.contains(['\r', '\n']) || value.contains(['\r', '\n']) {
            return Err("invalid header value".to_string());
        }
        req.push_str(&format!("{name}: {value}\r\n"));
    }
    req.push_str(&format!(
        "Connection: close\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
        content_type,
        body.len()
    ));
    stream
        .write_all(req.as_bytes())
        .and_then(|_| stream.write_all(body))
        .and_then(|_| stream.flush())
        .map_err(|e| format!("write request: {e}"))?;

    let mut reader = BufReader::new(stream);
    read_response(&mut reader, max_body_bytes)
}

fn basic_auth_header_value(user: &str, password: &str) -> String {
    let token = base64::engine::general_purpose::STANDARD.encode(format!("{user}:{password}"));
    format!("Basic {token}")
}

fn read_response<R: BufRead>(
    reader: &mut R,
    max_body_bytes: usize,
) -> Result<HttpResponse, String> {
    let mut status_line = String::new();
    reader
        .read_line(&mut status_line)
        .map_err(|e| format!("read status line: {e}"))?;
    if status_line.is_empty() {
        return Err("empty response".to_string());
    }
    let status_line = trim_crlf(&status_line);
    let mut parts = status_line.split_whitespace();
    let _http = parts.next().ok_or("missing http version")?;
    let status = parts
        .next()
        .ok_or("missing status code")?
        .parse::<u16>()
        .map_err(|_| "invalid status code".to_string())?;

    let mut headers: FxHashMap<String, String> = FxHashMap::default();
    let mut header_bytes = status_line.len() + 2;
    let mut line = String::new();
    loop {
        line.clear();
        reader
            .read_line(&mut line)
            .map_err(|e| format!("read header: {e}"))?;
        if line.is_empty() {
            return Err("unexpected EOF while reading headers".to_string());
        }
        header_bytes = header_bytes.saturating_add(line.len());
        if header_bytes > MAX_HEADER_BYTES {
            return Err("headers too large".to_string());
        }
        let line_trimmed = trim_crlf(&line);
        if line_trimmed.is_empty() {
            break;
        }
        let (name, value) = line_trimmed
            .split_once(':')
            .ok_or_else(|| format!("invalid header line: {line_trimmed}"))?;
        headers.insert(name.trim().to_ascii_lowercase(), value.trim().to_string());
    }

    let body = read_response_body(reader, &headers, max_body_bytes)?;
    Ok(HttpResponse {
        status,
        headers,
        body,
    })
}

fn read_response_body<R: BufRead>(
    reader: &mut R,
    headers: &FxHashMap<String, String>,
    max_body_bytes: usize,
) -> Result<Vec<u8>, String> {
    let is_chunked = transfer_encoding_is_chunked_response(headers);
    let content_length = headers
        .get("content-length")
        .and_then(|v| v.parse::<usize>().ok());

    match (is_chunked, content_length) {
        (true, _) => read_chunked_body(reader, max_body_bytes),
        (false, Some(len)) => {
            if len > max_body_bytes {
                return Err("body too large".to_string());
            }
            let mut body = vec![0u8; len];
            reader
                .read_exact(&mut body)
                .map_err(|e| format!("read body: {e}"))?;
            Ok(body)
        }
        (false, None) => {
            let mut body = Vec::new();
            reader
                .take(max_body_bytes as u64 + 1)
                .read_to_end(&mut body)
                .map_err(|e| format!("read body: {e}"))?;
            if body.len() > max_body_bytes {
                return Err("body too large".to_string());
            }
            Ok(body)
        }
    }
}

#[inline]
fn transfer_encoding_is_chunked_response(headers: &FxHashMap<String, String>) -> bool {
    headers.get("transfer-encoding").is_some_and(|v| {
        v.split(',')
            .any(|t| t.trim().eq_ignore_ascii_case("chunked"))
    })
}

fn validate_html_bytes(
    source_name: String,
    format: InputFormat,
    bytes: Vec<u8>,
    params: &Params,
    default_min_severity: Severity,
) -> Result<Report, html_inspector_core::ValidatorError> {
    let source = HtmlEventSource::from_bytes(source_name, format, bytes)?;
    let rules = pack_html_conformance()
        .merge(pack_aria())
        .merge(pack_i18n())
        .merge(pack_css_checks());

    let base_uri = params
        .doc
        .as_deref()
        .filter(|d| d.starts_with("http://") || d.starts_with("https://"))
        .map(|s| s.to_string());

    let min_severity = match params.level.as_deref() {
        Some("error") => Severity::Error,
        Some("warning") => Severity::Warning,
        Some("info") => Severity::Info,
        _ => default_min_severity,
    };

    html_inspector_core::validate_events(
        source,
        rules,
        Config {
            message_order: MessageOrder::Vnu,
            also_check_css: params.also_check_css,
            min_severity,
            base_uri,
            csp_header: params.csp_header.clone(),
            ..Config::default()
        },
    )
}

fn apply_level_filter(report: &mut Report, level: Option<&str>) {
    match level {
        Some("error") => report
            .messages
            .retain(|m| matches!(m.severity, Severity::Error)),
        Some("warning") => report
            .messages
            .retain(|m| matches!(m.severity, Severity::Error | Severity::Warning)),
        _ => {}
    }
}

fn format_gnu(report: &Report) -> String {
    use std::fmt::Write as _;
    let mut out = String::new();
    for m in &report.messages {
        let (line, col) = m.span.map_or((0, 0), |s| (s.line, s.col));
        let sev = match m.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Info => "info",
        };
        let _ = write!(
            &mut out,
            "{}:{}:{}: {}: ",
            report.source_name, line, col, sev
        );
        write_single_line(&mut out, &m.message);
        out.push('\n');
    }
    out
}

fn write_single_line(out: &mut String, s: &str) {
    for (idx, part) in s.split('\n').enumerate() {
        if idx != 0 {
            out.push(' ');
        }
        out.push_str(part);
    }
}

fn write_response(
    stream: &mut TcpStream,
    status: u16,
    content_type: &str,
    body: &[u8],
) -> std::io::Result<()> {
    let reason = match status {
        200 => "OK",
        400 => "Bad Request",
        405 => "Method Not Allowed",
        413 => "Payload Too Large",
        500 => "Internal Server Error",
        _ => "OK",
    };
    write!(
        stream,
        "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nCache-Control: no-cache\r\nConnection: close\r\n\r\n",
        status,
        reason,
        content_type,
        body.len()
    )?;
    stream.write_all(body)?;
    stream.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use html_inspector_core::{Category, Message, Span};
    use std::io::{BufReader, Cursor};

    #[test]
    fn parse_query_decodes_pairs_and_skips_invalid_escapes() {
        let q = "a=b+c&bad=%ZZ&x=%2F&k";
        let pairs: Vec<(String, String)> = parse_query(q).collect();
        assert_eq!(
            pairs,
            vec![
                ("a".to_string(), "b c".to_string()),
                ("x".to_string(), "/".to_string()),
                ("k".to_string(), "".to_string()),
            ]
        );
    }

    #[test]
    fn parse_query_skips_pair_when_key_is_invalid_escape() {
        let q = "ba%ZZ=d&x=y";
        let pairs: Vec<(String, String)> = parse_query(q).collect();
        assert_eq!(pairs, vec![("x".to_string(), "y".to_string())]);
    }

    #[test]
    fn parse_query_ignores_empty_pairs() {
        let q = "a=b&&c=d&";
        let pairs: Vec<(String, String)> = parse_query(q).collect();
        assert_eq!(
            pairs,
            vec![
                ("a".to_string(), "b".to_string()),
                ("c".to_string(), "d".to_string()),
            ]
        );
    }

    #[test]
    fn parse_query_allows_empty_keys_and_values() {
        let q = "=x&y=";
        let pairs: Vec<(String, String)> = parse_query(q).collect();
        assert_eq!(
            pairs,
            vec![
                ("".to_string(), "x".to_string()),
                ("y".to_string(), "".to_string())
            ]
        );
    }

    #[test]
    fn params_from_query_and_headers_prefers_query_over_request_header() {
        let mut headers: FxHashMap<String, String> = FxHashMap::default();
        headers.insert(
            "content-security-policy".to_string(),
            "script-src 'self'".to_string(),
        );
        let p = Params::from_query_and_headers("csp_header=script-src%20%27none%27", &headers);
        assert_eq!(p.csp_header.as_deref(), Some("script-src 'none'"));
    }

    #[test]
    fn params_from_query_and_headers_uses_request_header_when_query_absent() {
        let mut headers: FxHashMap<String, String> = FxHashMap::default();
        headers.insert(
            "content-security-policy".to_string(),
            "script-src 'none'".to_string(),
        );
        let p = Params::from_query_and_headers("", &headers);
        assert_eq!(p.csp_header.as_deref(), Some("script-src 'none'"));
    }

    #[test]
    fn server_accepts_csp_from_http_header_when_query_param_absent() {
        let mut headers: FxHashMap<String, String> = FxHashMap::default();
        headers.insert(
            "content-security-policy".to_string(),
            "script-src 'none'".to_string(),
        );
        let params = Params::from_query_and_headers("", &headers);
        let report = validate_html_bytes(
            "x.html".to_string(),
            InputFormat::Html,
            b"<!doctype html><meta charset=utf-8><script>alert(1)</script>".to_vec(),
            &params,
            Severity::Info,
        )
        .unwrap();
        assert!(report.messages.iter().any(|m| {
            m.code == "html.csp.inline_script.blocked" && m.message.contains("(HTTP header)")
        }));
    }

    #[test]
    fn gnu_format_includes_location_and_severity() {
        let mut report = Report {
            source_name: "x.html".to_string(),
            messages: vec![Message::new(
                "x",
                Severity::Error,
                Category::Html,
                "oops",
                Some(Span::new(0, 1, 3, 4)),
            )],
        };
        apply_level_filter(&mut report, Some("error"));
        let s = format_gnu(&report);
        assert!(s.contains("x.html:3:4: error: oops"));
    }

    #[test]
    fn apply_level_filter_warning_keeps_errors_and_warnings() {
        let mut report = Report {
            source_name: "x.html".to_string(),
            messages: vec![
                Message::new("e", Severity::Error, Category::Html, "e", None),
                Message::new("w", Severity::Warning, Category::Html, "w", None),
                Message::new("i", Severity::Info, Category::Html, "i", None),
            ],
        };
        apply_level_filter(&mut report, Some("warning"));
        assert_eq!(report.messages.len(), 2);
        assert!(matches!(report.messages[0].severity, Severity::Error));
        assert!(matches!(report.messages[1].severity, Severity::Warning));
    }

    #[test]
    fn gnu_format_replaces_newlines_with_spaces() {
        let report = Report {
            source_name: "x.html".to_string(),
            messages: vec![Message::new(
                "x",
                Severity::Warning,
                Category::Html,
                "a\n\nb\n",
                None,
            )],
        };
        let s = format_gnu(&report);
        assert!(s.contains("x.html:0:0: warning: a  b "));
    }

    #[test]
    fn write_single_line_preserves_trailing_empty_segments() {
        let mut out = String::new();
        write_single_line(&mut out, "a\n");
        assert_eq!(out, "a ");

        out.clear();
        write_single_line(&mut out, "\n");
        assert_eq!(out, " ");
    }

    #[test]
    fn write_single_line_preserves_leading_newline_as_space() {
        let mut out = String::new();
        write_single_line(&mut out, "\nhello");
        assert_eq!(out, " hello");
    }

    #[test]
    fn write_single_line_leaves_single_line_untouched() {
        let mut out = String::new();
        write_single_line(&mut out, "hello");
        assert_eq!(out, "hello");
    }

    #[test]
    fn write_single_line_preserves_trailing_newline_as_space() {
        let mut out = String::new();
        write_single_line(&mut out, "hello\n");
        assert_eq!(out, "hello ");
    }

    #[test]
    fn write_single_line_handles_empty_string() {
        let mut out = String::new();
        write_single_line(&mut out, "");
        assert_eq!(out, "");
    }

    #[test]
    fn query_parses_known_params() {
        let p = Params::parse("out=json&level=warning&parser=html&doc=a.html&also_check_css=yes");
        assert_eq!(p.out.as_deref(), Some("json"));
        assert_eq!(p.level.as_deref(), Some("warning"));
        assert_eq!(p.parser.as_deref(), Some("html"));
        assert_eq!(p.doc.as_deref(), Some("a.html"));
        assert!(p.also_check_css);
    }

    #[test]
    fn query_ignores_empty_and_invalid_pairs() {
        let p = Params::parse("out=json&&doc=%ZZ&parser=html&");
        assert_eq!(p.out.as_deref(), Some("json"));
        assert_eq!(p.parser.as_deref(), Some("html"));
        assert_eq!(p.doc, None);
    }

    #[test]
    fn query_ignores_empty_keys() {
        let p = Params::parse("=x&out=json");
        assert_eq!(p.out.as_deref(), Some("json"));
    }

    #[test]
    fn percent_decode_handles_spaces() {
        assert_eq!(percent_decode("a+b%20c").unwrap(), "a b c");
    }

    #[test]
    fn percent_decode_returns_input_when_no_escapes() {
        assert_eq!(percent_decode("abcDEF123-_.*").unwrap(), "abcDEF123-_.*");
    }

    #[test]
    fn ends_with_ascii_ci_is_byte_based_and_non_panicking() {
        assert!(super::ends_with_ascii_ci("☃.xhtml", ".xhtml"));
        assert!(!super::ends_with_ascii_ci("é", "e"));
        assert!(!super::ends_with_ascii_ci("a", ".xhtml"));
    }

    #[test]
    fn split_path_query_handles_origin_form() {
        let (path, query) = split_path_query("/path/to/x?a=b&c=d");
        assert_eq!(path, "/path/to/x");
        assert_eq!(query, "a=b&c=d");

        let (path, query) = split_path_query("/path/to/x");
        assert_eq!(path, "/path/to/x");
        assert_eq!(query, "");
    }

    #[test]
    fn split_path_query_handles_absolute_form() {
        let (path, query) = split_path_query("http://example.com/path/to/x?a=b");
        assert_eq!(path, "/path/to/x");
        assert_eq!(query, "a=b");

        let (path, query) = split_path_query("http://example.com:8080/path/to/x?a=b");
        assert_eq!(path, "/path/to/x");
        assert_eq!(query, "a=b");

        let (path, query) = split_path_query("http://example.com/?a=b");
        assert_eq!(path, "/");
        assert_eq!(query, "a=b");

        let (path, query) = split_path_query("https://example.com/path/to/x");
        assert_eq!(path, "/path/to/x");
        assert_eq!(query, "");

        let (path, query) = split_path_query("https://example.com");
        assert_eq!(path, "/");
        assert_eq!(query, "");
    }

    #[test]
    fn detect_input_format_and_name_prefers_parser_and_content_type_over_doc_name() {
        let mut p = Params {
            doc: Some("INDEX.HTML".to_string()),
            parser: Some("xml".to_string()),
            ..Default::default()
        };
        let (format, name) = detect_input_format_and_name(&p, "");
        assert_eq!(format, InputFormat::Xhtml);
        assert_eq!(name, "INDEX.HTML");

        p.parser = None;
        let (format, _) = detect_input_format_and_name(&p, "application/xhtml+xml");
        assert_eq!(format, InputFormat::Xhtml);
    }

    #[test]
    fn detect_input_format_and_name_uses_doc_extension_case_insensitively() {
        let p = Params {
            doc: Some("Doc.XhTmL".to_string()),
            ..Params::default()
        };
        let (format, name) = detect_input_format_and_name(&p, "");
        assert_eq!(format, InputFormat::Xhtml);
        assert_eq!(name, "Doc.XhTmL");

        let p = Params {
            doc: Some("Doc.HTML".to_string()),
            ..Params::default()
        };
        let (format, _) = detect_input_format_and_name(&p, "");
        assert_eq!(format, InputFormat::Html);
    }

    #[test]
    fn detect_input_format_and_name_uses_xml_extension() {
        let p = Params {
            doc: Some("Doc.xml".to_string()),
            ..Params::default()
        };
        let (format, name) = detect_input_format_and_name(&p, "");
        assert_eq!(format, InputFormat::Xhtml);
        assert_eq!(name, "Doc.xml");
    }

    #[test]
    fn query_skips_invalid_percent_escapes_and_empty_segments() {
        let p = Params::parse("out=json&&doc=a.html&bad=%GG&%GG=x");
        assert_eq!(p.out.as_deref(), Some("json"));
        assert_eq!(p.doc.as_deref(), Some("a.html"));
    }

    #[test]
    fn read_chunked_body_decodes_basic_payload() {
        let payload = b"4\r\nWiki\r\n5\r\npedia\r\n0\r\n\r\n";
        let mut reader = BufReader::new(Cursor::new(payload));
        let body = read_chunked_body(&mut reader, 1024).unwrap();
        assert_eq!(body, b"Wikipedia");
    }

    #[test]
    fn read_chunked_body_ignores_trailers() {
        let payload = b"3\r\nhey\r\n0\r\nX-Foo: bar\r\n\r\n";
        let mut reader = BufReader::new(Cursor::new(payload));
        let body = read_chunked_body(&mut reader, 1024).unwrap();
        assert_eq!(body, b"hey");
    }

    #[test]
    fn read_chunked_body_ignores_trailers_with_lf_terminator() {
        let payload = b"3\r\nhey\r\n0\r\nX-Foo: bar\n\n";
        let mut reader = BufReader::new(Cursor::new(payload));
        let body = read_chunked_body(&mut reader, 1024).unwrap();
        assert_eq!(body, b"hey");
    }

    #[test]
    fn read_chunked_body_parses_chunk_extensions() {
        let payload = b"4;ext=value\r\nWiki\r\n0\r\n\r\n";
        let mut reader = BufReader::new(Cursor::new(payload));
        let body = read_chunked_body(&mut reader, 1024).unwrap();
        assert_eq!(body, b"Wiki");
    }

    #[test]
    fn read_chunked_body_rejects_invalid_chunk_terminator() {
        let payload = b"3\r\nhey\n0\r\n\r\n";
        let mut reader = BufReader::new(Cursor::new(payload));
        let err = read_chunked_body(&mut reader, 1024).unwrap_err();
        assert!(err.contains("chunk CRLF") || err.contains("chunk terminator"));
    }

    #[test]
    fn read_chunked_body_enforces_max_size() {
        let payload = b"4\r\nWiki\r\n0\r\n\r\n";
        let mut reader = BufReader::new(Cursor::new(payload));
        let err = read_chunked_body(&mut reader, 3).unwrap_err();
        assert!(err.contains("body too large"));
    }

    #[test]
    fn read_request_parses_headers_and_fixed_length_body() {
        let mut reader = BufReader::new(Cursor::new(
            b"POST /submit?x=1 HTTP/1.1\r\n\
Host: example\r\n\
Content-Type: text/plain\r\n\
Content-Length: 4\r\n\
X-Test: a\r\n\
\r\n\
body",
        ));
        let req = read_request_from(&mut reader, 1024).unwrap();

        assert_eq!(req.method, "POST");
        assert_eq!(req.target, "/submit?x=1");
        assert_eq!(req.headers.get("host").map(String::as_str), Some("example"));
        assert_eq!(
            req.headers.get("content-type").map(String::as_str),
            Some("text/plain")
        );
        assert_eq!(req.headers.get("x-test").map(String::as_str), Some("a"));
        assert_eq!(req.body, b"body");
    }

    #[test]
    fn read_request_head_detects_100_continue_expectation_case_insensitively() {
        let mut reader = BufReader::new(Cursor::new(
            b"POST / HTTP/1.1\r\nExpect: 100-CONTINUE\r\nContent-Length: 0\r\n\r\n",
        ));
        let head = read_request_head(&mut reader).unwrap();
        assert!(head.expect_continue);
    }

    #[test]
    fn transfer_encoding_chunked_matches_exact_value_case_insensitively() {
        let mut headers: FxHashMap<String, String> = FxHashMap::default();
        assert!(!transfer_encoding_is_chunked(&headers));

        headers.insert("transfer-encoding".to_string(), "chunked".to_string());
        assert!(transfer_encoding_is_chunked(&headers));

        headers.insert("transfer-encoding".to_string(), "CHUNKED".to_string());
        assert!(transfer_encoding_is_chunked(&headers));

        headers.insert("transfer-encoding".to_string(), "gzip, chunked".to_string());
        assert!(!transfer_encoding_is_chunked(&headers));
    }

    #[test]
    fn read_request_body_enforces_max_content_length() {
        let mut reader = BufReader::new(Cursor::new(b"1234"));
        let mut headers: FxHashMap<String, String> = FxHashMap::default();
        headers.insert("content-length".to_string(), "4".to_string());
        let err = read_request_body(&mut reader, &headers, 3).unwrap_err();
        assert_eq!(err, "body too large");
    }

    #[test]
    fn read_request_body_prefers_chunked_over_content_length() {
        let mut reader = BufReader::new(Cursor::new(b"4\r\nWiki\r\n0\r\n\r\n"));
        let mut headers: FxHashMap<String, String> = FxHashMap::default();
        headers.insert("transfer-encoding".to_string(), "chunked".to_string());
        headers.insert("content-length".to_string(), "999".to_string());
        let body = read_request_body(&mut reader, &headers, 1024).unwrap();
        assert_eq!(body, b"Wiki");
    }

    #[test]
    fn ui_html_mentions_compare_endpoint_and_is_html() {
        let html = ui_html();
        assert!(html.contains("<!doctype html>"));
        assert!(html.contains("/api/compare"));
        assert!(html.contains("Use basic authentication"));
    }

    #[test]
    fn basic_auth_header_value_matches_known_example() {
        assert_eq!(
            super::basic_auth_header_value("Aladdin", "open sesame"),
            "Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ=="
        );
    }

    #[test]
    fn parse_http_base_url_accepts_host_port_and_path() {
        let (host, port, path) = parse_http_base_url("http://127.0.0.1:8889/").unwrap();
        assert_eq!(host, "127.0.0.1");
        assert_eq!(port, 8889);
        assert_eq!(path, "/");

        let (host, port, path) = parse_http_base_url("http://example.com:80/vnu").unwrap();
        assert_eq!(host, "example.com");
        assert_eq!(port, 80);
        assert_eq!(path, "/vnu");
    }

    #[test]
    fn read_response_decodes_chunked_body() {
        let mut r = std::io::Cursor::new(
            b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n4\r\nWiki\r\n5\r\npedia\r\n0\r\n\r\n"
                .to_vec(),
        );
        let resp = read_response(&mut r, 1024).unwrap();
        assert_eq!(resp.status, 200);
        assert_eq!(resp.body, b"Wikipedia");
    }
}
