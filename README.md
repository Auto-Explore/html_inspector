# HTML Inspector

Rust HTML validator focused on runtime risk (DOM correctness + interaction stability), not strict spec conformance.

MSRV: Rust 1.93 (Edition 2024).

- VNU test suite inputs: `tests/`
- VNU manifest (fixtures metadata): `tests/manifest.jsonl` (regenerate with `python3 scripts/generate_vnu_manifest.py`)
- CSS validation backend: `css_inspector` (crates.io dependency)

## Install

From crates.io:

`cargo install html_inspector_cli`

From a git checkout:

`cargo install --path crates/html_inspector_cli`

## Project goal

Project goal: prioritize validation findings by severity, so the highest severity is reserved for issues that are likely to break real-world behavior as opposed to being strictly spec compliant.

This project is written in Rust to integrate easily with the existing Rust ecosystem.

## Severity model

- `error`: high probability of incorrect behavior (DOM tree corruption, broken interactive semantics, duplicate/conflicting IDs, ARIA that contradicts native semantics)
- `warning`: works today but brittle (recoverable invalid HTML, deprecated markup, non-fatal a11y issues)
- `info`: signal/noise (framework-generated markers, minor spec purism)

The CLI defaults to `--min-severity warning` (errors + warnings). Use `--min-severity info` to include everything.

## Run

Validate a single file:

`cargo run -p html_inspector_cli -- file path/to/doc.html`

Validate and only report errors:

`cargo run -p html_inspector_cli -- --min-severity error file path/to/doc.html`

Validate HTML and also check CSS in `<style>` and `style=""`:

`cargo run -p html_inspector_cli -- file path/to/doc.html --also-check-css`

Validate a CSS file (delegates to the Rust CSS validator port):

`cargo run -p html_inspector_cli -- css path/to/styles.css`

Auto-detect by extension/URI:

`cargo run -p html_inspector_cli -- check path/or/uri`

Run as an HTTP server (vnu-client-style POST-body requests; supports `out=json|gnu`, `level=error|warning`, `parser=html|xml`, and `doc=`):

`cargo run -p html_inspector_cli -- serve 3090 --bind 127.0.0.1`

The server also serves a small browser UI at `/` for validating and comparing results against a running Java vnu checker:

- UI: `http://127.0.0.1:3090/`
- API (Rust): `POST /?out=json|gnu&parser=html|xml&level=error|warning&doc=name`
- API (UI helper): `POST /api/rust` and `POST /api/compare`

For a more complete CLI command reference, see `crates/html_inspector_cli/README.md`.

## Coverage

Generate Rust test coverage (requires `cargo-llvm-cov`):

`./scripts/coverage.sh`

## Prior work

This project builds on prior work by W3C Nu Validator: https://github.com/validator/validator
