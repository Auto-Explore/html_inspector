# html/semantics/embedded-content/the-img-element/not-rendered-dimension-getter.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/not-rendered-dimension-getter.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>Image intrinsic dimensions are returned if the image isn't rendered</title>
<link rel="author" title="Emilio Cobos Álvarez" href="mailto:emilio@crisal.io">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-img-width">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="container" style="display: none">
</div>
<script>
async_test(function(t) {
  var img = document.createElement('img');
  img.onload = t.step_func_done(function() {
    assert_equals(img.width, 389, "intrinsic width should've been returned")
    assert_equals(img.height, 590, "intrinsic height should've been returned")
    document.getElementById('container').appendChild(img);
    assert_equals(img.width, 389, "intrinsic width should've been returned");
    assert_equals(img.height, 590, "intrinsic height should've been returned");
  });
  img.src = "image-1.jpg";
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
  "source_name": "html/semantics/embedded-content/the-img-element/not-rendered-dimension-getter.html"
}
```
