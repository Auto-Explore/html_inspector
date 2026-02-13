# html/browsers/offline/resources/html/clock.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/offline/resources/html/clock.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!-- clock.html -->
<!DOCTYPE HTML>
<html>
 <head>
  <title>Clock</title>
  <script src="../js/clock.js"></script>
  <link rel="stylesheet" href="../css/clock.css">
 </head>
 <body>
  <p>The time is: <output id="clock"></output></p>
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
  "source_name": "html/browsers/offline/resources/html/clock.html"
}
```
