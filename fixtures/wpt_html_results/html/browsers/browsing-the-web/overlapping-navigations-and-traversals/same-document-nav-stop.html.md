# html/browsers/browsing-the-web/overlapping-navigations-and-traversals/same-document-nav-stop.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/overlapping-navigations-and-traversals/same-document-nav-stop.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Stop after a same-document navigations</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<body>
<script type="module">
import { createIframe } from "./resources/helpers.mjs";

promise_test(async t => {
  const iframe = await createIframe(t);

  iframe.contentWindow.location.hash = "#1";
  iframe.contentWindow.stop();

  assert_equals(iframe.contentWindow.location.hash, "#1");
}, "fragment navigations are not stopped by stop()");

promise_test(async t => {
  const iframe = await createIframe(t);

  iframe.contentWindow.history.pushState(null, "", "?1");
  iframe.contentWindow.stop();

  assert_equals(iframe.contentWindow.location.search, "?1");
}, "pushState() navigations are not stopped by stop()");
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
  "source_name": "html/browsers/browsing-the-web/overlapping-navigations-and-traversals/same-document-nav-stop.html"
}
```
