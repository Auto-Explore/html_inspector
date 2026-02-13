# html/syntax/parsing/foreign_content_008.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing/foreign_content_008.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
    <head>
        <title>HTML 5 Foreign Content SVG in HTML </title>
        <meta description="Test to verify SVG inside HTML 'BODY' element parses correctly" />
        <link rel="author" title="Microsoft" href="http://www.microsoft.com/" />

        <script type="text/javascript">
            function RunTest()
            {
                try
                {
                    if(document.body.childNodes[3].localName=="svg")
                    {
                        document.getElementsByTagName("rect")[0].setAttribute("fill","green");
                    }
                    else
                    {
                        document.getElementsByTagName("rect")[0].setAttribute("fill","red");
                    }
                }
                catch(ex)
                {
                    document.getElementsByTagName("rect")[0].setAttribute("fill","red")
                }
            }
        </script>
    </head>

    <body onLoad="RunTest()">
        <div class="testdata">
            <p id="instructions"> Test passes if a green rectangle is visible on the page below this line. </p>
        </div>
        <svg width="100px" height="100px" >
            <rect x="0" y="0" width="100px" height="100px" />
        </svg>

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
        "byte_end": 308,
        "byte_start": 277,
        "col": 9,
        "line": 8
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
  "source_name": "html/syntax/parsing/foreign_content_008.html"
}
```
