# html/syntax/parsing/foreign_content_006.html

Counts:
- errors: 2
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing/foreign_content_006.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
    <svg width="100px" height="100px">
        <rect width="100px" height="100px" fill="none" />
    </svg>

    <head>
        <title>HTML 5 Foreign Content SVG in HTML </title>
        <meta description="Test to verify SVG inside HTML 'HTML' element parses correctly" />
        <link rel="author" title="Microsoft" href="http://www.microsoft.com/" />
        <script type="text/javascript">
            function RunTest()
            {
                try
                {
                    if(document.body.childNodes[0].localName=="svg")
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
            <p id="instructions">Test passes if a green rectangle is visible on the page above this line.</p>
        </div>
    </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.body.already_open",
      "message": "Start tag “body” seen but an element of the same type was already open.",
      "severity": "Error",
      "span": {
        "byte_end": 1112,
        "byte_start": 1087,
        "col": 5,
        "line": 33
      }
    },
    {
      "category": "Html",
      "code": "html.parser.cannot_recover",
      "message": "Cannot recover after last error. Any further errors will be ignored.",
      "severity": "Error",
      "span": {
        "byte_end": 1112,
        "byte_start": 1087,
        "col": 5,
        "line": 33
      }
    },
    {
      "category": "Html",
      "code": "html.link.in_body.disallowed_rel",
      "message": "A “link” element must not appear as a descendant of a “body” element unless the “link” element has an “itemprop” attribute or has a “rel” attribute whose value contains “dns-prefetch”, “modulepreload”, “pingback”, “preconnect”, “prefetch”, “preload”, “prerender”, or “stylesheet”.",
      "severity": "Warning",
      "span": {
        "byte_end": 376,
        "byte_start": 304,
        "col": 9,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 416,
        "byte_start": 385,
        "col": 9,
        "line": 11
      }
    }
  ],
  "source_name": "html/syntax/parsing/foreign_content_006.html"
}
```
