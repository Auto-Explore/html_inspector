# html/canvas/element/manual/unclosed-canvas-1-expected.htm

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/unclosed-canvas-1-expected.htm",
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
    </head>
    <body>
        <p>There should be no text below this, because the text is inside a canvas element.
            The canvas is never closed, and the rest of the body ends up inside it.</p>
    </body>
</html>
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
  "source_name": "html/canvas/element/manual/unclosed-canvas-1-expected.htm"
}
```
