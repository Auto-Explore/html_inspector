# html/semantics/embedded-content/the-canvas-element/toBlob.jpeg.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-canvas-element/toBlob.jpeg.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Canvas test: toBlob.jpeg</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<canvas id="c"></canvas>
<script>
async_test(function() {
    on_event(window, "load", this.step_func(function() {
        var canvas = document.getElementById('c');
        var ctx = canvas.getContext('2d');
        canvas.toBlob(this.step_func_done(function(data) {
            assert_equals(data.type, "image/jpeg");
        }), 'image/jpeg');
    }));
}, "toBlob with image/jpeg returns a JPEG Blob");
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
  "source_name": "html/semantics/embedded-content/the-canvas-element/toBlob.jpeg.html"
}
```
