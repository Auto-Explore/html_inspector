# html/semantics/text-level-semantics/the-a-element/a-download-click-redirect-to-javascript.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/text-level-semantics/the-a-element/a-download-click-redirect-to-javascript.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Clicking on an &lt;a> element with a download attribute and href that redirects to 'javascript:' should not navigate or execute</title>
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
        assert_false(errorFrame.contentWindow.executed);
        errorFrame.contentWindow.addEventListener(
            "beforeunload", t.unreached_func("Page should not navigate."));

        errorFrame.contentDocument.querySelector("#error-url").click();
        t.step_timeout(_ => {
          assert_false(errorFrame.contentWindow.executed, "Redirecting to javascript: was suppressed.");
          t.done();
        }, 1000);
    }));
    errorFrame.src = "resources/a-download-redirect-to-javascript.html";
    document.body.appendChild(errorFrame);
}, "Do not navigate or execute JS when redirecting a download to 'javascript:..'");
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
  "source_name": "html/semantics/text-level-semantics/the-a-element/a-download-click-redirect-to-javascript.html"
}
```
