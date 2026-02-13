# html/browsers/browsing-the-web/overlapping-navigations-and-traversals/same-document-traversal-cross-document-traversal.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/overlapping-navigations-and-traversals/same-document-traversal-cross-document-traversal.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Cross-document traversals during same-document traversals</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<!--
  Compare this to cross-document-traversal-cross-document-traversal.html. Since
  there are no network loads for the first traversal here, it does observably go
  through. So we end up with both traversals before observable in sequence.
-->

<body>
<script type="module">
import { createIframe, waitForLoad, waitForHashchange, delay } from "./resources/helpers.mjs";

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
  await waitForHashchange(iframe.contentWindow);

  iframe.contentWindow.history.back();
  assert_equals(iframe.contentWindow.location.search, "?2", "must not go back synchronously 1 (search)");
  assert_equals(iframe.contentWindow.location.hash, "#3", "must not go back synchronously 1 (hash)");

  iframe.contentWindow.history.back();
  assert_equals(iframe.contentWindow.location.search, "?2", "must not go back synchronously 1 (search)");
  assert_equals(iframe.contentWindow.location.hash, "#3", "must not go back synchronously 1 (hash)");

  await waitForHashchange(iframe.contentWindow);
  assert_equals(iframe.contentWindow.location.search, "?2", "first hashchange event must be going back (search)");
  assert_equals(iframe.contentWindow.location.hash, "", "first hashchange event must be going back (hash)");

  await waitForLoad(iframe);
  assert_equals(iframe.contentWindow.location.search, "?1", "first load event must be going back (search)");
  assert_equals(iframe.contentWindow.location.hash, "", "first load event must be going back (hash)");
}, "traversals in the same (back) direction: queued up");

promise_test(async t => {
  const iframe = await createIframe(t);

  // Setup
  // Extra delay()s are necessary because if we navigate "inside" the load
  // handler (i.e. in a promise reaction for the load handler) then it will
  // be a replace navigation.
  iframe.contentWindow.location.search = "?1";
  await waitForLoad(iframe);
  await delay(t, 0);
  iframe.contentWindow.location.hash = "#2";
  await waitForHashchange(iframe.contentWindow);
  iframe.contentWindow.location.search = "?3";
  await waitForLoad(iframe);
  await delay(t, 0);
  iframe.contentWindow.history.back();
  await waitForLoad(iframe);
  iframe.contentWindow.history.back();
  await waitForHashchange(iframe.contentWindow);
  assert_equals(iframe.contentWindow.location.search, "?1", "we made our way to ?1 for setup (search)");
  assert_equals(iframe.contentWindow.location.hash, "", "we made our way to ?1 for setup (search)");

  iframe.contentWindow.history.forward();
  assert_equals(iframe.contentWindow.location.search, "?1", "must not go forward synchronously 1 (search)");
  assert_equals(iframe.contentWindow.location.hash, "", "must not go forward synchronously 1 (hash)");

  iframe.contentWindow.history.forward();
  assert_equals(iframe.contentWindow.location.search, "?1", "must not go forward synchronously 2 (search)");
  assert_equals(iframe.contentWindow.location.hash, "", "must not go forward synchronously 2 (hash)");

  await waitForHashchange(iframe.contentWindow);
  assert_equals(iframe.contentWindow.location.search, "?1", "first hashchange event must be going forward (search)");
  assert_equals(iframe.contentWindow.location.hash, "#2", "first hashchange event must be going forward (hash)");

  await waitForLoad(iframe);
  assert_equals(iframe.contentWindow.location.search, "?3", "first load event must be going forward (search)");
  assert_equals(iframe.contentWindow.location.hash, "#2", "first load event must be going forward (hash)");
}, "traversals in the same (forward) direction: queued up");
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
  "source_name": "html/browsers/browsing-the-web/overlapping-navigations-and-traversals/same-document-traversal-cross-document-traversal.html"
}
```
