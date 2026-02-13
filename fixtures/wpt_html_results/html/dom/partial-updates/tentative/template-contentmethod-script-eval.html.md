# html/dom/partial-updates/tentative/template-contentmethod-script-eval.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/partial-updates/tentative/template-contentmethod-script-eval.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<meta charset="utf-8" />
<title>HTML partial updates - content template updates script with plain text but doesn't execute</title>
<link rel=help href="https://github.com/WICG/declarative-partial-updates">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<script id="placeholder" contentname="s">
    window.state = "init";
</script>
<script>
    window.placeholder = document.querySelector("#placeholder");
</script>
<template contentmethod="replace-children"><script contentname="s">window.state = 'patched';</script></template>
<script>
test(() => {
    assert_equals(placeholder.firstElementChild, null);
    assert_equals(placeholder.textContent, "window.state = 'patched';");
    // The script has already started, so patching it would have no effect.
    assert_equals(window.state, "init");
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
  "source_name": "html/dom/partial-updates/tentative/template-contentmethod-script-eval.html"
}
```
