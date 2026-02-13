# html/browsers/browsing-the-web/overlapping-navigations-and-traversals/cross-document-traversal-stop.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/overlapping-navigations-and-traversals/cross-document-traversal-stop.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Stop during cross-document traversals</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<!--
  The spec says that stop() must not stop traverals.

  (Note: the spec also says the UI "stop" button must not stop traversals, but
  that does not match browsers. See https://github.com/whatwg/html/issues/6905.
  But that is not what's under test here.)
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

  window.stop();

  await waitForLoad(iframe);
  assert_equals(iframe.contentWindow.location.search, "?1", "must go back eventually");
}, "cross-document traversals are not stopped by stop()");
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
  "source_name": "html/browsers/browsing-the-web/overlapping-navigations-and-traversals/cross-document-traversal-stop.html"
}
```
