# html/syntax/parsing/foreign_content_010.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing/foreign_content_010.html",
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
        <meta description="Test to verify SVG Self Closing tags parses properly" />
        <link rel="author" title="Microsoft" href="http://www.microsoft.com/" />

        <script type="text/javascript">
            function RunTest()
            {
                try
                {
                    var parentNode = document.getElementById("svg1");
                    if(parentNode.childNodes[1].localName=="rect" && parentNode.childNodes[3].localName=="circle")
                    {
                        document.getElementById("testresult").innerHTML = "PASS";
                    }
                    else
                    {
                        document.getElementById("testresult").innerHTML = "FAIL"
                    }
                }
                catch(ex)
                {
                    document.getElementById("testresult").innerHTML = "FAIL";
                }
            }
        </script>
    </head>

    <body onload="RunTest()">

        <div class="testdata">
            <p id="instructions"> Test passes if the word "PASS" appears below  </p>
            <p> Test Result : </p>
            <p id="testresult"> RUNNING </div>
        </div>
        <svg id="svg1" width="100px" height="100px">
            <rect id="rect1" />
            <circle id="circle1" />
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
        "byte_end": 298,
        "byte_start": 267,
        "col": 9,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “div”.",
      "severity": "Error",
      "span": {
        "byte_end": 1289,
        "byte_start": 1283,
        "col": 9,
        "line": 37
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
  "source_name": "html/syntax/parsing/foreign_content_010.html"
}
```
