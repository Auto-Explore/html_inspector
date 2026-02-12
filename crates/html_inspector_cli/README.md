# html_inspector_cli (Rust) — CLI + local web UI

`html_inspector_cli` is the Rust validator front-end for this repo’s Rust rule packs.
It can validate HTML/XHTML, validate CSS, and start a small local web UI for
interactive validation and Rust-vs-Java comparisons.

Most commands are intended to be run from the repo root.

## Requirements

- Rust toolchain (`cargo`)
- Java 11+ (only needed for Java parity/compare workflows)
- Python 3 (only needed for updating datasets/resources)

## Commands

All commands are invoked as:

```bash
cargo run -p html_inspector_cli -- [--min-severity error|warning|info] <command> [args...]
```

Global options:

- `--min-severity <error|warning|info>`: only include messages at/above this level (and skip running rules that can never emit at/above it). Default: `warning`.

### `file` — validate an HTML/XHTML file

```bash
cargo run -p html_inspector_cli -- file path/to/doc.html
cargo run -p html_inspector_cli -- file path/to/doc.xhtml xhtml
```

Options:

- `html|xhtml` (positional): force input format (default: `html`)
- `--also-check-css`: validate inline CSS in `<style>` and `style=""`
- `--csp-header <value>`: best-effort CSP enforcement/warnings

### `check` — auto-detect by extension/URI

Detects based on the input suffix:

- `*.css` → CSS validation (`css` command)
- otherwise → HTML validation (like `file`)

```bash
cargo run -p html_inspector_cli -- check path/to/doc.html
cargo run -p html_inspector_cli -- check path/to/doc.xhtml xhtml
cargo run -p html_inspector_cli -- check path/to/styles.css
```

### `css` — validate a CSS file or URI

```bash
cargo run -p html_inspector_cli -- css path/to/styles.css
cargo run -p html_inspector_cli -- css https://example.test/styles.css --allow-network
```

Options:

- `--profile <css1|css2|css21|css3|css3svg|svg|svgbasic|svgtiny>` (default: `css3svg`)
- `--medium <name>` (default: `all`)
- `--warning <n>` (default: `-1` like vnu.jar)
- `--allow-network`: required to fetch `http(s)` URIs

### `serve` — start the Rust HTTP server + browser UI

```bash
cargo run -p html_inspector_cli -- serve 3090 --bind 127.0.0.1
```

Options:

- `[port]` positional (default: `8888`)
- `--bind <ip>` (default: `127.0.0.1`)
- `--max-bytes <n>`: request body limit (default: 20 MiB)

The server:

- Serves a small browser UI at `http://<bind>:<port>/`
- Exposes an API compatible with vnu-client-style POSTs:
  `POST /?out=json|gnu&parser=html|xml&level=error|warning&doc=name`
- Provides UI helper endpoints: `POST /api/rust` and `POST /api/compare`
- Health: `GET /health`

## Rust UI vs Java UI

### Start the Rust UI

```bash
cargo run -p html_inspector_cli -- serve 3090 --bind 127.0.0.1
```

Open `http://127.0.0.1:3090/`.

### Start a Java vnu server (for comparisons)

Use different ports for Rust and Java if you run them side-by-side.

If you already have `build/dist/vnu.jar`, you can run the Java server directly:

```bash
java -cp build/dist/vnu.jar nu.validator.servlet.Main 8889
```

Then in the Rust UI, set “Java base URL” to `http://127.0.0.1:8889/` and use
“Compare (Rust vs Java)”.

If you don’t have the jar yet, obtain/build it separately (outside this repo) and place it at `build/dist/vnu.jar`.

## Updating the portable suite manifest

The repo vendors upstream VNU tests under `tests/`. `tests/manifest.jsonl` is a
metadata manifest for those fixtures.

Regenerate the manifest:

```bash
python3 scripts/generate_vnu_manifest.py
```

## Testing

Run tests:

```bash
cargo test -p html_inspector_cli
```
