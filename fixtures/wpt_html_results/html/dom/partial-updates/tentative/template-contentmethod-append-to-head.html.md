# html/dom/partial-updates/tentative/template-contentmethod-append-to-head.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/partial-updates/tentative/template-contentmethod-append-to-head.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<head contentname="head">
<meta charset="utf-8" />
<title >HTML partial updates - cannot patch head directly</title>
<link rel=help href="https://github.com/WICG/declarative-partial-updates">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<template contentmethod="append">
    <head contentname="head"><meta id="added" content="something"></head>
</template>
<script>
test(() => {
    const meta = document.getElementById("added");
    assert_equals(meta, null);
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
  "source_name": "html/dom/partial-updates/tentative/template-contentmethod-append-to-head.html"
}
```
