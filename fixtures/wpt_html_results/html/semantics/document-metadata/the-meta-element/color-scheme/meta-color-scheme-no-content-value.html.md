# html/semantics/document-metadata/the-meta-element/color-scheme/meta-color-scheme-no-content-value.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-meta-element/color-scheme/meta-color-scheme-no-content-value.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Meta color-scheme - no content value</title>
<meta name="color-scheme">
<link rel="help" href="https://html.spec.whatwg.org/multipage/semantics.html#meta-color-scheme">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="support/compute-root-color-scheme.js"></script>
<!--
  NOTE: This test assumes that the browser's default color-scheme is "light",
  see https://github.com/web-platform-tests/wpt/pull/31268 for reasoning
-->
<script>
  assert_root_color_scheme("light", "Meta color-scheme without content attribute has no effect.");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.meta.missing_content",
      "message": "Element “meta” is missing one or more of the following attributes: “content”, “property”.",
      "severity": "Warning",
      "span": {
        "byte_end": 94,
        "byte_start": 68,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/document-metadata/the-meta-element/color-scheme/meta-color-scheme-no-content-value.html"
}
```
