# html/syntax/parsing/foreign_content_004.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing/foreign_content_004.html",
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
        <meta description="Test to verify SVG inside HTML CAPTION element parses correctly" />
        <link rel="author" title="Microsoft" href="http://www.microsoft.com/" />

        <script type="text/javascript">
            function RunTest()
            {
                try
                {
                    var svgNS = "http://www.w3.org/2000/svg";
                    if(document.getElementsByTagName("caption")[0].childNodes[1].localName=="svg" && document.getElementsByTagName("rect")[0].namespaceURI==svgNS)
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
            <p id="instructions">Test passes if a green square appears above and centered relative to the table.</p>
        </div>
        <div>
            <table border="1">
                <caption>
                    <svg width="100px" height="100px">
                        <rect width="100px" height="100px" fill="none" />
                    </svg>
                </caption>
                <thead>
                    <th>FillerText</th>
                    <th>FillerText</th>
                    <th>FillerText</th>
                </thead>
                <tr>
                    <td>FillerText</td>
                    <td>FillerText</td>
                    <td>FillerText</td>
                </tr>
                <tr>
                    <td>FillerText</td>
                    <td>FillerText</td>
                    <td>FillerText</td>
                </tr>
                <tr>
                    <td>FillerText</td>
                    <td>FillerText</td>
                    <td>FillerText</td>
                </tr>
            </table>
        </div>
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
        "byte_end": 309,
        "byte_start": 278,
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
  "source_name": "html/syntax/parsing/foreign_content_004.html"
}
```
