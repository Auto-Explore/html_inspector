# html/semantics/embedded-content/the-iframe-element/iframe-network-error.sub.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/iframe-network-error.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Network errors with iframe elements</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<body>
<script>
"use strict";

async_test(t => {
  const iframe = document.createElement("iframe");
  iframe.src = "//{{hosts[][nonexistent]}}/";
  iframe.onload = () => t.done();
  iframe.onerror = t.unreached_func("error event must not fire");
  document.body.append(iframe);
}, "new iframe: nonexistent host");

async_test(t => {
  const iframe = document.createElement("iframe");
  iframe.src = "../resources/not-embeddable.html";
  iframe.onload = () => t.done();
  iframe.onerror = t.unreached_func("error event must not fire");
  document.body.append(iframe);
}, "new iframe: X-Frame-Options prevents embedding");

async_test(t => {
  const iframe = document.createElement("iframe");
  iframe.src = "/common/blank.html";
  iframe.name = "existingIframe1";
  iframe.onload = t.step_func(() => {
    iframe.onload = () => t.done();
    iframe.onerror = t.unreached_func("error event must not fire 2");

    frames.existingIframe1.location.href = "//{{hosts[][nonexistent]}}/";
  });
  iframe.onerror = t.unreached_func("error event must not fire 1");
  document.body.append(iframe);
}, "navigating an existing iframe: nonexistent host");

async_test(t => {
  const iframe = document.createElement("iframe");
  iframe.src = "/common/blank.html";
  iframe.name = "existingIframe2";
  iframe.onload = t.step_func(() => {
    iframe.onload = () => t.done();
    iframe.onerror = t.unreached_func("error event must not fire 2");

    frames.existingIframe2.location.href = "../resources/not-embeddable.html";
  });
  iframe.onerror = t.unreached_func("error event must not fire 1");
  document.body.append(iframe);
}, "navigating an existing iframe: X-Frame-Options prevents embedding");
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/iframe-network-error.sub.html"
}
```
