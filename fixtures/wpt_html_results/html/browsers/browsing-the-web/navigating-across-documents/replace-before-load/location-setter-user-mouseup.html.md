# html/browsers/browsing-the-web/navigating-across-documents/replace-before-load/location-setter-user-mouseup.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/replace-before-load/location-setter-user-mouseup.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>No replace before load, triggered by location setters called as part of user-initiated mouseups</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="resources/helpers.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>

<!--
  We test this separate from click because the spec as of
  https://html.spec.whatwg.org/commit-snapshots/4ba46b025ec806ded7b4911bf8f9dd7bf9ff365e/#location-object-setter-navigate
  referenced click handlers specifically, instead of using the general user activation concept which
  includes other events like mouseup.
-->

<body>
<script>
"use strict";

promise_test(async t => {
  const sentinelIframe = await setupSentinelIframe(t);
  const startingHistoryLength = history.length;

  const code = `
    const button = document.createElement("button");
    button.id = "the-button";
    button.textContent = "needs to have content to be clickable";
    button.onmouseup = () => { location.href = "/common/blank.html?thereplacement"; };
    document.currentScript.before(button);
    parent.test_driver.click(button);
  `;

  const startURL = "resources/slow-code-injector.html?pipe=sub(none)&code=" + encodeURIComponent(code);
  const afterReplacementURL = "/common/blank.html?thereplacement";
  const iframe = insertIframe(t, startURL);

  assert_equals(history.length, startingHistoryLength, "Inserting the under-test iframe must not change history.length");

  await waitForLoad(t, iframe, afterReplacementURL);
  assert_equals(history.length, startingHistoryLength + 1, "history.length must change after waiting for the load");
}, "href");

promise_test(async t => {
  const sentinelIframe = await setupSentinelIframe(t);
  const startingHistoryLength = history.length;

  const code = `
    const button = document.createElement("button");
    button.id = "the-button";
    button.textContent = "needs to have content to be clickable";
    button.onmouseup = () => { location.search = "thereplacement"; };
    document.currentScript.before(button);
    parent.test_driver.click(button);
  `;

  const startURL = "resources/slow-code-injector.html?pipe=sub(none)&code=" + encodeURIComponent(code);
  const afterReplacementURL = "resources/slow-code-injector.html?thereplacement";
  const iframe = insertIframe(t, startURL);

  assert_equals(history.length, startingHistoryLength, "Inserting the under-test iframe must not change history.length");

  await waitForLoad(t, iframe, afterReplacementURL);
  assert_equals(history.length, startingHistoryLength + 1, "history.length must change after waiting for the load");
}, "search");

promise_test(async t => {
  const sentinelIframe = await setupSentinelIframe(t);
  const startingHistoryLength = history.length;

  const code = `
    const button = document.createElement("button");
    button.id = "the-button";
    button.textContent = "needs to have content to be clickable";
    button.onmouseup = () => { location.hash = "thereplacement"; };
    document.currentScript.before(button);
    parent.test_driver.click(button);
  `;

  const startURL = "resources/slow-code-injector.html?pipe=sub(none)&code=" + encodeURIComponent(code);
  const afterReplacementURL = startURL + "#thereplacement";
  const iframe = insertIframe(t, startURL);

  assert_equals(history.length, startingHistoryLength, "Inserting the under-test iframe must not change history.length");

  await waitForLoad(t, iframe, afterReplacementURL);
  assert_equals(history.length, startingHistoryLength + 1, "history.length must change after waiting for the load");
}, "hash");
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/replace-before-load/location-setter-user-mouseup.html"
}
```
