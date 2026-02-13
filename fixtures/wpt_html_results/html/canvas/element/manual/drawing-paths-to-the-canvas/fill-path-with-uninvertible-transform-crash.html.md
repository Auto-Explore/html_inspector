# html/canvas/element/manual/drawing-paths-to-the-canvas/fill-path-with-uninvertible-transform-crash.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/drawing-paths-to-the-canvas/fill-path-with-uninvertible-transform-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script>
  let context = document.createElement("canvas").getContext("2d");
  context.moveTo(0, 0);
  context.setTransform(4, 2, 6, 3, 0, 0); // non-invertible (4 * 3 - 6 * 2 = 0)
  context.fill("nonzero");
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
  "source_name": "html/canvas/element/manual/drawing-paths-to-the-canvas/fill-path-with-uninvertible-transform-crash.html"
}
```
