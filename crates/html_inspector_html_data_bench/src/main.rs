use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

#[cfg(unix)]
use std::sync::atomic::{AtomicI32, Ordering};

#[cfg(unix)]
static JAVA_PGID: AtomicI32 = AtomicI32::new(0);
#[cfg(unix)]
static RUST_PGID: AtomicI32 = AtomicI32::new(0);

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

#[derive(Clone)]
struct Doc {
    content_type: &'static str,
    bytes: Vec<u8>,
}

#[cfg(not(unix))]
struct ChildGuard(std::process::Child);

#[cfg(not(unix))]
impl ChildGuard {
    fn new(child: std::process::Child) -> Self {
        Self(child)
    }
}

#[cfg(unix)]
struct ChildGuard {
    child: std::process::Child,
    pgid_slot: &'static AtomicI32,
}

#[cfg(unix)]
impl ChildGuard {
    fn new(child: std::process::Child, pgid_slot: &'static AtomicI32) -> Self {
        if let Ok(pid) = i32::try_from(child.id()) {
            pgid_slot.store(pid, Ordering::Relaxed);
        }
        Self { child, pgid_slot }
    }
}

#[cfg(not(unix))]
impl Drop for ChildGuard {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

#[cfg(unix)]
impl Drop for ChildGuard {
    fn drop(&mut self) {
        if let Ok(pid) = i32::try_from(self.child.id()) {
            unsafe {
                let _ = libc::kill(-pid, libc::SIGKILL);
            }
        }
        self.pgid_slot.store(0, Ordering::Relaxed);
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn host_header(connect_ip: IpAddr, port: u16) -> String {
    match connect_ip {
        IpAddr::V4(v4) => format!("{v4}:{port}"),
        IpAddr::V6(v6) => format!("[{v6}]:{port}"),
    }
}

fn http_get(
    connect_ip: IpAddr,
    port: u16,
    path: &str,
    timeout: Duration,
) -> std::io::Result<String> {
    let addr = SocketAddr::new(connect_ip, port);
    let mut s = TcpStream::connect_timeout(&addr, timeout)?;
    s.set_read_timeout(Some(timeout))?;
    s.set_write_timeout(Some(timeout))?;
    write!(
        s,
        "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        path,
        host_header(connect_ip, port),
    )?;
    s.flush()?;
    let mut buf = Vec::new();
    s.read_to_end(&mut buf)?;
    Ok(String::from_utf8_lossy(&buf).to_string())
}

fn wait_http_ready(
    connect_ip: IpAddr,
    port: u16,
    path: &str,
    timeout: Duration,
) -> Result<(), String> {
    let start = Instant::now();
    while start.elapsed() < timeout {
        match http_get(connect_ip, port, path, Duration::from_millis(500)) {
            Ok(resp) if resp.starts_with("HTTP/1.1 200") => return Ok(()),
            Ok(_) => {}
            Err(_) => {}
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    Err(format!(
        "timeout waiting for http://{}{}",
        host_header(connect_ip, port),
        path
    ))
}

fn wait_http_responding(
    connect_ip: IpAddr,
    port: u16,
    path: &str,
    timeout: Duration,
) -> Result<(), String> {
    let start = Instant::now();
    while start.elapsed() < timeout {
        match http_get(connect_ip, port, path, Duration::from_millis(500)) {
            Ok(resp) if resp.starts_with("HTTP/") => return Ok(()),
            Ok(_) => {}
            Err(_) => {}
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    Err(format!(
        "timeout waiting for http://{}{} to respond",
        host_header(connect_ip, port),
        path
    ))
}

fn http_post(
    connect_ip: IpAddr,
    port: u16,
    query: &str,
    content_type: &str,
    body: &[u8],
    timeout: Duration,
) -> Result<(), String> {
    let addr = SocketAddr::new(connect_ip, port);
    let mut s = TcpStream::connect_timeout(&addr, timeout).map_err(|e| e.to_string())?;
    s.set_read_timeout(Some(timeout))
        .map_err(|e| e.to_string())?;
    s.set_write_timeout(Some(timeout))
        .map_err(|e| e.to_string())?;
    write!(
        s,
        "POST /?{} HTTP/1.1\r\nHost: {}\r\nUser-Agent: ae-agent/0.1\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        query,
        host_header(connect_ip, port),
        content_type,
        body.len()
    )
    .map_err(|e| e.to_string())?;
    s.write_all(body).map_err(|e| e.to_string())?;
    s.flush().map_err(|e| e.to_string())?;
    let mut resp = Vec::new();
    s.read_to_end(&mut resp).map_err(|e| e.to_string())?;
    if resp.starts_with(b"HTTP/1.1 200") || resp.starts_with(b"HTTP/1.0 200") {
        return Ok(());
    }
    let first_line = resp.split(|&b| b == b'\n').next().unwrap_or_default();
    Err(format!(
        "non-200 response from {}: {}",
        host_header(connect_ip, port),
        String::from_utf8_lossy(first_line).trim()
    ))
}

fn run_server_once(
    connect_ip: IpAddr,
    port: u16,
    out: &str,
    level: &str,
    docs: &[Doc],
) -> Result<Duration, String> {
    let start = Instant::now();
    for doc in docs {
        let query = format!("out={}&level={}", out, level);
        http_post(
            connect_ip,
            port,
            &query,
            doc.content_type,
            &doc.bytes,
            Duration::from_secs(30),
        )?;
    }
    Ok(start.elapsed())
}

fn ensure_rust_cli_built(workspace_root: &Path, release: bool) -> Result<PathBuf, String> {
    let target_dir = workspace_root.join("target");
    let bin = if release {
        target_dir.join("release").join("html_inspector_cli")
    } else {
        target_dir.join("debug").join("html_inspector_cli")
    };
    if bin.exists() {
        return Ok(bin);
    }

    let mut cmd = Command::new("cargo");
    cmd.current_dir(workspace_root)
        .arg("build")
        .arg("--offline")
        .arg("-p")
        .arg("html_inspector_cli");
    if release {
        cmd.arg("--release");
    }
    let status = cmd.status().map_err(|e| format!("run cargo build: {e}"))?;
    if !status.success() {
        return Err(format!("cargo build failed: {status}"));
    }
    if !bin.exists() {
        return Err(format!("missing rust server binary at {}", bin.display()));
    }
    Ok(bin)
}

fn start_rust_server(
    workspace_root: &Path,
    bind: IpAddr,
    port: u16,
    release: bool,
    quiet: bool,
) -> Result<ChildGuard, String> {
    let bin = ensure_rust_cli_built(workspace_root, release)?;
    let mut cmd = Command::new(bin);
    cmd.arg("serve")
        .arg(port.to_string())
        .arg("--bind")
        .arg(bind.to_string());
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        unsafe {
            cmd.pre_exec(|| {
                if libc::setpgid(0, 0) != 0 {
                    return Err(std::io::Error::last_os_error());
                }
                #[cfg(target_os = "linux")]
                {
                    let _ = libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGKILL);
                }
                Ok(())
            });
        }
    }
    if quiet {
        cmd.stdout(Stdio::null()).stderr(Stdio::null());
    }
    cmd.spawn()
        .map(|c| {
            #[cfg(unix)]
            return ChildGuard::new(c, &RUST_PGID);
            #[cfg(not(unix))]
            return ChildGuard::new(c);
        })
        .map_err(|e| format!("spawn rust server: {e}"))
}

fn start_java_server(
    jar_path: &Path,
    bind: IpAddr,
    port: u16,
    level: &str,
    quiet: bool,
) -> Result<ChildGuard, String> {
    let java_bin = if Path::new("/usr/bin/java").exists() {
        Path::new("/usr/bin/java")
    } else {
        Path::new("java")
    };
    let mut cmd = Command::new(java_bin);
    cmd.arg(format!("-Dnu.validator.client.level={level}"))
        .arg(format!("-Dnu.validator.servlet.bind-address={bind}"))
        .arg("-cp")
        .arg(jar_path)
        .arg("nu.validator.servlet.Main")
        .arg(port.to_string());
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        unsafe {
            cmd.pre_exec(|| {
                if libc::setpgid(0, 0) != 0 {
                    return Err(std::io::Error::last_os_error());
                }
                #[cfg(target_os = "linux")]
                {
                    let _ = libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGKILL);
                }
                Ok(())
            });
        }
    }
    if quiet {
        cmd.stdout(Stdio::null()).stderr(Stdio::null());
    }
    cmd.spawn()
        .map(|c| {
            #[cfg(unix)]
            return ChildGuard::new(c, &JAVA_PGID);
            #[cfg(not(unix))]
            return ChildGuard::new(c);
        })
        .map_err(|e| format!("spawn java server: {e}"))
}

#[cfg(unix)]
unsafe extern "C" fn handle_term_signal(sig: i32) { unsafe {
    let java_pgid = JAVA_PGID.load(Ordering::Relaxed);
    if java_pgid > 0 {
        let _ = libc::kill(-java_pgid, libc::SIGKILL);
    }
    let rust_pgid = RUST_PGID.load(Ordering::Relaxed);
    if rust_pgid > 0 {
        let _ = libc::kill(-rust_pgid, libc::SIGKILL);
    }
    libc::_exit(128 + sig);
}}

#[cfg(unix)]
fn install_signal_handlers() -> Result<(), String> {
    unsafe {
        let mut ign: libc::sigaction = std::mem::zeroed();
        ign.sa_sigaction = libc::SIG_IGN;
        libc::sigemptyset(&mut ign.sa_mask);
        ign.sa_flags = 0;
        if libc::sigaction(libc::SIGPIPE, &ign, std::ptr::null_mut()) != 0 {
            return Err(format!(
                "sigaction(SIGPIPE): {}",
                std::io::Error::last_os_error()
            ));
        }

        let mut act: libc::sigaction = std::mem::zeroed();
        act.sa_sigaction = handle_term_signal as *const () as usize;
        libc::sigemptyset(&mut act.sa_mask);
        act.sa_flags = 0;

        for sig in [libc::SIGINT, libc::SIGTERM, libc::SIGHUP] {
            if libc::sigaction(sig, &act, std::ptr::null_mut()) != 0 {
                return Err(format!(
                    "sigaction({sig}): {}",
                    std::io::Error::last_os_error()
                ));
            }
        }
    }
    Ok(())
}

#[cfg(unix)]
fn install_panic_hook() {
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let java_pgid = JAVA_PGID.load(Ordering::Relaxed);
        if java_pgid > 0 {
            unsafe {
                let _ = libc::kill(-java_pgid, libc::SIGKILL);
            }
        }
        let rust_pgid = RUST_PGID.load(Ordering::Relaxed);
        if rust_pgid > 0 {
            unsafe {
                let _ = libc::kill(-rust_pgid, libc::SIGKILL);
            }
        }
        default_hook(info);
    }));
}

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if arg_flag(&args, "--help") {
        eprintln!(
            "usage: html_inspector_html_data_bench [--iterations N] [--warmup N] [--java-only|--rust-only] [--out gnu|json] [--level error|warning] [--html-dir PATH] [--recursive]\n\
             \n\
             Spawns both a Java vnu server and a Rust html_inspector server, then benchmarks HTTP calls to them.\n\
             Startup cost of servers is excluded (servers are started once, then benchmark loops begin).\n\
             \n\
             Options:\n\
             - --java-jar PATH   (default: build/dist/vnu.jar)\n\
             - --bind IP         (default: 127.0.0.1)\n\
             - --connect-ip IP   (default: --bind; but if --bind is 0.0.0.0 then 127.0.0.1)\n\
             - --java-port P     (default: 3090)\n\
             - --rust-port P     (default: 3091)\n\
             - --limit-files N   (default: all)\n\
             - --html-dir PATH   (default: tests/html)\n\
             - --recursive       (recurse into subdirectories of --html-dir)\n\
             - --release         (build/run Rust server in release mode)\n\
             - --keep-logs       (do not suppress server stdout/stderr)"
        );
        return Ok(());
    }

    #[cfg(unix)]
    {
        install_signal_handlers()?;
        install_panic_hook();
    }

    let iterations: usize = arg_value(&args, "--iterations")
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);
    let warmup: usize = arg_value(&args, "--warmup")
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);
    let java_only = arg_flag(&args, "--java-only");
    let rust_only = arg_flag(&args, "--rust-only");
    let limit_files: Option<usize> = arg_value(&args, "--limit-files").and_then(|s| s.parse().ok());
    let out_format = arg_value(&args, "--out").unwrap_or_else(|| "gnu".to_string());
    let level = arg_value(&args, "--level").unwrap_or_else(|| "error".to_string());
    let release = arg_flag(&args, "--release");
    let keep_logs = arg_flag(&args, "--keep-logs");
    let recursive = arg_flag(&args, "--recursive");

    let bind: IpAddr = arg_value(&args, "--bind")
        .and_then(|s| s.parse().ok())
        .unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    let connect_ip: IpAddr = arg_value(&args, "--connect-ip")
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| {
            if bind.is_unspecified() {
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))
            } else {
                bind
            }
        });
    let java_port: u16 = arg_value(&args, "--java-port")
        .and_then(|s| s.parse().ok())
        .unwrap_or(3090);
    let rust_port: u16 = arg_value(&args, "--rust-port")
        .and_then(|s| s.parse().ok())
        .unwrap_or(3091);
    if out_format != "gnu" && out_format != "json" {
        return Err(format!("unsupported --out {}; use gnu or json", out_format));
    }
    if level != "error" && level != "warning" {
        return Err(format!(
            "unsupported --level {}; use error or warning",
            level
        ));
    }

    let root = arg_value(&args, "--html-dir")
        .map(PathBuf::from)
        .unwrap_or_else(|| repo_root().join("tests").join("html"));
    let mut files = collect_html_files(&root, recursive);
    if let Some(n) = limit_files {
        files.truncate(n);
    }
    if files.is_empty() {
        return Err(format!("no html files found in {}", root.display()));
    }

    let jar_path = arg_value(&args, "--java-jar")
        .map(PathBuf::from)
        .unwrap_or_else(|| repo_root().join("build").join("dist").join("vnu.jar"));
    if !jar_path.exists() {
        return Err(format!("missing vnu.jar at {}", jar_path.display()));
    }

    let mut docs: Vec<Doc> = Vec::with_capacity(files.len());
    let mut total_bytes = 0usize;
    for p in &files {
        let bytes = std::fs::read(p).unwrap_or_default();
        let ext = p
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        let content_type = if matches!(ext.as_str(), "xhtml" | "xht" | "xml") {
            "application/xhtml+xml; charset=utf-8"
        } else {
            "text/html; charset=utf-8"
        };
        total_bytes += bytes.len();
        docs.push(Doc {
            content_type,
            bytes,
        });
    }

    let quiet = !keep_logs;

    let _java_server = if rust_only {
        None
    } else {
        let child = start_java_server(&jar_path, bind, java_port, &level, quiet)?;
        wait_http_responding(connect_ip, java_port, "/", Duration::from_secs(15))
            .map_err(|e| format!("java server not ready: {e}"))?;
        Some(child)
    };

    let _rust_server = if java_only {
        None
    } else {
        let child = start_rust_server(&repo_root(), bind, rust_port, release, quiet)?;
        wait_http_ready(connect_ip, rust_port, "/health", Duration::from_secs(15))
            .map_err(|e| format!("rust server not ready: {e}"))?;
        Some(child)
    };

    let mut java_times: Vec<Duration> = Vec::new();
    let mut rust_times: Vec<Duration> = Vec::new();

    for _ in 0..warmup {
        if !rust_only {
            let _ = run_server_once(connect_ip, java_port, &out_format, &level, &docs)
                .map_err(|e| format!("java request failed: {e}"))?;
        }
        if !java_only {
            let _ = run_server_once(connect_ip, rust_port, &out_format, &level, &docs)
                .map_err(|e| format!("rust request failed: {e}"))?;
        }
    }

    for _ in 0..iterations {
        if !rust_only {
            java_times.push(
                run_server_once(connect_ip, java_port, &out_format, &level, &docs)
                    .map_err(|e| format!("java request failed: {e}"))?,
            );
        }
        if !java_only {
            rust_times.push(
                run_server_once(connect_ip, rust_port, &out_format, &level, &docs)
                    .map_err(|e| format!("rust request failed: {e}"))?,
            );
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

    println!("corpus: files={} bytes={}", files.len(), total_bytes);
    println!(
        "http: out={} level={} bind={} connect_ip={} java_port={} rust_port={}",
        out_format, level, bind, connect_ip, java_port, rust_port
    );
    if let Some((min, med, mean)) = stats(&java_times) {
        println!("java: min={:?} med={:?} mean={:?}", min, med, mean);
    }
    if let Some((min, med, mean)) = stats(&rust_times) {
        println!("rust: min={:?} med={:?} mean={:?}", min, med, mean);
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
