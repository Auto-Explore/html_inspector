# html/infrastructure/safe-passing-of-structured-data/structured-cloning-error-extra.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/infrastructure/safe-passing-of-structured-data/structured-cloning-error-extra.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Structured cloning of Error objects: extra tests</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<!-- Most tests are in the general framework in structuredclone_0.html.
     This contains specialty tests that don't fit into that framework. -->

<body>

<script>
"use strict";
test(t => {
  const exceptionToThrow = new Error("throw me!");

  const badError = new Error();
  Object.defineProperty(badError, "name", { get() { throw exceptionToThrow; } });

  const worker = new Worker("./resources/echo-worker.js");
  t.add_cleanup(() => worker.terminate());

  assert_throws_exactly(exceptionToThrow, () => {
    worker.postMessage(badError);
  });
}, "Throwing name getter fails serialization");

// https://bugs.chromium.org/p/chromium/issues/detail?id=1030086
// https://github.com/whatwg/html/pull/5150
async_test(t => {
  window.onmessage = t.step_func_done(e => {
    assert_equals(e.data.name, "TypeError");
  });

  const iframe = document.createElement("iframe");
  iframe.onload = () => {
    if (iframe.contentWindow.location === "about:blank") {
      return;
    }

    iframe.contentWindow.doIt();
  };
  iframe.src = "resources/post-parent-type-error.html";
  document.body.append(iframe);
}, "Errors sent across realms should preserve their type");
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
  "source_name": "html/infrastructure/safe-passing-of-structured-data/structured-cloning-error-extra.html"
}
```
