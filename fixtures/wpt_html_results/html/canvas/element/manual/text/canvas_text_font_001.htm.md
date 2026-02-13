# html/canvas/element/manual/text/canvas_text_font_001.htm

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/text/canvas_text_font_001.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype HTML>
<html>
    <head>
        <title>HTML5 Canvas Test:  Ignore property-independent style sheet syntax "inherit" in Text</title>
        <link rel="match" href="canvas_text_font_001-ref.htm" />
        <link rel="author" title="Microsoft" href="http://www.microsoft.com" />
        <link rel="help" href="http://www.w3.org/TR/2dcontext/#dom-context-2d-font" />
        <meta name="assert" content=":  Ignore 'inherit' property-independent style sheet syntax without assigning a new font value." />
        <script type="text/javascript">
            function runTest()
            {
                var canvas = document.getElementById("canvas1");
                var ctx = canvas.getContext("2d");

                // Assign a valid font.
                ctx.font = "40px Ahem";

                // Assign property-independent style sheet syntax 'inherit' as font (this must be ignored).
                ctx.font = "20px inherit";
                ctx.fillText("Filler", 5, 50);

                // Assign a valid font which was used earlier.
                ctx.font = "40px Ahem";
                ctx.fillText("Filler", 5, 100);
            }
        </script>
    </head>
    <body onload="runTest()">
        <p>Description:  Ignore "inherit" property-independent style sheet syntax without assigning a new font value.</p>
        <p>Test passes if there are two identical black boxes below.</p>
        <canvas id="canvas1" width="300" height="150">Browser does not support HTML5 Canvas.</canvas>
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
        "byte_end": 550,
        "byte_start": 519,
        "col": 9,
        "line": 9
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
  "source_name": "html/canvas/element/manual/text/canvas_text_font_001.htm"
}
```
