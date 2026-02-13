# html/syntax/parsing/foreign_content_013.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing/foreign_content_013.html",
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
        <meta description="Test to verify SVG elements are styled using CLASS Selector" />
        <link rel="author" title="Microsoft" href="http://www.microsoft.com/" />

        <style>
            .svg1
            {
                fill: green ;
            }
        </style>
    </head>

    <body>
        <div class="testdata">
            <p id="instructions">Test passes if a green square is visible below this line. </p>
        </div>
        <div id="d1">
            <svg class="svg1" width="100px" height="100px">
                <rect  width="100px" height="100px"/>
            </svg>
        </div>
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
  "source_name": "html/syntax/parsing/foreign_content_013.html"
}
```
