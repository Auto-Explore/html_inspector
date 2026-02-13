# html/semantics/text-level-semantics/the-a-element/a-download-click-404.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/text-level-semantics/the-a-element/a-download-click-404.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Clicking on an &lt;a> element with a download attribute and href that leads to 404 should not navigate</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-a-element:activation-behaviour">
<link rel="help" href="https://html.spec.whatwg.org/multipage/links.html#attr-hyperlink-download">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<script>
"use strict";
async_test(t => {
    const errorFrame = document.createElement("iframe");

    errorFrame.addEventListener("load", t.step_func(function () {
        errorFrame.contentWindow.addEventListener(
            "beforeunload", t.unreached_func("Navigated instead of downloading"));

        errorFrame.contentDocument.querySelector("#error-url").click();
        t.step_timeout(() => t.done(), 1000);
    }));
    errorFrame.src = "resources/a-download-404.html";
    document.body.appendChild(errorFrame);
}, "Do not navigate to 404 for anchor with download");
</script>
</body>
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
  "source_name": "html/semantics/text-level-semantics/the-a-element/a-download-click-404.html"
}
```
