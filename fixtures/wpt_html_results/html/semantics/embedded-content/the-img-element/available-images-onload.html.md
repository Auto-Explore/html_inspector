# html/semantics/embedded-content/the-img-element/available-images-onload.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/available-images-onload.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
<title>Ensure images from available images list can be drawn to a canvas</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-list-of-available-images">
<meta charset="utf-8">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
  async_test(function(t) {
    var i = new Image();
    i.onerror = t.unreached_func();
    i.onload = t.step_func(function() {
      var i2 = new Image();
      // Potentially start multiple image loading tasks by performing several
      // relevant mutations. This could lead to an invalid state later in an
      // erroneous implementation.
      i2.crossOrigin = true;
      // Start an image loading task that is expected to short-circuit since
      // the requested image is present in the list of available images.
      i2.src = "3.jpg";
      i2.onerror = t.unreached_func();
      // Ensure the loaded image is in a state that is usable by a 2d canvas.
      i2.onload = t.step_func_done(function() {
        var c = document.createElement('canvas');
        var ctx = c.getContext('2d');
        ctx.drawImage(i2, 0, 0);
      });
    });
    // Request an image which should be added to the list of available images.
    i.src = "3.jpg";
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
  "source_name": "html/semantics/embedded-content/the-img-element/available-images-onload.html"
}
```
