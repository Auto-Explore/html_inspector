# html/canvas/element/manual/imagebitmap/createImageBitmap-sizeOverflow.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/imagebitmap/createImageBitmap-sizeOverflow.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<title>createImageBitmap with size overflow</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<script>
promise_test(function() {
    var imgData = new ImageData(20, 20);
    return createImageBitmap(imgData, 4294967400, 10, 10, 10);
}, "createImageBitmap does not crash or reject the promise when passing very large sx");

promise_test(function() {
    var imgData = new ImageData(20, 20);
    return createImageBitmap(imgData, 10, 4294967400, 10, 10);
}, "createImageBitmap does not crash or reject the promise when passing very large sy");

promise_test(function() {
    var imgData = new ImageData(20, 20);
    return createImageBitmap(imgData, 10, 10, 4294967400, 10);
}, "createImageBitmap does not crash or reject the promise when passing very large sw");

promise_test(function() {
    var imgData = new ImageData(20, 20);
    return createImageBitmap(imgData, 10, 10, 10, 4294967400);
}, "createImageBitmap does not crash or reject the promise when passing very large sh");

promise_test(function() {
    var imgData = new ImageData(20, 20);
    return createImageBitmap(imgData, 4294967400, 4294967400, 4294967400, 4294967400);
}, "createImageBitmap does not crash or reject the promise when passing very large sx, sy, sw and sh");

promise_test(function(t) {
    var imgData = new ImageData(20, 20);
    var imageBitmapOptions = {imageOrientation:'from-image', premultiplyAlpha:'default',
                              colorSpaceConversion:'none', resizeHeight:2122252543, resizeQuality:'high'};
    return createImageBitmap(imgData, 0, 0, 4294967295, 64)
        .then(imageBitmap => promise_rejects_dom(t, "InvalidStateError",
            createImageBitmap(imageBitmap, imageBitmapOptions)));
}, "createImageBitmap throws an InvalidStateError error with big imageBitmap scaled up in big height");

promise_test(function(t) {
    var imgData = new ImageData(20, 20);
    var imageBitmapOptions = {imageOrientation:'from-image', premultiplyAlpha:'default',
                              colorSpaceConversion:'none', resizeWidth:2122252543, resizeQuality:'high'};
    return createImageBitmap(imgData, 0, 0, 4294967295, 64)
        .then(imageBitmap => promise_rejects_dom(t, "InvalidStateError",
            createImageBitmap(imageBitmap, imageBitmapOptions)));
}, "createImageBitmap throws an InvalidStateError error with big imageBitmap scaled up in big width");
</script>
</body>
</html>
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
  "source_name": "html/canvas/element/manual/imagebitmap/createImageBitmap-sizeOverflow.html"
}
```
