# html/browsers/the-window-object/named-access-on-the-window-object/existing-prop.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-window-object/named-access-on-the-window-object/existing-prop.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Named access on the window object - Existing property</title>
<link rel="author" title="Matthew Phillips" href="mailto:matthew@matthewphillips.info">
<link rel="help" href="https://html.spec.whatwg.org/#named-access-on-the-window-object">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<body>
<script>
"use strict";

test(() => {
  window["existing-one"] = 5;

  const img = document.createElement("img");
  img.setAttribute("id", "existing-one");
  document.body.appendChild(img);

  assert_equals(window["existing-one"], 5);
}, "A named property must not be set if a property already exists");

test(() => {
  window["existing-undefined"] = undefined;

  const img = document.createElement("img");
  img.setAttribute("id", "existing-undefined");
  document.body.appendChild(img);

  assert_equals(window["existing-undefined"], undefined);
}, "A named property must not be set if a property already exists, even if undefined");

test(() => {
  const img = document.createElement("img");
  img.setAttribute("id", "user-val");
  document.body.appendChild(img);

  window["user-val"] = 5;

  assert_equals(window["user-val"], 5);
}, "If a property is occupied by a named property but then the user sets it, the user's value shadows it");

test(() => {
  const img = document.createElement("img");
  img.setAttribute("id", "undefined");
  document.body.appendChild(img);

  /* eslint-disable no-void */
  assert_equals(undefined, void 0);
  assert_equals(window.undefined, void 0);
  /* eslint-enable no-void */
}, "A named property must not be set if the property already exists in the proxy");
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
  "source_name": "html/browsers/the-window-object/named-access-on-the-window-object/existing-prop.html"
}
```
