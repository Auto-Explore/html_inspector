# html/dom/partial-updates/tentative/template-contentmethod-style-in-head.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/partial-updates/tentative/template-contentmethod-style-in-head.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<meta charset="utf-8" />
<title>HTML partial updates - patch updates style with plain text</title>
<link rel=help href="https://github.com/WICG/declarative-partial-updates">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<style id="placeholder" contentname="cn"></style>
<body>
<div id="target"></div>
<template contentmethod="replace">
    <style contentname="cn" id="placeholder">
        #target { color: rgba(100, 0, 100); }
        #something:after { content: "<div id=dontparseme></div>" };
    </style>
</template>
<script>
test(() => {
    assert_equals(document.querySelector("#placeholder").firstElementChild, null);
    assert_equals(getComputedStyle(document.getElementById("target")).color, "rgb(100, 0, 100)");
});

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
  "source_name": "html/dom/partial-updates/tentative/template-contentmethod-style-in-head.html"
}
```
