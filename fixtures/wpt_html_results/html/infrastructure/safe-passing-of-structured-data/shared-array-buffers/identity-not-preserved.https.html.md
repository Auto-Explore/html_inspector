# html/infrastructure/safe-passing-of-structured-data/shared-array-buffers/identity-not-preserved.https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/infrastructure/safe-passing-of-structured-data/shared-array-buffers/identity-not-preserved.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>SharedArrayBuffers, when cloned, do not give back the same object</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#structuredserialize">
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/utils.js"></script>
<script src="resources/test-sab.js"></script>

<div id="log"></div>

<script>
"use strict";

async_test(t => {
  const testId = token();
  const sab = new SharedArrayBuffer(1);
  window.addEventListener("message", t.step_func(({ data }) => {
    if (data.testId !== testId) {
      return;
    }

    assertSABsHaveSameBackingBlock(sab, data.sab);

    t.done();
  }));

  window.postMessage({ testId, sab }, "*");
}, "postMessaging to this window does not give back the same SharedArrayBuffer (but does use the same backing block)");

async_test(t => {
  const testId = token();
  const sab = new SharedArrayBuffer();
  const worker = new Worker("../resources/echo-worker.js");

  worker.addEventListener("message", t.step_func(({ data }) => {
    if (data.testId !== testId) {
      return;
    }

    assert_not_equals(data.sab, sab);
    t.done();
  }));

  worker.postMessage({ testId, sab });
}, "postMessaging to a worker and back does not give back the same SharedArrayBuffer");

async_test(t => {
  const testId = token();
  const sab = new SharedArrayBuffer();
  window.addEventListener("message", t.step_func(({ data }) => {
    if (data.testId !== testId) {
      return;
    }

    assert_not_equals(data.sab, sab);
    t.done();
  }));

  const iframe = document.createElement("iframe");
  iframe.onload = t.step_func(() => {
    iframe.contentWindow.postMessage({ testId, sab }, "*");
  });
  iframe.src = "../resources/echo-iframe.html";
  document.body.appendChild(iframe);
}, "postMessaging to an iframe and back does not give back the same SharedArrayBuffer");
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
  "source_name": "html/infrastructure/safe-passing-of-structured-data/shared-array-buffers/identity-not-preserved.https.html"
}
```
