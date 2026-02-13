# html/browsers/the-window-object/named-access-on-the-window-object/doc-no-window.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-window-object/named-access-on-the-window-object/doc-no-window.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Named access on the window object - Document without window</title>
<link rel="author" title="Matthew Phillips" href="mailto:matthew@matthewphillips.info">
<link rel="help" href="https://html.spec.whatwg.org/#named-access-on-the-window-object">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<body>
<script>
"use strict";

test(() => {
  const doc = document.implementation.createHTMLDocument();
  // Sanity check
  assert_equals(doc.defaultView, null);

  const img = doc.createElement("img");
  img.setAttribute("id", "foo");
  doc.body.appendChild(img);
  assert_true(Boolean(img.parentNode));

  img.setAttribute("id", "bar");
  assert_equals(img.getAttribute("id"), "bar");

  doc.body.removeChild(img);
  assert_false(Boolean(img.parentNode));
}, "A document without a Window must not cause errors");
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
  "source_name": "html/browsers/the-window-object/named-access-on-the-window-object/doc-no-window.html"
}
```
