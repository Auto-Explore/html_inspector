# html/browsers/offline/manifest_section_empty-manual.https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/offline/manifest_section_empty-manual.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html manifest="resources/manifest/section_empty.manifest">
  <head>
    <title>Offline Application Cache - manifest_section_empty</title>
    <script src="resources/js/clock.js"></script>
    <link rel="stylesheet" href="resources/css/result.css">
    <link rel="stylesheet" href="resources/css/clock.css">
    <link rel="stylesheet" href="resources/css/online.css" type="text/css" media="screen">
  </head>
  <body>
    <ol>
      <li>Disable the network connection.</li>
      <li>Refresh the page.</li>
      <li>If the time element and colors of result elements are normally displayed, then test is <span class="manualpass"><b>PASS</b></span>, otherwise <span class="manualfail"><b>FAIL</b></span>.</li>
    </ol>

    <p class="connectivity" width="600">The time is: <output id="clock"></output></p>
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
  "source_name": "html/browsers/offline/manifest_section_empty-manual.https.html"
}
```
