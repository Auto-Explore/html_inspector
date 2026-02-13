# html/browsers/browsing-the-web/overlapping-navigations-and-traversals/cross-document-traversal-same-document-nav.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/overlapping-navigations-and-traversals/cross-document-traversal-same-document-nav.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Same-document navigations during cross-document traversals</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<!--
  The spec explicitly covers this case, with a Jake diagram:
  https://whatpr.org/html/6315/browsing-the-web.html#example-sync-navigation-steps-queue-jumping-basic
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

  iframe.contentWindow.history.back();

  assert_equals(iframe.contentWindow.location.search, "?2", "must not go back synchronously");

  iframe.contentWindow.location.hash = "#3";
  assert_equals(iframe.contentWindow.location.search, "?2");
  assert_equals(iframe.contentWindow.location.hash, "#3");

  // Eventually ends up on ?1
  await t.step_wait(() => iframe.contentWindow.location.search === "?1" && iframe.contentWindow.location.hash === "");
}, "same-document traversals + fragment navigations");

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

  iframe.contentWindow.history.back();

  assert_equals(iframe.contentWindow.location.search, "?2", "must not go back synchronously");

  iframe.contentWindow.history.pushState(null, "", "?3");
  assert_equals(iframe.contentWindow.location.search, "?3");

  // Eventually ends up on ?1
  await t.step_wait(() => iframe.contentWindow.location.search === "?1");
}, "same-document traversals + pushState()");
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
  "source_name": "html/browsers/browsing-the-web/overlapping-navigations-and-traversals/cross-document-traversal-same-document-nav.html"
}
```
