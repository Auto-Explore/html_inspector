# Open-sourcing checklist (html_inspector)

This file captures repo-specific items to address before making this project public.

## Current findings (from this working tree)

- **Git history is effectively empty:** `git log` shows only one commit (`84492ef`), but the working tree has ~5.8k staged additions and some staged entries that are *missing on disk* (`AD` in `git status`).
- **Build is not standalone:** the workspace depends on a sibling path dependency: `css_inspector = { path = "../css_inspector/crates/css_inspector" }` in `Cargo.toml`.
- **Top-level license exists:** the workspace declares `license = "MIT"` in `Cargo.toml`, and the repo includes a top-level `LICENSE` file.
- **Large local build artifacts exist:** `target/` is ~7.3 GiB (ignored by git, but easy to accidentally ship in a zip/tarball).
- **Vendored upstream fixtures exist:** `tests/` contains the upstream VNU test suite tree plus large third-party fixture content; only a single license text was found under `tests/langdetect/`.

## Before making the repo public

### 1) Clean up source control state

- [ ] Get to a clean index + working tree (`git status` should be empty) before publishing a “first public commit”.
- [ ] Remove any staged-but-missing paths (the `AD` entries). A safe reset that does **not** delete files is:
  - `git reset` (resets the index to `HEAD`)
  - then re-add what you actually want: `git add -A`
- [ ] Ensure no build artifacts are tracked (keep `target/`, `__pycache__/`, etc. ignored).
- [ ] Delete local artifacts before publishing a source archive: `rm -rf target`.

### 2) Make the build work for strangers

- [ ] Replace the external path dependency on `../css_inspector/...` with one of:
  - a crates.io dependency (preferred once published),
  - a git dependency, or
  - an optional feature (so `cargo build` works without CSS support).
- [ ] Decide whether to commit `Cargo.lock`:
  - For a CLI/binary, committing `Cargo.lock` is recommended for reproducible builds.
  - If you choose to commit it, remove `Cargo.lock` from `.gitignore`.
- [ ] Document prerequisites and minimum versions (Rust MSRV; optional Java/Python requirements).

### 3) Licensing and third-party notices (high priority)

- [x] Add a top-level `LICENSE` file consistent with `Cargo.toml` (currently `MIT`).
- [ ] Audit and document third-party materials you ship in-tree, especially:
  - `tests/` (vendored VNU suite, plus embedded third-party fixture content like JS and large HTML pages),
  - `crates/markup5ever_rcdom` (contains upstream html5ever/markup5ever_rcdom code; ensure you comply with its MIT/Apache terms and include required notices/files).
- [ ] Add one of:
  - `THIRD_PARTY_NOTICES.md`, or
  - a `LICENSES/` directory with the relevant upstream license texts + attribution notes.
- [ ] Ensure the repo-level licensing statement matches reality (your code can be MIT, while vendored fixtures remain under their original licenses).

### 4) Project metadata and expectations

- [ ] Update `README.md` with:
  - what the project is / isn’t (runtime-risk vs spec conformance is great—keep it),
  - install/build steps from a fresh clone,
  - a short “License” section pointing to `LICENSE`,
  - a note about optional CSS validation (until the dependency is published),
  - a note about the vendored `tests/` suite (source + licensing pointer).
- [ ] Add `CONTRIBUTING.md` (how to run tests, style, how to submit PRs).
- [ ] Add `SECURITY.md` (private vulnerability reporting channel).
- [ ] Consider adding `CODE_OF_CONDUCT.md` if you expect external contributions.

### 5) Basic automation (recommended)

- [ ] Add CI (e.g. GitHub Actions) for at least:
  - `cargo fmt --check`
  - `cargo test --workspace`
  - `cargo clippy --workspace` (optional but useful)
- [ ] Optionally add license checking (`cargo deny`) once third-party licensing is clarified.

## Quick pre-public checks

- `git status --porcelain=v1` is empty
- `rg -n --hidden -S "(api[_-]?key|secret|token|password)"` has no surprising hits outside `tests/`
- `cargo test --workspace` passes from a clean clone (no sibling repos required)
