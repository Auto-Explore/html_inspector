# html/browsers/history/the-history-interface/blank2.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-history-interface/blank2.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>Dummy page 2</title>
  </head>
  <body>
    <script type="text/javascript">
if( self == top || !parent.reportload ) {
  document.write("<p>FAIL. Browser got confused when navigating forwards, and navigated the whole window to the iframe's location, instead of just navigating the iframe. It is not possible to run the testsuite.<\/p>");
}
    </script>
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
        "byte_end": 118,
        "byte_start": 87,
        "col": 5,
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
  "source_name": "html/browsers/history/the-history-interface/blank2.html"
}
```
