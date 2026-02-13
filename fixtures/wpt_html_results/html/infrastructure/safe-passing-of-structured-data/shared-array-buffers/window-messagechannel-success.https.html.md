# html/infrastructure/safe-passing-of-structured-data/shared-array-buffers/window-messagechannel-success.https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/infrastructure/safe-passing-of-structured-data/shared-array-buffers/window-messagechannel-success.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Structured cloning of SharedArrayBuffers using MessageChannel</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#structuredserialize">
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="resources/test-incrementer.js"></script>

<div id="log"></div>

<script>
"use strict";

promise_test(t => {
  const worker = new Worker("resources/incrementer-worker-with-channel.js");
  const channel = new MessageChannel();
  worker.postMessage(channel.port2, [channel.port2]);

  return testSharingViaIncrementerScript(t, channel.port1, "window", channel.port1, "worker");
}, "postMessaging to a dedicated worker via MessageChannel allows them to see each others' modifications");
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
  "source_name": "html/infrastructure/safe-passing-of-structured-data/shared-array-buffers/window-messagechannel-success.https.html"
}
```
