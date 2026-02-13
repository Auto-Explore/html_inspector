# html/dom/partial-updates/tentative/template-contentmethod-prepend-invalid-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/partial-updates/tentative/template-contentmethod-prepend-invalid-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<meta charset="utf-8" />
<title>HTML partial updates - declarative prepend element</title>
<link rel=help href="https://github.com/WICG/declarative-partial-updates">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id="placeholder" contentname="placeholder"><span id="ref">old</span>content</div>
<template contentmethod="prepend">
    <div contentname="placeholder"><span>New </span><script id="removal-script">
            document.querySelector("#ref").remove();
        </script><span>Junk </span></div>
</template>
<script>
test(() => {
    document.querySelector("#removal-script").remove();
    assert_equals(document.getElementById("placeholder").textContent, "New content");
}, "If the first node is no longer a child of the parent during prepend, next insertions abort");
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
  "source_name": "html/dom/partial-updates/tentative/template-contentmethod-prepend-invalid-ref.html"
}
```
