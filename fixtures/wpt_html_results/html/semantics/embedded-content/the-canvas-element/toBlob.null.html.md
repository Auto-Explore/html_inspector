# html/semantics/embedded-content/the-canvas-element/toBlob.null.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-canvas-element/toBlob.null.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Canvas test: toBlob.null</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/html/canvas/resources/canvas-tests.js"></script>
<link rel="stylesheet" href="/html/canvas/resources/canvas-tests.css">
<body class="show_output">

<h1>toBlob.null</h1>
<p class="desc">toBlob with zero dimension returns a null Blob</p>

<p class="output">Actual output:</p>
<canvas id="c" class="output" width="100" height="0"><p class="fallback">FAIL (fallback content)</p></canvas>

<ul id="d"></ul>
<script>
async_test(function() {
    on_event(window, "load", this.step_func(function() {
         var toBlobCalled = false;
         c.toBlob(this.step_func(function(blob) {
           toBlobCalled = true;
           _assertSame(blob, null, "blob", "null");
           c.width = 0;
           c.height = 100;
           c.toBlob(this.step_func_done(function(blob) {
             _assertSame(blob, null, "blob", "null");
           }), 'image/jpeg');
         }), 'image/jpeg');
         assert_false(toBlobCalled, "toBlob callback shouldn't be called synchronously");
    }));
}, "toBlob with zero dimension returns a null Blob");
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
  "source_name": "html/semantics/embedded-content/the-canvas-element/toBlob.null.html"
}
```
