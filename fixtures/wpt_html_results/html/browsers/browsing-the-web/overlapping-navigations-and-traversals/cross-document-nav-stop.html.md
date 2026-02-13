# html/browsers/browsing-the-web/overlapping-navigations-and-traversals/cross-document-nav-stop.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/overlapping-navigations-and-traversals/cross-document-nav-stop.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Stop during cross-document navigations</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<body>
<script type="module">
import { createIframe, waitForPotentialNetworkLoads } from "./resources/helpers.mjs";

promise_test(async t => {
  const iframe = await createIframe(t);

  iframe.contentWindow.location.search = "?1";
  iframe.contentWindow.onload = t.unreached_func("load event fired");
  iframe.contentWindow.stop();

  await waitForPotentialNetworkLoads(t);
  assert_equals(iframe.contentWindow.location.search, "");
}, "cross-document navigations are stopped by stop()");
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
  "source_name": "html/browsers/browsing-the-web/overlapping-navigations-and-traversals/cross-document-nav-stop.html"
}
```
