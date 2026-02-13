# html/semantics/document-metadata/the-meta-element/color-scheme/meta-color-scheme-remove-head.html

Counts:
- errors: 3
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-meta-element/color-scheme/meta-color-scheme-remove-head.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Remove head with meta color-scheme</title>
<meta name="color-scheme" content="dark">
<link rel="help" href="https://html.spec.whatwg.org/multipage/semantics.html#meta-color-scheme">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="support/compute-root-color-scheme.js"></script>
<!--
  NOTE: This test assumes that the browser's default color-scheme is "light",
  see https://github.com/web-platform-tests/wpt/pull/31268 for reasoning
-->
<body></body>
<script>
  assert_root_color_scheme("dark", "Meta color-scheme applies.");
  document.head.remove();
  assert_root_color_scheme("light", "Initial value after removing head including meta color-scheme.");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_start_tag",
      "message": "Stray start tag “script”.",
      "severity": "Error",
      "span": {
        "byte_end": 554,
        "byte_start": 546,
        "col": 1,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 750,
        "byte_start": 554,
        "col": 9,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 759,
        "byte_start": 750,
        "col": 1,
        "line": 17
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
  "source_name": "html/semantics/document-metadata/the-meta-element/color-scheme/meta-color-scheme-remove-head.html"
}
```
