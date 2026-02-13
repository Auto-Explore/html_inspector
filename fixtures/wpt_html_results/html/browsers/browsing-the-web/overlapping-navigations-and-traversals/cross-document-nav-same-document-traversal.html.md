# html/browsers/browsing-the-web/overlapping-navigations-and-traversals/cross-document-nav-same-document-traversal.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/overlapping-navigations-and-traversals/cross-document-nav-same-document-traversal.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Same-document traversal during cross-document navigation</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<!--
  According to the spec, "apply the history step" will set the ongoing
  navigation to "traversal", canceling any navigation that is still processing
  in parallel and hasn't yet reached "apply the history step".
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

  iframe.contentWindow.location.search = "?1";
  iframe.contentWindow.onload = t.unreached_func("load event fired");

  iframe.contentWindow.history.back();

  assert_equals(iframe.contentWindow.location.search, "", "must not go back synchronously (search)");
  assert_equals(iframe.contentWindow.location.hash, "#2", "must not go back synchronously (hash)");

  // Does go back eventually, and only one step
  await t.step_wait(() => iframe.contentWindow.location.hash === "#1" && iframe.contentWindow.location.search === "");
}, "cross-document navigations are stopped by same-document back()");
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
  "source_name": "html/browsers/browsing-the-web/overlapping-navigations-and-traversals/cross-document-nav-same-document-traversal.html"
}
```
