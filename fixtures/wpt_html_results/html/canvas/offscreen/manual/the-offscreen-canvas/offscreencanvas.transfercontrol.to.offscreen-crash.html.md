# html/canvas/offscreen/manual/the-offscreen-canvas/offscreencanvas.transfercontrol.to.offscreen-crash.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/offscreen/manual/the-offscreen-canvas/offscreencanvas.transfercontrol.to.offscreen-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<title>Transfer Canvas without Context</title>
<canvas id="canvas"></canvas>

<script>
canvas.transferControlToOffscreen();
canvas.toBlob(_ => {});
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
        "byte_end": 7,
        "byte_start": 0,
        "col": 1,
        "line": 1
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
  "source_name": "html/canvas/offscreen/manual/the-offscreen-canvas/offscreencanvas.transfercontrol.to.offscreen-crash.html"
}
```
