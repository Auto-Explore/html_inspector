# html/rendering/replaced-elements/attributes-for-embedded-content-and-images/canvas-aspect-ratio.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/attributes-for-embedded-content-and-images/canvas-aspect-ratio.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Canvas width and height attributes are used as the surface size, and also to infer aspect ratio</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="resources/aspect-ratio.js"></script>
<style>
  canvas {
    width: 100%;
    max-width: 100px;
    height: auto;
  }
</style>
<body>
<canvas id="contained" width="250" height="100" style="contain: size;"></canvas>
<script>
function assert_ratio(img, expected) {
  let epsilon = 0.001;
  assert_approx_equals(parseInt(getComputedStyle(img).width, 10) / parseInt(getComputedStyle(img).height, 10), expected, epsilon);
}

function test_computed_style(width, height, expected) {
  test_computed_style_aspect_ratio("canvas", {width: width, height: height}, expected);
}

test(function() {
  canvas = document.getElementById("contained");
  assert_ratio(canvas, 2.5);
}, "Canvas width and height attributes are used as the surface size with contain:size");

// Create and append a new canvas and immediately check the ratio.
test(function() {
  var canvas = document.createElement("canvas");
  canvas.setAttribute("width", "250");
  canvas.setAttribute("height", "100");
  document.body.appendChild(canvas);
  // Canvases always use the aspect ratio from their surface size.
  assert_ratio(canvas, 2.5);
}, "Canvas width and height attributes are used as the surface size");

test_computed_style("10", "20", "auto 10 / 20");
test_computed_style("0", "1", "auto 0 / 1");
test_computed_style("1", "0", "auto 1 / 0");
test_computed_style("0", "0", "auto 0 / 0");
test_computed_style("0.5", "1.5", "auto 0 / 1");
test_computed_style("10%", "20", "auto 10 / 20");
test_computed_style(null, null, "auto");
test_computed_style("10", null, "auto");
test_computed_style(null, "20", "auto");
test_computed_style("xx", "20", "auto");
test_computed_style("20", "xx", "auto");

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
  "source_name": "html/rendering/replaced-elements/attributes-for-embedded-content-and-images/canvas-aspect-ratio.html"
}
```
