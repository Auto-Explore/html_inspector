# html/browsers/the-window-object/named-access-on-the-window-object/multi-match.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-window-object/named-access-on-the-window-object/multi-match.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Named access on the window object - Multiple matches</title>
<link rel="author" title="Matthew Phillips" href="mailto:matthew@matthewphillips.info">
<link rel="help" href="https://html.spec.whatwg.org/#named-access-on-the-window-object">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<body>
<script>
"use strict";

test(() => {
  const img1 = document.createElement("img");
  img1.setAttribute("name", "foo");
  document.body.appendChild(img1);

  const img0 = document.createElement("img");
  img0.setAttribute("name", "foo");
  document.body.insertBefore(img0, img1);

  const div = document.createElement("div");
  div.setAttribute("id", "foo");
  document.body.appendChild(div);

  assert_true(window.foo instanceof window.HTMLCollection);
  assert_equals(window.foo.length, 3);
  assert_true(window.foo[0] === img0);
  assert_true(window.foo[1] === img1);
  assert_true(window.foo[2] === div);
}, "A named property must return a HTMLCollection if there are multiple matches");
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
  "source_name": "html/browsers/the-window-object/named-access-on-the-window-object/multi-match.html"
}
```
