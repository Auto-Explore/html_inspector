# html/canvas/offscreen/manual/the-offscreen-canvas/offscreencanvas.getcontext.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/offscreen/manual/the-offscreen-canvas/offscreencanvas.getcontext.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/html/canvas/resources/canvas-tests.js"></script>
<link rel="help" href="https://html.spec.whatwg.org/#dom-offscreencanvas-getcontext">
<script>

test(function() {
    var offscreenCanvas = new OffscreenCanvas(1, 1);
    assert_throws_js(TypeError, function() { offscreenCanvas.getContext('3d'); });
}, "Test that getContext with un-supported string throws a TypeError.");

test(function() {
    var offscreenCanvas1 = new OffscreenCanvas(1, 1);
    var ctx1 = offscreenCanvas1.getContext('2d');
    assert_true(ctx1 instanceof OffscreenCanvasRenderingContext2D);

    var offscreenCanvas2 = new OffscreenCanvas(1, 1);
    var ctx2 = offscreenCanvas2.getContext('webgl');
    assert_true(ctx2 instanceof WebGLRenderingContext);

    var offscreenCanvas3 = new OffscreenCanvas(1, 1);
    var ctx3 = offscreenCanvas3.getContext('webgl2');
    assert_true(ctx3 instanceof WebGL2RenderingContext);
}, "Test that getContext with supported string returns correct results");

test(function() {
    var offscreenCanvas1 = new OffscreenCanvas(1, 1);
    var ctx1 = offscreenCanvas1.getContext('2d');
    var ctx2 = offscreenCanvas1.getContext('webgl');
    assert_equals(ctx2, null);

    var offscreenCanvas2 = new OffscreenCanvas(1, 1);
    var ctx3 = offscreenCanvas2.getContext('webgl');
    var ctx4 = offscreenCanvas2.getContext('2d');
    assert_equals(ctx4, null);
}, "Test that getContext twice with different context type returns null the second time");

test(function() {
    var offscreenCanvas = new OffscreenCanvas(1, 2);
    var ctx = offscreenCanvas.getContext('2d');
    var dstCanvas = ctx.canvas;
    assert_equals(dstCanvas.width, 1);
    assert_equals(dstCanvas.height, 2);
}, "Test that 2dcontext.canvas should return the original OffscreenCanvas");

test(function() {
    var offscreenCanvas = new OffscreenCanvas(1, 2);
    var ctx = offscreenCanvas.getContext('webgl');
    var dstCanvas = ctx.canvas;
    assert_equals(dstCanvas.width, 1);
    assert_equals(dstCanvas.height, 2);
}, "Test that webglcontext.canvas should return the original OffscreenCanvas");

test(function() {
    var offscreenCanvas = new OffscreenCanvas(10, 10);
    var ctx = offscreenCanvas.getContext('2d', {alpha: false});
    ctx.fillStyle = 'rgba(0, 255, 0, 0.5)';
    ctx.fillRect(0, 0, 10, 10);
    _assertPixelApprox(offscreenCanvas, 5,5, 0,127,0,255, 2);
}, "Test that OffscreenCanvasRenderingContext2D with alpha disabled makes the OffscreenCanvas opaque");

test(function() {
    var offscreenCanvas = new OffscreenCanvas(10, 10);
    var ctx = offscreenCanvas.getContext('2d', {alpha: true});
    ctx.fillStyle = 'rgba(0, 255, 0, 0.5)';
    ctx.fillRect(0, 0, 10, 10);
    _assertPixelApprox(offscreenCanvas, 5,5, 0,255,0,127, 2);
}, "Test that OffscreenCanvasRenderingContext2D with alpha enabled preserves the alpha");

test(function() {
    var offscreenCanvas = new OffscreenCanvas(10, 10);
    var ctx = offscreenCanvas.getContext('2d');
    ctx.fillStyle = 'rgba(0, 255, 0, 0.5)';
    ctx.fillRect(0, 0, 10, 10);
    _assertPixelApprox(offscreenCanvas, 5,5, 0,255,0,127, 2);
}, "Test that 'alpha' context creation attribute is true by default");

</script>

```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/canvas/offscreen/manual/the-offscreen-canvas/offscreencanvas.getcontext.html"
}
```
