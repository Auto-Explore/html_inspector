# html/semantics/document-metadata/the-meta-element/color-scheme/meta-color-scheme-remove.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-meta-element/color-scheme/meta-color-scheme-remove.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Remove color-scheme meta tag</title>
<meta id="dark" name="color-scheme" content="dark">
<meta id="light" name="color-scheme" content="light">
<link rel="help" href="https://html.spec.whatwg.org/multipage/semantics.html#meta-color-scheme">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="support/compute-root-color-scheme.js"></script>
<!--
  NOTE: This test assumes that the browser's default color-scheme is "light",
  see https://github.com/web-platform-tests/wpt/pull/31268 for reasoning
-->
<script>
  assert_root_color_scheme("dark", "First meta applies.");
  dark.remove();
  assert_root_color_scheme("light", "Second meta applies after first one is removed.");
  light.remove();
  assert_root_color_scheme("light", "Initial color-scheme with both meta elements removed.");
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/document-metadata/the-meta-element/color-scheme/meta-color-scheme-remove.html"
}
```
