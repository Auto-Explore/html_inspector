# html/canvas/element/manual/unclosed-canvas-2-expected.htm

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/unclosed-canvas-2-expected.htm",
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
        <div><canvas></canvas></div>
        <p>This text should be visible, even though it's preceded by an unclosed canvas tag,
            because of the &lt;/div&gt; that closes an element opened before the canvas.
            There's nothing special about div; we get the same results with other types of
            elements.
        </p>
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
  "source_name": "html/canvas/element/manual/unclosed-canvas-2-expected.htm"
}
```
