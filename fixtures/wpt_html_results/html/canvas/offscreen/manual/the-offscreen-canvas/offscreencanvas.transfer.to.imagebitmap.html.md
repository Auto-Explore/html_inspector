# html/canvas/offscreen/manual/the-offscreen-canvas/offscreencanvas.transfer.to.imagebitmap.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/offscreen/manual/the-offscreen-canvas/offscreencanvas.transfer.to.imagebitmap.html",
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
<link rel="help" href="https://html.spec.whatwg.org/#dom-offscreencanvas-transfertoimagebitmap">

<script id="myWorker" type="text/worker">

self.onmessage = function(e) {
};

</script>

<script>
function makeWorker(script)
{
    var blob = new Blob([script]);
    return new Worker(URL.createObjectURL(blob));
}

test(function() {
    function testSize(contextType) {
        var offscreenCanvas = new OffscreenCanvas(100, 50);
        var ctx = offscreenCanvas.getContext(contextType);
        var image = offscreenCanvas.transferToImageBitmap();
        assert_equals(image.width, 100);
        assert_equals(image.height, 50);
    }

    testSize('2d');
    testSize('webgl');
}, "Test that transferToImageBitmap returns an ImageBitmap with correct width and height");

test(function() {
    function testImageBitmapClr(shouldCallTwice, alphaVal) {
        var offscreenCanvas = new OffscreenCanvas(100, 50);
        var ctx = offscreenCanvas.getContext('2d', {alpha: alphaVal});
        ctx.fillStyle = "#0f0";
        ctx.fillRect(0, 0, 100, 50);
        var image = offscreenCanvas.transferToImageBitmap();

        if (shouldCallTwice)
            image = offscreenCanvas.transferToImageBitmap();

        var drawCanvas = document.createElement('canvas');
        drawCanvas.width = 100;
        drawCanvas.height = 50;
        var dCtx = drawCanvas.getContext('2d');
        dCtx.drawImage(image, 0, 0);

        if (shouldCallTwice) {
            if (alphaVal)
                _assertPixel(drawCanvas, 50,25, 0,0,0,0);
            else
                _assertPixel(drawCanvas, 50,25, 0,0,0,255);
        } else {
            _assertPixel(drawCanvas, 50,25, 0,255,0,255);
        }
    }

    testImageBitmapClr(false, true);
    testImageBitmapClr(true, true);
    testImageBitmapClr(true, false);
}, "Test that transferToImageBitmap returns an ImageBitmap with correct color");

test(function() {
    var offscreenCanvas = new OffscreenCanvas(100, 50);
    var ctx = offscreenCanvas.getContext('2d');
    ctx.lineWidth = 10;
    var image = offscreenCanvas.transferToImageBitmap();
    assert_equals(ctx.lineWidth, 10);
}, "Test that transferToImageBitmap won't change context's property");

test(function() {
    var offscreenCanvas = new OffscreenCanvas(100, 50);
    var ctx = offscreenCanvas.getContext('2d');
    ctx.rect(0, 0, 25, 50);
    ctx.clip();
    ctx.translate(20, 20);

    ctx.fillStyle = '#0f0';
    var image1 = offscreenCanvas.transferToImageBitmap();
    // trasnform should be preserved
    ctx.fillRect(0, 0, 10, 10);
    var image2 = offscreenCanvas.transferToImageBitmap();

    var drawCanvas = document.createElement('canvas');
    drawCanvas.width = 100;
    drawCanvas.height = 50;
    var dCtx = drawCanvas.getContext('2d');
    dCtx.drawImage(image2, 0, 0);
    // Verify that transform was carried over.
    _assertPixel(drawCanvas, 23,25, 0,255,0,255);
    // Verify that clip was carried over.
    _assertPixel(drawCanvas, 27,25, 0,0,0,0);
}, "Test that transferToImageBitmap preserves transform");

async_test(function(t) {
    var worker = makeWorker(document.getElementById("myWorker").textContent);
    var offscreenCanvas = new OffscreenCanvas(10, 10);
    worker.postMessage(offscreenCanvas, [offscreenCanvas]);
    assert_throws_dom("InvalidStateError", function() { offscreenCanvas.transferToImageBitmap(); });
    t.done();
}, "Test that call transferToImageBitmap on a detached OffscreenCanvas throws an exception");

test(function() {
    var offscreenCanvas = new OffscreenCanvas(10, 10);
    assert_throws_dom("InvalidStateError", function() { offscreenCanvas.transferToImageBitmap(); });
}, "Test that transferToImageBitmap without a context throws an exception");

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
  "source_name": "html/canvas/offscreen/manual/the-offscreen-canvas/offscreencanvas.transfer.to.imagebitmap.html"
}
```
