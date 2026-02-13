# html/webappapis/dynamic-markup-insertion/html-unsafe-methods/Document-parseHTMLUnsafe-url-pushstate.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/dynamic-markup-insertion/html-unsafe-methods/Document-parseHTMLUnsafe-url-pushstate.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<link rel=author href="mailto:jarhar@chromium.org">
<!-- This test was adapted from DOMParser-parseFromString-url-pushstate.html -->
<title>parseHTMLUnsafe test of how the document's URL is set (no base, pushstate)</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<iframe src="/html/webappapis/dynamic-markup-insertion/html-unsafe-methods/resources/parseHTMLUnsafe-iframe-pushstate.html" onload="window.resolveLoadPromise();"></iframe>

<script>
"use strict";
history.pushState(null, "", "/fake/push-state-from-outer-frame");
</script>
<script src="/html/webappapis/dynamic-markup-insertion/html-unsafe-methods/resources/parseHTMLUnsafe-url-tests.js"></script>
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
  "source_name": "html/webappapis/dynamic-markup-insertion/html-unsafe-methods/Document-parseHTMLUnsafe-url-pushstate.html"
}
```
