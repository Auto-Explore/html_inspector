# html/dom/partial-updates/tentative/template-contentmethod-prepend-text.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/partial-updates/tentative/template-contentmethod-prepend-text.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<meta charset="utf-8" />
<title>HTML partial updates - declarative prepend text</title>
<link rel=help href="https://github.com/WICG/declarative-partial-updates">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id="placeholder" contentname="placeholder"> content</div>
<template contentmethod="prepend"><div contentname="placeholder">Some</div></template>
<script>
test(() => {
    assert_equals(document.getElementById("placeholder").textContent, "Some content");
}, "<template contentmethod=prepend> with text");
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
  "source_name": "html/dom/partial-updates/tentative/template-contentmethod-prepend-text.html"
}
```
