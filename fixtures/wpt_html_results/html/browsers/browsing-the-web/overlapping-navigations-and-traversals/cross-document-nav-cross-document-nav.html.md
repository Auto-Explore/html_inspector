# html/browsers/browsing-the-web/overlapping-navigations-and-traversals/cross-document-nav-cross-document-nav.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/overlapping-navigations-and-traversals/cross-document-nav-cross-document-nav.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Cross-document navigation after a cross-document navigation</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<!--
  According to the spec, the navigate algorithm synchronously cancels ongoing
  non-mature navigations.
-->

<body>
<script type="module">
import { createIframe, waitForLoad, waitForPotentialNetworkLoads } from "./resources/helpers.mjs";

promise_test(async t => {
  const iframe = await createIframe(t);

  iframe.contentWindow.location.search = "?1";
  iframe.contentWindow.location.search = "?2";
  assert_equals(iframe.contentWindow.location.search, "");

  await waitForLoad(iframe);
  assert_equals(iframe.contentWindow.location.search, "?2");

  iframe.onload = t.unreached_func("second load event");
  await waitForPotentialNetworkLoads(t);
  assert_equals(iframe.contentWindow.location.search, "?2");
}, "cross-document navigation then cross-document navigation");
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
  "source_name": "html/browsers/browsing-the-web/overlapping-navigations-and-traversals/cross-document-nav-cross-document-nav.html"
}
```
