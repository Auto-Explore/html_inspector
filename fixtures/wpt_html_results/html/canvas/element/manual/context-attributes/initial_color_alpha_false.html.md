# html/canvas/element/manual/context-attributes/initial_color_alpha_false.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/context-attributes/initial_color_alpha_false.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>CanvasRenderingContext 2D with alpha=flase context creation parameter is initialized with solid black.</title>
<link rel="author" title="Justin Novosad" href="mailto:junov@chromium.org">
<link rel="help" href="https://html.spec.whatwg.org/#concept-canvas-alpha">
<link rel="match" href="initial_color_alpha_false-ref.html">
<meta name="assert" content="Canvas pixels are initialized to opaque black when alpha attribute is false.">
<p>Test passes if a 100x100 black square is displayed below.</p>
<canvas id="c" width=100 height=100></canvas>
<script>
document.getElementById("c").getContext("2d", {alpha: false});
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
  "source_name": "html/canvas/element/manual/context-attributes/initial_color_alpha_false.html"
}
```
