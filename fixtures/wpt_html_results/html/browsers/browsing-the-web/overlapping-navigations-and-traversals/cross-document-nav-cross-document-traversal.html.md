# html/browsers/browsing-the-web/overlapping-navigations-and-traversals/cross-document-nav-cross-document-traversal.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/overlapping-navigations-and-traversals/cross-document-nav-cross-document-traversal.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Cross-document traversal during cross-document navigation</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<!--
  According to the spec, "apply the history step" will set the ongoing
  navigation to "traversal", canceling any non-mature navigations.
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

  iframe.contentWindow.location.search = "?3";
  iframe.contentWindow.history.back();

  assert_equals(iframe.contentWindow.location.search, "?2", "must not go back synchronously");

  await waitForLoad(iframe);
  assert_equals(iframe.contentWindow.location.search, "?1", "must go back one step eventually");
}, "cross-document navigations are stopped by cross-document back()");
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
  "source_name": "html/browsers/browsing-the-web/overlapping-navigations-and-traversals/cross-document-nav-cross-document-traversal.html"
}
```
