# html/infrastructure/safe-passing-of-structured-data/shared-array-buffers/no-transferring.https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/infrastructure/safe-passing-of-structured-data/shared-array-buffers/no-transferring.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>SharedArrayBuffers cannot be transferred</title>
<link rel="help" href="https://html.spec.whatwg.org/#structuredclone">
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<script>
"use strict";

test(() => {
  const sab = new SharedArrayBuffer();
  assert_throws_dom("DataCloneError", () => window.postMessage(sab, "*", [sab]));
  assert_throws_dom("DataCloneError", () => window.postMessage("test", "*", [sab]));
}, "Trying to transfer a SharedArrayBuffer to this window throws");

test(() => {
  const sab = new SharedArrayBuffer();
  const worker = new Worker("../resources/echo-worker.js");
  assert_throws_dom("DataCloneError", () => worker.postMessage(sab, [sab]));
  assert_throws_dom("DataCloneError", () => worker.postMessage("test", [sab]));
}, "Trying to transfer a SharedArrayBuffer to a worker throws");

test(() => {
  const sab = new SharedArrayBuffer();
  const channel = new MessageChannel();
  assert_throws_dom("DataCloneError", () => channel.port1.postMessage(sab, [sab]));
  assert_throws_dom("DataCloneError", () => channel.port1.postMessage("test", [sab]));
}, "Trying to transfer a SharedArrayBuffer through a MessagePort throws");
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
  "source_name": "html/infrastructure/safe-passing-of-structured-data/shared-array-buffers/no-transferring.https.html"
}
```
