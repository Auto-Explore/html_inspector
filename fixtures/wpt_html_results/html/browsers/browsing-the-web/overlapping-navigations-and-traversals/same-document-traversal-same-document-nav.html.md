# html/browsers/browsing-the-web/overlapping-navigations-and-traversals/same-document-traversal-same-document-nav.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/overlapping-navigations-and-traversals/same-document-traversal-same-document-nav.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Same-document navigations during same-document traversals</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<!--
  Per spec, same-document navigations ignore the "ongoing navigation" flag and
  just happen synchronously, then queue onto the session history traversal queue
  to update the source of truth. However, the traversal was queued first, so it
  will ignore that update when calculating its endpoint.
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

  iframe.contentWindow.history.back();

  assert_equals(iframe.contentWindow.location.hash, "#2", "must not go back synchronously");

  iframe.contentWindow.location.hash = "#3";
  assert_equals(iframe.contentWindow.location.hash, "#3");

  // Eventually ends up on #1
  await t.step_wait(() => iframe.contentWindow.location.hash === "#1");
}, "same-document traversals are not canceled by fragment navigations and calculate their endpoint based on the original placement");

promise_test(async t => {
  const iframe = await createIframe(t);

  // Setup
  iframe.contentWindow.history.pushState(null, "", "/1");
  await delay(t, 0);
  iframe.contentWindow.history.pushState(null, "", "/2");
  await delay(t, 0);

  iframe.contentWindow.history.back();

  assert_equals(iframe.contentWindow.location.pathname, "/2", "must not go back synchronously");

  iframe.contentWindow.history.pushState(null, "", "/3");
  assert_equals(iframe.contentWindow.location.pathname, "/3");

  // Eventually ends up on /1
  await t.step_wait(() => iframe.contentWindow.location.pathname === "/1");
}, "same-document traversals are not canceled by pushState() and calculate their endpoint based on the original placement");
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
  "source_name": "html/browsers/browsing-the-web/overlapping-navigations-and-traversals/same-document-traversal-same-document-nav.html"
}
```
