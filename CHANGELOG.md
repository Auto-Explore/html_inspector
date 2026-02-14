# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

## [0.1.0]

Initial public release.

### Added

- `html_inspector_cli`: validate HTML/XHTML and CSS, plus an optional local HTTP server + browser UI.
- `html_inspector_html`: HTML event sources (lightweight scanner by default; `html5ever` backend behind a feature).
- Rule packs: `html_inspector_rules_html`, `html_inspector_rules_aria`, `html_inspector_rules_i18n`, `html_inspector_rules_css`.
- CSS validation engine: `css_inspector` (also exposed via `html_inspector_css`).
