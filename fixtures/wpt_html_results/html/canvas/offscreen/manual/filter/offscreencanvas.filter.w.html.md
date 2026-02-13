# html/canvas/offscreen/manual/filter/offscreencanvas.filter.w.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/offscreen/manual/filter/offscreencanvas.filter.w.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="offscreencanvas.filter.js"></script>
<script id='myWorker' type='text/worker'>
self.onmessage = function(e) {
  var getOffscreenCanvasForFilter = function(filter, pattern) {
    var oc = new OffscreenCanvas(80, 80);
    var offCtx = oc.getContext('2d');
    offCtx.filter = filter;
    offCtx.drawImage(pattern, 5, 5);
    offCtx.drawImage(pattern, 25, 25);
    offCtx.drawImage(pattern, 45, 45);
    return oc;
  };

  var filters = e.data.filters;
  var pattern = e.data.pattern;
  var ret = [];
  for (var i = 0; i < filters.length; i++) {
     var oc = getOffscreenCanvasForFilter(filters[i], pattern);
     var imageBitmap = oc.transferToImageBitmap();
     ret.push(imageBitmap);
  }
  self.postMessage(ret, ret);
};
</script>
<script>
var patternCanvas = createPatternCanvas();

// Build a list of image data on regular canvas with different filters
var listCanvasImageData = [];
for (var j = 0; j < filters.length; j++) {
  var ctx = getRegularContextForFilter(filters[j], patternCanvas);
  listCanvasImageData.push(ctx.getImageData(0, 0, 80, 80).data);
}

function consumeImageBitmap(patternImage) {
  async_test(t => {
    var blob = new Blob([document.getElementById('myWorker').textContent]);
    var worker = new Worker(URL.createObjectURL(blob));
    worker.addEventListener('message', msg => {
      for (var i = 0; i < msg.data.length; ++i) {
        var outputCtx = document.createElement("canvas").getContext('2d');
        outputCtx.drawImage(msg.data[i], 0, 0, 80, 80);
        matchImageDataResults(outputCtx.getImageData(0, 0, 80, 80).data, listCanvasImageData[i], filters[i]);
      }
      t.done();
    });
  worker.postMessage({filters: filters, pattern: patternImage}, [patternImage]);
  });
}

createImageBitmap(patternCanvas).then(consumeImageBitmap);
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
  "source_name": "html/canvas/offscreen/manual/filter/offscreencanvas.filter.w.html"
}
```
