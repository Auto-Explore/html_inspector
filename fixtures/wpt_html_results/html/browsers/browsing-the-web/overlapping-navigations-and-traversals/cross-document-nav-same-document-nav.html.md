# html/browsers/browsing-the-web/overlapping-navigations-and-traversals/cross-document-nav-same-document-nav.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/overlapping-navigations-and-traversals/cross-document-nav-same-document-nav.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Cross-document navigation after a same-document navigation</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<!--
  According to the spec, the "URL and history update steps" (used by
  pushState()) and the fragment navigation steps, do *not* modify the ongoing
  navigation, i.e. do not cancel any navigations.
-->

<body>
<script type="module">
import { createIframe, waitForLoad } from "./resources/helpers.mjs";

promise_test(async t => {
  const iframe = await createIframe(t);

  iframe.contentWindow.location.search = "?1";
  iframe.contentWindow.location.hash = "#2";

  assert_equals(iframe.contentWindow.location.search, "");
  assert_equals(iframe.contentWindow.location.hash, "#2");

  await waitForLoad(iframe);
  assert_equals(iframe.contentWindow.location.search, "?1");
  assert_equals(iframe.contentWindow.location.hash, "");
}, "cross-document navigation then fragment navigation");

promise_test(async t => {
  const iframe = await createIframe(t);

  iframe.contentWindow.location.search = "?1";
  iframe.contentWindow.history.pushState(null, "", "/2");

  assert_equals(iframe.contentWindow.location.search, "");
  assert_equals(iframe.contentWindow.location.pathname, "/2");

  await waitForLoad(iframe);
  assert_equals(iframe.contentWindow.location.search, "?1");
  assert_equals(iframe.contentWindow.location.pathname, "/common/blank.html");
}, "cross-document navigation then pushState()");
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
  "source_name": "html/browsers/browsing-the-web/overlapping-navigations-and-traversals/cross-document-nav-same-document-nav.html"
}
```
