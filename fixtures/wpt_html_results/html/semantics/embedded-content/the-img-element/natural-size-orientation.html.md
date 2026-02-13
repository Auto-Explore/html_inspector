# html/semantics/embedded-content/the-img-element/natural-size-orientation.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/natural-size-orientation.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>naturalWidth and naturalHeight on HTMLImageElement reflect orientation metadata</title>
<link rel="author" title="Cameron McCormack" href="mailto:cam@mcc.id.au">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<style>
.ignore-orientation { image-orientation: none; }
</style>
<body>
<script>
async_test(function(t) {
  let img = document.createElement("img");
  img.src = "/images/green-100x50.png";
  img.onload = t.step_func_done(function() {
    assert_equals(img.naturalWidth, 100);
    assert_equals(img.naturalHeight, 50);
    img.remove();
  });
  document.body.append(img);
}, "naturalWidth and naturalHeight return correct values for an image without orientation metadata");

async_test(function(t) {
  let img = document.createElement("img");
  img.src = "/images/arrow-oriented-upright.jpg";
  img.onload = t.step_func_done(function() {
    assert_equals(img.naturalWidth, 144);
    assert_equals(img.naturalHeight, 240);
    img.remove();
  });
  document.body.append(img);
}, "naturalWidth and naturalHeight return re-oriented values for an image with orientation metadata");

async_test(function(t) {
  let img = document.createElement("img");
  img.src = "/images/arrow-oriented-upright.jpg";
  img.className = "ignore-orientation";
  img.onload = t.step_func_done(function() {
    assert_equals(img.naturalWidth, 144);
    assert_equals(img.naturalHeight, 240);
    img.remove();
  });
  document.body.append(img);
}, "naturalWidth and naturalHeight return re-oriented values for an image with orientation metadata even with image-orientation:none");
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
  "source_name": "html/semantics/embedded-content/the-img-element/natural-size-orientation.html"
}
```
