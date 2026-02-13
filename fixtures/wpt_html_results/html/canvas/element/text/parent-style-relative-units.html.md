# html/canvas/element/text/parent-style-relative-units.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/text/parent-style-relative-units.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Canvas test: CanvasTextDrawingStyles.font with canvas relative units</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/canvas.html#dom-context-2d-font">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<canvas id="canvas" style="font-size: 30px; line-height: 40px"></canvas>
<script>
  const canvas = document.getElementById("canvas");
  const ctx = canvas.getContext("2d");

  test(() => {
    ctx.font = "2em serif";
    assert_equals(ctx.font, "60px serif");
  }, "Font-size based on canvas element font-size");

  test(() => {
    // Line-height should be forced to normal, but also irrelevant for resolving
    // lh-units for font-size.
    ctx.font = "2lh/100 serif";
    assert_equals(ctx.font, "80px serif");
  }, "Font-size based on canvas element line-height");
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
  "source_name": "html/canvas/element/text/parent-style-relative-units.html"
}
```
