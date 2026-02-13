# html/rendering/replaced-elements/attributes-for-embedded-content-and-images/canvas-dimension-attributes.html

Counts:
- errors: 3
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/attributes-for-embedded-content-and-images/canvas-dimension-attributes.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Canvas width and height attributes are used as the surface size, and also to infer aspect ratio</title>
<link rel="author" title="Oriol Brufau" href="obrufau@igalia.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#attributes-for-embedded-content-and-images">
<link rel="match" href="/css/reference/ref-filled-green-200px-square.html">
<meta name="assert" content="
    Unlike <img>, <canvas> doesn't map its dimension attributes to the dimension properties.
    Therefore, the 1st <canvas> should be 100px wide since it infers an aspect ratio of 150 / 150.
    The 2nd <canvas> should be 100px tall since it infers an aspect ratio of 150 / 300.
    The 3rd <canvas> should be 150px wide since it infers an aspect ratio of 150 / 100.
    The 4th <canvas> should be 100px tall since it infers an aspect ratio of 150 / 300.">

<style>
div {
  background: red;
  width: 200px;
  font-size: 0;
}
canvas {
  vertical-align: top;
}
</style>
<p>Test passes if there is a filled green square and <strong>no red</strong>.</p>
<div>
  <canvas id="canvas" width="150" style="background: green; height: 100px"></canvas>
  <canvas id="canvas" height="300" style="background: green; width: 100px"></canvas>
  <canvas id="canvas" width="150" height="100" style="background: green; height: 100px"></canvas>
  <canvas id="canvas" width="150" height="300" style="background: green; width: 50px"></canvas>
</div>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.id.duplicate",
      "message": "Duplicate ID “canvas”.",
      "severity": "Error",
      "span": {
        "byte_end": 1239,
        "byte_start": 1166,
        "col": 3,
        "line": 26
      }
    },
    {
      "category": "Html",
      "code": "html.id.duplicate",
      "message": "Duplicate ID “canvas”.",
      "severity": "Error",
      "span": {
        "byte_end": 1337,
        "byte_start": 1251,
        "col": 3,
        "line": 27
      }
    },
    {
      "category": "Html",
      "code": "html.id.duplicate",
      "message": "Duplicate ID “canvas”.",
      "severity": "Error",
      "span": {
        "byte_end": 1433,
        "byte_start": 1349,
        "col": 3,
        "line": 28
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/rendering/replaced-elements/attributes-for-embedded-content-and-images/canvas-dimension-attributes.html"
}
```
