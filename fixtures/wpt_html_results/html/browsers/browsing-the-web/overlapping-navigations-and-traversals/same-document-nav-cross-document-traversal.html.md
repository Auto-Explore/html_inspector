# html/browsers/browsing-the-web/overlapping-navigations-and-traversals/same-document-nav-cross-document-traversal.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/overlapping-navigations-and-traversals/same-document-nav-cross-document-traversal.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Traversal after a same-document navigations</title>
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
import { createIframe, waitForLoad, delay } from "./resources/helpers.mjs";

promise_test(async t => {
  const iframe = await createIframe(t);

  // Setup
  // Extra delay()s are necessary because if we navigate "inside" the load
  // handler (i.e. in a promise reaction for the load handler) then it will
  // be a replace navigation.
  iframe.contentWindow.location.search = "?1";
  await waitForLoad(iframe);
  await delay(t, 0);
  iframe.contentWindow.location.search = "?2";
  await waitForLoad(iframe);
  await delay(t, 0);

  iframe.contentWindow.location.hash = "#3";
  iframe.contentWindow.history.go(-2);

  assert_equals(iframe.contentWindow.location.search, "?2", "must not go back synchronously (search)");
  assert_equals(iframe.contentWindow.location.hash, "#3", "must not go back synchronously (hash)");

  await waitForLoad(iframe);
  assert_equals(iframe.contentWindow.location.search, "?1", "must go back eventually (search)");
  assert_equals(iframe.contentWindow.location.hash, "", "must go back eventually (hash)");
}, "fragment navigation then go(-2)");

promise_test(async t => {
  const iframe = await createIframe(t);

  // Setup
  // Extra delay()s are necessary because if we navigate "inside" the load
  // handler (i.e. in a promise reaction for the load handler) then it will
  // be a replace navigation.
  iframe.contentWindow.location.search = "?1";
  await waitForLoad(iframe);
  await delay(t, 0);
  iframe.contentWindow.location.search = "?2";
  await waitForLoad(iframe);
  await delay(t, 0);

  iframe.contentWindow.history.pushState(null, "", "/3");
  iframe.contentWindow.history.go(-2);

  assert_equals(iframe.contentWindow.location.search, "", "must not go back synchronously (search)");
  assert_equals(iframe.contentWindow.location.pathname, "/3", "must not go back synchronously (pathname)");

  await waitForLoad(iframe);
  assert_equals(iframe.contentWindow.location.search, "?1", "must go back eventually (search)");
  assert_equals(iframe.contentWindow.location.pathname, "/common/blank.html", "must go back eventually (pathname)");

}, "pushState then go(-2)");
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
  "source_name": "html/browsers/browsing-the-web/overlapping-navigations-and-traversals/same-document-nav-cross-document-traversal.html"
}
```
