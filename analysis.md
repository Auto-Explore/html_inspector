# Repository audit (missing / external resources)

This repo has been reorganized to a flat workspace at the repo root (`Cargo.toml`, `crates/*`, `scripts/*`) and now vendors the upstream VNU `tests/` tree directly.

## VNU suite inputs + manifest

- Suite inputs: `tests/` (vendored from upstream VNU)
- Suite manifest (fixtures metadata): `tests/manifest.jsonl`
  - Regenerate: `python3 scripts/generate_vnu_manifest.py`
  - Current generator behavior: emits a **curated “valid-only” manifest** (skips upstream `invalid/` + `*-novalid*` cases) and asserts `max_errors=0` for each included case.

## Vendored upstream tooling references

The vendored `tests/` tree still contains upstream scripts/docs that reference tooling that is *not* shipped in this repo (e.g. `checker.py`). Examples:

- `tests/Makefile` references `checker.py jar`
- `tests/schema-validation/README.md` references `./checker.py check`

## CSS validator (external repo)

`ae_css_validator` (the old in-tree CSS workspace/resources) is not part of this repo anymore.

- This repo’s HTML validator uses a CSS backend via `crates/html_inspector_css` (a wrapper crate).
- The current workspace dependency is `css_inspector` (path dependency to a sibling repo):
  - `Cargo.toml` → `css_inspector = { path = "../css_inspector/crates/css_inspector" }`

For publishing as a standalone open-source repo, you’ll want to replace that path dependency with a crates.io (or git) dependency once `css_inspector` / `css_inspector_core` is published.

Planned crate renames (per request):

- `html_inspector_css` → `css_inspector`
- `ae_css_validator_core` → `css_inspector_core`

## vnu.jar (not vendored)

- This repo does not include the jar.
- The Rust server UI can optionally compare results against a running Java vnu server if you provide a base URL.

## Open-source hygiene

- No compile-time embedded resources were found (`include_str!`/`include_bytes!`/`include!`/`#[path="..."]`).
- There is no top-level `LICENSE`/`COPYING` file even though the workspace declares `license = "MIT"` in `Cargo.toml`.
