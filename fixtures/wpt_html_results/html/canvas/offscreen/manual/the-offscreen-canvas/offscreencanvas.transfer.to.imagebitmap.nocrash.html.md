# html/canvas/offscreen/manual/the-offscreen-canvas/offscreencanvas.transfer.to.imagebitmap.nocrash.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/offscreen/manual/the-offscreen-canvas/offscreencanvas.transfer.to.imagebitmap.nocrash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<body>
    <p>Tests that an ImageBitmap in Offscreen without content does not crash.</p>

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>

async_test(function(t) {
  var v1 = document.createElement("canvas");
  var v2 = v1.transferControlToOffscreen();
  var v3 = v2.getContext("bitmaprenderer");
  v2.width = 67;
  t.done();
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
  "source_name": "html/canvas/offscreen/manual/the-offscreen-canvas/offscreencanvas.transfer.to.imagebitmap.nocrash.html"
}
```
