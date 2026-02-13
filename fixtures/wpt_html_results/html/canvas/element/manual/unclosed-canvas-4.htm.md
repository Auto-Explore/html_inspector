# html/canvas/element/manual/unclosed-canvas-4.htm

Counts:
- errors: 2
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/unclosed-canvas-4.htm",
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
        <link rel="match" href="unclosed-canvas-4-expected.htm">
        <meta name="assert" content="Test what if canvas tag is unclosed in body" />
        <script type="text/javascript"></script>
    </head>
    <body>
        <p>There should be no text below this, because the text is inside a canvas element
        and the &lt;/div&gt; that's also inside the canvas element does not close an open element.
        The canvas is never closed, and the rest of the body ends up inside it.
        There's nothing special about div; we get the same results with other types of elements.
        The fact that the canvas tag uses XML self-closing syntax has no effect.</p>
        <canvas/></div>This text should NOT be visible if JavaScript is enabled.
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
        "byte_end": 274,
        "byte_start": 243,
        "col": 9,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.parse.self_closing.non_void",
      "message": "Self-closing syntax (“/>”) used on a non-void HTML element. Ignoring the slash and treating as a start tag.",
      "severity": "Error",
      "span": {
        "byte_end": 776,
        "byte_start": 767,
        "col": 9,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “div”.",
      "severity": "Error",
      "span": {
        "byte_end": 782,
        "byte_start": 776,
        "col": 18,
        "line": 15
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
  "source_name": "html/canvas/element/manual/unclosed-canvas-4.htm"
}
```
