# html/browsers/browsing-the-web/navigating-across-documents/refresh/after-same-document-navigation.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/refresh/after-same-document-navigation.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>Referrer from Refresh after Same-Document Navigation</title>
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=266554">
<link rel="help" href="https://html.spec.whatwg.org/multipage/semantics.html#pragma-directives:navigate">
<link rel="author" title="Zach Hoffman" href="mailto:zach@zrhoffman.net">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<script>
promise_test(async t => {
  const refreshTo = new URL("resources/refreshed.txt", location).href;
  const refreshFrom = new URL("resources/refresh-with-section.sub.html", location).href + "?" + new URLSearchParams({url: refreshTo});

  const frame = document.createElement("iframe");
  const { promise: frameLoaded, resolve: resolveFrameLoaded } = Promise.withResolvers();

  let loadCount = 0;
  frame.addEventListener("load", t.step_func(() => {
    loadCount++;
    try {
      if (loadCount === 1) {
        assert_equals(frame.contentWindow.location.href, refreshFrom + "#section", "same-document navigation occurred");
        assert_equals(frame.contentWindow.referrer.textContent, location.href, "referrer header is parent frame");
        assert_equals(frame.contentDocument.referrer, location.href, "document referrer is parent frame");
      }
    } finally {
      if (loadCount === 2) {
        resolveFrameLoaded();
      }
    }
  }));

  frame.src = refreshFrom;
  document.body.appendChild(frame);
  await frameLoaded;

  assert_equals(frame.contentWindow.location.href, refreshTo, "refresh page has expected URL")
  assert_equals(frame.contentDocument.referrer, frame.src, "referrer does not include fragment");
});
</script>
</body>
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/refresh/after-same-document-navigation.html"
}
```
