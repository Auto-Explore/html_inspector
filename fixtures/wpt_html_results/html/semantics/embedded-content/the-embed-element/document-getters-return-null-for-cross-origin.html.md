# html/semantics/embedded-content/the-embed-element/document-getters-return-null-for-cross-origin.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-embed-element/document-getters-return-null-for-cross-origin.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Test that getSVGDocument() returns null for a cross-origin document.</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<embed src='data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><rect height="100" width="100"/></svg>'></embed>
<script>
const embed = document.querySelector('embed');
var t = async_test('HTMLEmbedElement.getSVGDocument() for cross-origin document');
window.addEventListener(
    'load', t.step_func_done(() => { assert_equals(embed.getSVGDocument(), null); }));
</script>
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.embed.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\"><rect height=\"100\" width=\"100\"/></svg>” for attribute “src” on element “embed”.",
      "severity": "Warning",
      "span": {
        "byte_end": 388,
        "byte_start": 234,
        "col": 1,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “embed”.",
      "severity": "Error",
      "span": {
        "byte_end": 396,
        "byte_start": 388,
        "col": 155,
        "line": 7
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
  "source_name": "html/semantics/embedded-content/the-embed-element/document-getters-return-null-for-cross-origin.html"
}
```
