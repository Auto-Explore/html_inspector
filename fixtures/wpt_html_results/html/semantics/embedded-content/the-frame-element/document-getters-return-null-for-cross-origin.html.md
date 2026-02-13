# html/semantics/embedded-content/the-frame-element/document-getters-return-null-for-cross-origin.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-frame-element/document-getters-return-null-for-cross-origin.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Test that contentDocument returns null for a cross-origin document.</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
var t = async_test('HTMLFrameElement.contentDocument for cross-origin document');
window.addEventListener(
    'load', t.step_func_done(() => { assert_equals(document.querySelector('frame').contentDocument, null); }));
</script>
<frameset>
<frame src='data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink"><rect height="100" width="100"/></svg>'></frame>
</frameset>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.element.frameset.obsolete",
      "message": "The “frameset” element is obsolete. Use the “iframe” element and CSS instead, or use server-side includes.",
      "severity": "Warning",
      "span": {
        "byte_end": 474,
        "byte_start": 464,
        "col": 1,
        "line": 11
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
  "source_name": "html/semantics/embedded-content/the-frame-element/document-getters-return-null-for-cross-origin.html"
}
```
