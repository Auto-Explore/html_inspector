# html/canvas/element/manual/wide-gamut-canvas/imagedata-no-color-settings-crash.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/wide-gamut-canvas/imagedata-no-color-settings-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
test(function() {
    var canvas = document.createElement('canvas');
    var ctx = canvas.getContext('2d',
        {})
    var dataFloat32 = new Float32Array(4);
    var imageData = ctx.createImageData(dataFloat32, 1, 1,
        {});
    ctx.putImageData(imageData, 5, 5);
    var data = ctx.getImageData(5,5,1,1).data;
}, "Putting a float-32 ImageData with no color settings on a context 2D should not crash.");

test(function() {
    var canvas = document.createElement('canvas');
    var ctx = canvas.getContext('2d',
        {})
    var dataUint16 = new Uint16Array(4);
    var imageData = ctx.createImageData(dataUint16, 1, 1,
        {});
    ctx.putImageData(imageData, 5, 5);
    var data = ctx.getImageData(5,5,1,1).data;
}, "Putting a uint-16 ImageData with no color settings on a context 2D should not crash.");
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
  "source_name": "html/canvas/element/manual/wide-gamut-canvas/imagedata-no-color-settings-crash.html"
}
```
