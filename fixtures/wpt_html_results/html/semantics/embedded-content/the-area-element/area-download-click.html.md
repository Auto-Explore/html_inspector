# html/semantics/embedded-content/the-area-element/area-download-click.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-area-element/area-download-click.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Clicking on an &lt;area> element with a download attribute must not throw an exception</title>
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-area-element:activation-behaviour">
<link rel="help" href="https://github.com/whatwg/html/issues/2116">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<script>
"use strict";
async_test(t => {
    const frame = document.createElement("iframe");

    frame.addEventListener("load", t.step_func(function () {
        frame.contentWindow.addEventListener(
            "beforeunload", t.unreached_func("Navigated instead of downloading"));
        const string = "test";
        const blob = new Blob([string], { type: "text/html" });

        const link = frame.contentDocument.querySelector("#blob-url");
        link.href = URL.createObjectURL(blob);

        link.click();

        t.step_timeout(() => t.done(), 1000);
    }));
    frame.src = "resources/area-download-click.html";
    document.body.appendChild(frame);
}, "Clicking on an <area> element with a download attribute must not throw an exception");
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
  "source_name": "html/semantics/embedded-content/the-area-element/area-download-click.html"
}
```
