# html/canvas/element/manual/unclosed-canvas-1.htm

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/unclosed-canvas-1.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype HTML>
<html>
    <head>
        <title>unclosed canvas tag in body</title>
        <link rel="match" href="unclosed-canvas-1-expected.htm">
        <meta name="assert" content="Test what if canvas tag is unclosed in tag p" />
        <script type="text/javascript"></script>
    </head>
    <body>
        <p>There should be no text below this, because the text is inside a canvas element.
            The canvas is never closed, and the rest of the body ends up inside it. </p>
        <canvas>This text should NOT be visible if JavaScript is enabled.
    </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 275,
        "byte_start": 244,
        "col": 9,
        "line": 7
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
  "source_name": "html/canvas/element/manual/unclosed-canvas-1.htm"
}
```
