# html/browsers/browsing-the-web/navigating-across-documents/javascript-url-security-check-failure.sub.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/javascript-url-security-check-failure.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>javascript: URL security check</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<body>
<script>
"use strict";

const cases = [
  ["cross-origin", "http://{{hosts[][www]}}:{{ports[http][0]}}/common/blank.html"],
  ["cross-origin-domain but same-origin", "/html/browsers/windows/resources/document-domain-setter.html"]
];

for (const [description, url] of cases) {
  promise_test(async t => {
    const iframe = await insertIframe(t, url);

    const unreached = t.unreached_func("message event fired");
    t.add_cleanup(() => window.removeEventListener("message", unreached));
    window.addEventListener("message", unreached);

    iframe.src = `javascript:parent.postMessage("boo", "*")`;

    // If no message was received after this time, the test passes.
    await new Promise(r => t.step_timeout(r, 50));
  }, `${description}, setting src`);

  promise_test(async t => {
    const iframe = await insertIframe(t, url);

    const unreached = t.unreached_func("message event fired");
    t.add_cleanup(() => window.removeEventListener("message", unreached));
    window.addEventListener("message", unreached);

    iframe.contentWindow.location.href = `javascript:parent.postMessage("boo", "*")`;

    // If no message was received after this time, the test passes.
    await new Promise(r => t.step_timeout(r, 50));
  }, `${description}, setting location.href`);
}

function insertIframe(t, url) {
  return new Promise((resolve, reject) => {
    const iframe = document.createElement("iframe");
    iframe.src = url;
    iframe.onload = () => resolve(iframe);
    iframe.onerror = () => reject(new Error("Failed to load the outer iframe"));

    t.add_cleanup(() => iframe.remove());

    document.body.append(iframe);
  });
}
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/javascript-url-security-check-failure.sub.html"
}
```
