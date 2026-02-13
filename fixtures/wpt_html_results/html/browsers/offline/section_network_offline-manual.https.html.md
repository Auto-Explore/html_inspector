# html/browsers/offline/section_network_offline-manual.https.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/offline/section_network_offline-manual.https.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html manifest="resources/manifest/clock.manifest">
  <head>
    <title>Offline Application Cache - Section_network_offline</title>
    <link rel="stylesheet" href="resources/css/result.css">
  </head>
  <body>
    <ol>
      <li>Disable the network connection.</li>
      <li>Refresh the page.</li>
      <li>If only the frame element can't be loaded, then test is <span class="manualpass"><b>PASS</b></span>, otherwise <span class="manualfail"><b>FAIL</b></span>.</li>
    </ol>

    <IFRAME id="TestFrame" name="TestWindow" src="html/clock.html" width="600" height="50" scrolling="auto" frameborder="1">
    </IFRAME>
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
  "source_name": "html/browsers/offline/section_network_offline-manual.https.html"
}
```
