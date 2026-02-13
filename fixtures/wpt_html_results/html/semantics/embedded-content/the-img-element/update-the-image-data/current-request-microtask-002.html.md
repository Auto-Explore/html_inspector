# html/semantics/embedded-content/the-img-element/update-the-image-data/current-request-microtask-002.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/update-the-image-data/current-request-microtask-002.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Current request microtask handling with multiple tasks.</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<script>
async_test(function(t) {
  let img;
  function testSrcOnMicrotask(expectedCurrentSrc, done) {
    window.queueMicrotask(t.step_func(() => {
      assert_equals(img.currentSrc, expectedCurrentSrc, `currentSrc should be ${expectedCurrentSrc}`);
      if (done) {
        t.done();
      }
    }));
  }
  testSrcOnMicrotask("");
  img = new Image();
  let png = "/images/green.png?" + Math.random();
  let resolved_png = new URL(png, document.documentURI).href;
  testSrcOnMicrotask("");
  // Both .src and .srcset assignments are relevant mutations. So the first task should be "canceled" (return early).
  // appendChild is not a relevant mutation unless in a <picture> element.
  img.src = png;
  testSrcOnMicrotask("");
  img.srcset = png;
  testSrcOnMicrotask(resolved_png);
  document.body.appendChild(img);
  testSrcOnMicrotask(resolved_png, /* done = */ true);
});
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
  "source_name": "html/semantics/embedded-content/the-img-element/update-the-image-data/current-request-microtask-002.html"
}
```
