# html/browsers/the-window-object/named-access-on-the-window-object/removing.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-window-object/named-access-on-the-window-object/removing.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Named access on the window object - Removing elements</title>
<link rel="author" title="Matthew Phillips" href="mailto:matthew@matthewphillips.info">
<link rel="help" href="https://html.spec.whatwg.org/#named-access-on-the-window-object">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<body>
<script>
"use strict";

test(() => {
  const img = document.createElement("img");
  img.setAttribute("id", "foo");
  document.body.appendChild(img);
  assert_equals(window.foo, img);

  document.body.removeChild(img);
  assert_false("foo" in window);
}, "Removing an element must update the named properties");

test(() => {
  const text = document.createTextNode("foo");
  document.body.appendChild(text);

  const img = document.createElement("img");
  img.setAttribute("id", "removed");
  document.body.appendChild(img);
  assert_equals(window.removed, img);

  document.body.removeChild(text);
  assert_equals(window.removed, img);
}, "Removing a non-element node must not cause errors");
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
  "source_name": "html/browsers/the-window-object/named-access-on-the-window-object/removing.html"
}
```
