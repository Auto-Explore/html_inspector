# html/browsers/the-window-object/named-access-on-the-window-object/nested-context.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-window-object/named-access-on-the-window-object/nested-context.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Named access on the window object - Nested browsing context</title>
<link rel="author" title="Matthew Phillips" href="mailto:matthew@matthewphillips.info">
<link rel="help" href="https://html.spec.whatwg.org/#named-access-on-the-window-object">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<body>
<script>
"use strict";

test(() => {
  const iframe = document.createElement("iframe");
  iframe.setAttribute("name", "foo");
  document.body.appendChild(iframe);

  assert_equals(window.foo, iframe.contentWindow);
}, "A named property that matches any element that contains a nested " +
  "browsing context, must return the WindowProxy of that context");

test(() => {
  const iframe = document.createElement("iframe");
  iframe.setAttribute("id", "bar");
  document.body.appendChild(iframe);

  assert_equals(window.bar, iframe);
}, "A named property that matches an element that contains a nested " +
  "browsing context must return the element if using id");
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
  "source_name": "html/browsers/the-window-object/named-access-on-the-window-object/nested-context.html"
}
```
