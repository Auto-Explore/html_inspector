# html/webappapis/dynamic-markup-insertion/html-unsafe-methods/Document-parseHTMLUnsafe-url-base.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/dynamic-markup-insertion/html-unsafe-methods/Document-parseHTMLUnsafe-url-base.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<link rel=author href="mailto:jarhar@chromium.org">
<!-- This was adapted from DOMParser-parseFromString-url-base.html -->
<title>parseHTMLUnsafe test of how the document's URL is set (base, no pushstate)</title>
<base href="/fake/base-from-outer-frame">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<iframe src="/html/webappapis/dynamic-markup-insertion/html-unsafe-methods/resources/parseHTMLUnsafe-iframe-base.html" onload="window.resolveLoadPromise();"></iframe>
<script src="/html/webappapis/dynamic-markup-insertion/html-unsafe-methods/resources/parseHTMLUnsafe-url-tests.js"></script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.base.must_come_before_link_or_script",
      "message": "The “base” element must come before any “link” or “script” elements in the document.",
      "severity": "Warning",
      "span": {
        "byte_end": 270,
        "byte_start": 229,
        "col": 1,
        "line": 5
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
  "source_name": "html/webappapis/dynamic-markup-insertion/html-unsafe-methods/Document-parseHTMLUnsafe-url-base.html"
}
```
