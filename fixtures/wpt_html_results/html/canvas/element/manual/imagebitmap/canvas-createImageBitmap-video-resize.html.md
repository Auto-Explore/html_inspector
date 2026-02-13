# html/canvas/element/manual/imagebitmap/canvas-createImageBitmap-video-resize.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/imagebitmap/canvas-createImageBitmap-video-resize.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/media.js"></script>
<script>
function createNewCanvas(width, height)
{
    var canvas = document.createElement("canvas");
    canvas.width = width;
    canvas.height = height;
    var ctx = canvas.getContext("2d");
    ctx.clearRect(0, 0, width, height);
    return ctx;
}

function checkLowResult(imageBitmap, bw, bh, video, sx, sy, sw, sh)
{
    var ctx1 = createNewCanvas(bw, bh);
    var ctx2 = createNewCanvas(bw, bh);
    ctx1.drawImage(imageBitmap, 0, 0);
    ctx2.drawImage(video, sx, sy, sw, sh, 0, 0, bw, bh);
    var data1 = ctx1.getImageData(0, 0, bw, bh).data;
    var data2 = ctx2.getImageData(0, 0, bw, bh).data;
    var dataMatched = true;
    for (var i = 0; i < data1.length; i++) {
        // data1[i] is strictly the same as data2[i] on software rendering.
        // But on GPU, the difference could be quite significant.
        if (Math.abs(data1[i] - data2[i]) > 33) {
            dataMatched = false;
            break;
        }
    }
    assert_true(dataMatched);
}

function generateTest()
{
    bitmapWidth = video.videoWidth/2;
    bitmapHeight = video.videoHeight/2;
    return Promise.all([
        createImageBitmap(video, {resizeWidth: bitmapWidth, resizeHeight: bitmapHeight, resizeQuality: "low"}),
        createImageBitmap(video, 10, 10, bitmapWidth, bitmapHeight, {resizeWidth: bitmapWidth, resizeHeight: bitmapHeight, resizeQuality: "low"}),
    ]).then(t.step_func_done(([noCropLow, cropLow]) => {
        checkLowResult(noCropLow, bitmapWidth, bitmapHeight, video, 0, 0, video.videoWidth, video.videoHeight);
        checkLowResult(cropLow, bitmapWidth, bitmapHeight, video, 10, 10, bitmapWidth, bitmapHeight);
    }), t.step_func_done(function() {
        assert_true(false, 'Promise rejected');
    }));
}

var t = async_test('createImageBitmap(HTMLVideoElement) with resize option');

// HTMLVideoElement
var video = document.createElement("video");
video.preload = "auto";
if (video.requestVideoFrameCallback) {
    video.requestVideoFrameCallback(t.step_func(() => generateTest()));
} else {
    video.oncanplaythrough = t.step_func(() => generateTest());
}
video.src = getVideoURI("/media/counting");
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
  "source_name": "html/canvas/element/manual/imagebitmap/canvas-createImageBitmap-video-resize.html"
}
```
