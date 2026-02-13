# html/browsers/browsing-the-web/overlapping-navigations-and-traversals/same-document-nav-same-document-traversal.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/overlapping-navigations-and-traversals/same-document-nav-same-document-traversal.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Same-document traversal after a same-document navigations</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<!--
  These tests are kind of silly since it's hard to imagine any other result:
  same-document navigations are always synchronous so of course back() won't
  cancel them.

  Nevertheless they're nice as a basis from which to write corresponding app
  history tests, where the consequences aren't as obvious.
-->

<body>
<script type="module">
import { createIframe, delay } from "./resources/helpers.mjs";

promise_test(async t => {
  const iframe = await createIframe(t);

  // Setup
  iframe.contentWindow.location.hash = "#1";
  await delay(t, 0);
  iframe.contentWindow.location.hash = "#2";
  await delay(t, 0);

  iframe.contentWindow.location.hash = "#3";
  iframe.contentWindow.history.back();

  assert_equals(iframe.contentWindow.location.hash, "#3", "must not go back synchronously");

  // Does go back eventually, and only one step
  await t.step_wait(() => iframe.contentWindow.location.hash === "#2");
}, "fragment navigation then back()");

promise_test(async t => {
  const iframe = await createIframe(t);

  // Setup
  iframe.contentWindow.history.pushState(null, "", "?1");
  await delay(t, 0);
  iframe.contentWindow.history.pushState(null, "", "?2");
  await delay(t, 0);

  iframe.contentWindow.history.pushState(null, "", "?3");
  iframe.contentWindow.history.back();

  assert_equals(iframe.contentWindow.location.search, "?3", "must not go back synchronously");

  // Does go back eventually, and only one step
  await t.step_wait(() => iframe.contentWindow.location.search === "?2");
}, "pushState then back()");
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
  "source_name": "html/browsers/browsing-the-web/overlapping-navigations-and-traversals/same-document-nav-same-document-traversal.html"
}
```
