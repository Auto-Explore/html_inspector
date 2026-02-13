# html/browsers/browsing-the-web/scroll-to-fragid/forward-triggers-hashchange.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/scroll-to-fragid/forward-triggers-hashchange.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Navigating forward after replace() should still trigger hashchange</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#history-traversal">
<link rel="author" href="mailto:d@domenic.me" title="Domenic Denicola">

<!-- While writing ./replacement-enabled.html, a bug was discovered in Firefox where it does not
fire hashchange when using history.forward(), at least under certain conditions. So, this test
exercises that specifically. -->

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="navigate-helpers.js"></script>

<body>

<script>
"use strict";
let resolve, iframe;
promise_test(() => {
  iframe = document.createElement("iframe");
  iframe.src = "/common/blank.html";
  iframe.addEventListener("load", runTest);
  document.body.appendChild(iframe);

  return new Promise(r => resolve = r);
});

function runTest() {
  iframe.removeEventListener("load", runTest);
  const frameWindow = iframe.contentWindow;

  resolve((async () => {
    await navigateAndWaitForChange(frameWindow, w => w.location.href = "/common/blank.html#apple");
    await navigateAndWaitForChange(frameWindow, w => w.location.href = "/common/blank.html#banana");
    await navigateAndWaitForChange(frameWindow, w => w.location.href = "/common/blank.html#cat");

    await navigateAndWaitForChange(frameWindow, w => w.history.back());
    await navigateAndWaitForChange(frameWindow,
      w => w.location.replace("/common/blank.html#zebra"));
    await navigateAndWaitForChange(frameWindow, w => w.history.forward());
  })());
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
  "source_name": "html/browsers/browsing-the-web/scroll-to-fragid/forward-triggers-hashchange.html"
}
```
