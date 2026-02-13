# html/canvas/offscreen/manual/filter/offscreencanvas.filter.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/offscreen/manual/filter/offscreencanvas.filter.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="offscreencanvas.filter.js"></script>
<script>
var patternCanvas = createPatternCanvas();

var getOffscreenContextForFilter = function(filter, pattern) {
  var oc = new OffscreenCanvas(80, 80);
  var offCtx = oc.getContext('2d');
  offCtx.filter = filter;
  offCtx.drawImage(pattern, 5, 5);
  offCtx.drawImage(pattern, 25, 25);
  offCtx.drawImage(pattern, 45, 45);
  return offCtx;
};

var testFilter = function(filter) {
  var offCtx = getOffscreenContextForFilter(filter, patternCanvas);
  var ctx = getRegularContextForFilter(filter, patternCanvas);
  var offImageData = offCtx.getImageData(0, 0, 80, 80).data;
  var imageData = ctx.getImageData(0, 0, 80, 80).data;
  matchImageDataResults(offImageData, imageData, filter);
};

generate_tests(testFilter, [filters]);

</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.start_tag_without_doctype",
      "message": "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 40,
        "byte_start": 0,
        "col": 1,
        "line": 1
      }
    },
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
  "source_name": "html/canvas/offscreen/manual/filter/offscreencanvas.filter.html"
}
```
