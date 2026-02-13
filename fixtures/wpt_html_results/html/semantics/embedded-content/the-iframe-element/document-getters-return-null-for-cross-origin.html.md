# html/semantics/embedded-content/the-iframe-element/document-getters-return-null-for-cross-origin.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/document-getters-return-null-for-cross-origin.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Test that contentDocument/getSVGDocument() return null for a cross-origin document.</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<iframe src='data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><rect height="100" width="100"/></svg>'></iframe>
<script>
const iframe = document.querySelector('iframe');
var t1 = async_test('HTMLIFrameElement.contentDocument for cross-origin document');
window.addEventListener(
    'load', t1.step_func_done(() => { assert_equals(iframe.contentDocument, null); }));
var t2 = async_test('HTMLIFrameElement.getSVGDocument() for cross-origin document');
window.addEventListener(
    'load', t2.step_func_done(() => { assert_equals(iframe.getSVGDocument(), null); }));
</script>
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.iframe.src.invalid",
      "message": "Bad value “data:image/svg+xml,<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\"><rect height=\"100\" width=\"100\"/></svg>” for attribute “src” on element “iframe”.",
      "severity": "Warning",
      "span": {
        "byte_end": 404,
        "byte_start": 249,
        "col": 1,
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/document-getters-return-null-for-cross-origin.html"
}
```
