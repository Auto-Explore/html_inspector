# html/dom/partial-updates/tentative/template-contentmethod-append-text-and-element.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/partial-updates/tentative/template-contentmethod-append-text-and-element.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<meta charset="utf-8" />
<title>HTML partial updates - declarative append</title>
<link rel=help href="https://github.com/WICG/declarative-partial-updates">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id="placeholder" contentname="placeholder">Some</div>
<template contentmethod="append"><div contentname="placeholder"> <span>content</span></div></template>
<script>
test(() => {
    const placeholder = document.getElementById("placeholder");
    assert_equals(placeholder.textContent, "Some content");
    assert_equals(placeholder.firstChild.textContent, "Some ");
    assert_equals(placeholder.firstChild.nextSibling.tagName, "SPAN");
}, "<template contentmethod=append> with text");
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
  "source_name": "html/dom/partial-updates/tentative/template-contentmethod-append-text-and-element.html"
}
```
