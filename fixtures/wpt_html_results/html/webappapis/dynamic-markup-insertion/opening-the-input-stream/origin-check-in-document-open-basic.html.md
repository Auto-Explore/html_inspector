# html/webappapis/dynamic-markup-insertion/opening-the-input-stream/origin-check-in-document-open-basic.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/dynamic-markup-insertion/opening-the-input-stream/origin-check-in-document-open-basic.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Origin check in document.open() - Basic usage</title>
<link rel="author" title="Jochen Eisinger" href="mailto:jochen@chromium.org">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#opening-the-input-stream">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/html/resources/common.js"></script>
<body>
<script>
testInIFrame(undefined, (ctx) => {
  try {
    ctx.iframes[0].contentDocument.open();
  } catch (e) {
    assert_unreached("Opening a same origin document throws");
  }
}, "It should be possible to open same origin documents.");

testInIFrame(undefined, (ctx) => {
  try {
    ctx.iframes[0].contentDocument.write("");
  } catch (e) {
    assert_unreached("Implicitly opening a same origin document throws");
  }
}, "It should be possible to implicitly open same origin documents.");
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
  "source_name": "html/webappapis/dynamic-markup-insertion/opening-the-input-stream/origin-check-in-document-open-basic.html"
}
```
