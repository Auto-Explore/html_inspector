# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

## [0.2.3]

### Added

- `html_inspector_cli`: more invalid-HTML fixtures (including CSS issue highlighting and a larger ARIA test case).

### Fixed

- `html_inspector_rules_css`: CSS errors in `style="..."` attributes now point at the attribute/value span (instead of the tag), improving issue highlighting.
- `html_inspector_rules_html`: duplicate `id` errors now point at the `id` attribute span (instead of the tag), when available.

## [0.2.2]

### Added

- `html_inspector_cli`: fixture-driven invalid HTML/XHTML tests (`tests/invalid_html_fixtures`) with an auto-generated `*.expected.json` on first run.

### Fixed

- `html_inspector_html` (`html5ever` backend): propagate attribute spans into emitted events so rules can point at specific attributes.
- `html_inspector_rules_aria`: use attribute spans when reporting invalid ARIA attributes / missing `idref` targets (points at the attribute, not the tag).

## [0.1.0]

Initial public release.

### Added

- `html_inspector_cli`: validate HTML/XHTML and CSS, plus an optional local HTTP server + browser UI.
- `html_inspector_html`: HTML event sources (lightweight scanner by default; `html5ever` backend behind a feature).
- Rule packs: `html_inspector_rules_html`, `html_inspector_rules_aria`, `html_inspector_rules_i18n`, `html_inspector_rules_css`.
- CSS validation engine: `css_inspector` (also exposed via `html_inspector_css`).
