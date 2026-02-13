# html/canvas/element/manual/unclosed-canvas-2.htm

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/unclosed-canvas-2.htm",
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
        <link rel="match" href="unclosed-canvas-2-expected.htm">
        <meta name="assert" content="Test what if canvas tag is unclosed in tag div" />
        <script type="text/javascript"></script>
    </head>
    <body>
        <div><canvas></div>
        <p>This text should be visible, even though it's preceded by an unclosed canvas tag,
        because of the &lt;/div&gt; that closes an element opened before the canvas.
        There's nothing special about div; we get the same results with other types of elements.</p>
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
        "byte_end": 277,
        "byte_start": 246,
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
  "source_name": "html/canvas/element/manual/unclosed-canvas-2.htm"
}
```
