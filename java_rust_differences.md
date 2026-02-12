# Java vs Rust (html_inspector) rule differences

This repository contains the Rust implementation (the `html_inspector_*` crates under `crates/`).

## Fixtures

This repo vendors a curated subset of upstream VNU fixtures for parity/coverage work.

- Suite inputs: `tests/`
- Suite manifest: `tests/manifest.jsonl` (regenerate with `python3 scripts/generate_vnu_manifest.py`)
- Comparison workflow: run `cargo run -p html_inspector_cli -- serve ...` and compare against a running Java vnu server.

## Divergences list (rule + test)

- Extra/disallowed attributes are downgraded from `error` to `warning` in the Rust validator (messages like `Attribute “foo” not allowed on element “div” at this point.`). Documented in `README.md` and covered by `crates/html_inspector_rules_html/src/rules/tests.rs`.
