# html/browsers/browsing-the-web/scroll-to-fragid/replacement-enabled.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/scroll-to-fragid/replacement-enabled.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Navigating to a fragment should not clear forward history</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#scroll-to-fragid">
<link rel="help" href="https://github.com/whatwg/html/issues/2796">
<link rel="help" href="https://github.com/whatwg/html/pull/2869">
<link rel="author" href="mailto:d@domenic.me" title="Domenic Denicola">

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

    assert_equals(frameWindow.location.hash, "#cat");

    // Might not be 4 (= 3 for iframe + 1 initial) due to cross-browser differences or if people are
    // running this test in a window that has previously been places. The important thing for this
    // test is the delta from this value.
    const afterThreeNavigations = frameWindow.history.length;

    await navigateAndWaitForChange(frameWindow, w => w.history.back());

    assert_equals(frameWindow.location.hash, "#banana");
    assert_equals(frameWindow.history.length, afterThreeNavigations,
      "back() must not change the history length");

    await navigateAndWaitForChange(frameWindow,
      w => w.location.replace("/common/blank.html#zebra"));

    assert_equals(frameWindow.location.hash, "#zebra");
    assert_equals(frameWindow.history.length, afterThreeNavigations,
      "replace() must not change the history length");

    // As of the time of this writing (2017-08-14), Firefox is not firing hashchange on forward, so
    // we automatically assume navigation succeeded after 100 ms. A sibling test will test this
    // particular Firefox bug.
    await navigateAndWaitForChange(frameWindow, w => w.history.forward(),
      { assumeSuccessAfter: 500 });

    assert_equals(frameWindow.location.hash, "#cat");
    assert_equals(frameWindow.history.length, afterThreeNavigations,
      "forward() must not change the history length");

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
  "source_name": "html/browsers/browsing-the-web/scroll-to-fragid/replacement-enabled.html"
}
```
