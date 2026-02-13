# html/dom/partial-updates/tentative/template-contentmethod-multiple.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/partial-updates/tentative/template-contentmethod-multiple.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<meta charset="utf-8" />
<title>HTML partial updates - multiple patches</title>
<link rel=help href="https://github.com/WICG/declarative-partial-updates">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div contentname="p1">Old P1</div>
<div contentname="p2">Old P2</div>
<div contentname="p3">Old P3</div>
<template contentmethod="replace-children">
    <div contentname="p1"><span>New P1</span></div>
    <div contentname="p2"><span>New P2</span></div>
</template>
<template contentmethod="replace">
    <div contentname="p3"><span>New P3</span></div>
</template>
<template contentmethod="append">
    <div contentname="p2"><span>...</span><span></span>more P2</span></div>
</template>
<template contentmethod="prepend">
    <div contentname="p2"><span>And</span><span>...</span></div>
    <div contentname="p3"><span>Pre P3 </span></div>
</template>
<script>
test(() => {
    assert_equals(document.querySelector("div[contentname=p1]").textContent, 'New P1');
    assert_equals(document.querySelector("div[contentname=p2]").textContent, 'And...New P2...more P2');
    assert_equals(document.querySelector("div[contentname=p3]").textContent, 'Pre P3 New P3');
}, "Multiple <template contentmethod>");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “span”.",
      "severity": "Error",
      "span": {
        "byte_end": 745,
        "byte_start": 738,
        "col": 63,
        "line": 19
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
  "source_name": "html/dom/partial-updates/tentative/template-contentmethod-multiple.html"
}
```
