# html/canvas/element/manual/unclosed-canvas-3.htm

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/unclosed-canvas-3.htm",
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
        <link rel="match" href="unclosed-canvas-3-expected.htm">
        <meta name="assert" content="Test what if canvas tag is unclosed in unclosed div" />
        <script type="text/javascript"></script>
    </head>
    <body>
        <p>There should be no text below this, because the text is inside a canvas element
        and the &lt;/div&gt; that's also inside the canvas element does not close an open element.
        The canvas is never closed, and the rest of the body ends up inside it.
        There's nothing special about div; we get the same results with other types of elements.</p>
        <canvas></div>This text should NOT be visible if JavaScript is enabled.
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
        "byte_end": 282,
        "byte_start": 251,
        "col": 9,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “div”.",
      "severity": "Error",
      "span": {
        "byte_end": 708,
        "byte_start": 702,
        "col": 17,
        "line": 14
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
  "source_name": "html/canvas/element/manual/unclosed-canvas-3.htm"
}
```
