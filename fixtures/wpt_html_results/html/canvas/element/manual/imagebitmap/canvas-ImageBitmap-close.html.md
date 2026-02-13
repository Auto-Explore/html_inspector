# html/canvas/element/manual/imagebitmap/canvas-ImageBitmap-close.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/imagebitmap/canvas-ImageBitmap-close.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<body>
    <p>Tests that the close method of ImageBitmap does dispose the image data.</p>

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>

promise_test(function(t) {
    var worker = new Worker('worker-onmessage-noop.js');

    var imgHeight = 10;
    var imgWidth = 10;
    var imageData = new ImageData(10, 10);
    var bitmap;
    var ctx;
    return createImageBitmap(imageData).then(imageBitmap => {
        bitmap = imageBitmap;
        assert_equals(bitmap.width, imgWidth, "bitmap.width = 10");
        assert_equals(bitmap.height, imgWidth, "bitmap.height = 10");

        // Apply structured clone to the bitmap, nothing should be changed
        worker.postMessage({data: bitmap});
        assert_equals(bitmap.width, imgWidth, "bitmap.width = 10");
        assert_equals(bitmap.height, imgWidth, "bitmap.height = 10");

        // After calling close, the image data associated with the bitmap should no longer exist
        bitmap.close();
        assert_equals(bitmap.width, 0, "bitmap.width = 0");
        assert_equals(bitmap.height, 0, "bitmap.height = 0");

        var canvas = document.createElement("canvas");
        canvas.width = imgWidth;
        canvas.height = imgHeight;
        ctx = canvas.getContext("2d");
        assert_throws_dom("InvalidStateError", function() { ctx.drawImage(bitmap, 0, 0); });

        // Try to apply structured clone to an already closed bitmap
        try {
            worker.postMessage({data: bitmap});
            throw new Error("Apply clone to an closed bitmap should be rejected");
        }
        catch(ex) {
             // Apply structured clone to an already closed bitmap is rejected as expected.
        }

        // Try to apply transferring to an already closed bitmap
        try {
            worker.postMessage({data: bitmap}, [bitmap]);
            throw new Error("Apply transferring to an closed bitmap should be rejected");
        } catch(ex) {
             // Apply structured clone to an already closed bitmap is rejected as expected.
        }

        // Calling createImageBitmap from a closed bitmap should be rejected
        return createImageBitmap(bitmap).then(function() {
           throw new Error("createImageBitmap from a closed bitmap should be rejected");
        }, ex => {
            // Calling createImageBitmap from a closed ImageBitmap is rejected as expected.
        });
    }).then(() => {
        // Call close to a already closed bitmap should be noop.
        bitmap.close();
        assert_equals(bitmap.width, 0, "bitmap.height = 0");
        assert_equals(bitmap.height, 0, "bitmap.height = 0");

        return createImageBitmap(imageData).then(imageBitmap => {
            bitmap = imageBitmap;
            assert_equals(bitmap.width, imgWidth, "bitmap.width = 10");
            assert_equals(bitmap.height, imgWidth, "bitmap.height = 10");

            // Transfer the bitmap to a worker
            worker.postMessage({data: bitmap}, [bitmap]);

            // After transferring, the bitmap is neutered.
            assert_equals(bitmap.width, 0, "bitmap.height = 0");
            assert_equals(bitmap.height, 0, "bitmap.height = 0");

            // Calling close to a neutered bitmap should be noop.
            bitmap.close();
            assert_equals(bitmap.width, 0, "bitmap.height = 0");
            assert_equals(bitmap.height, 0, "bitmap.height = 0");

        });
    }).catch(function(ex) {
        throw new Error("No exception should be thrown.");
    })
});
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
  "source_name": "html/canvas/element/manual/imagebitmap/canvas-ImageBitmap-close.html"
}
```
