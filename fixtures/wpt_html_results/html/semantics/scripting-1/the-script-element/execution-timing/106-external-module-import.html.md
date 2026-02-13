# html/semantics/scripting-1/the-script-element/execution-timing/106-external-module-import.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/execution-timing/106-external-module-import.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html><head>
        <title> scheduler: stylesheets blocking external module scripts</title>
        <script src="/resources/testharness.js"></script>
        <script src="/resources/testharnessreport.js"></script>
        <!-- this stylesheet blocks scripts -->
        <link rel="stylesheet" href="css/import.css?pipe=trickle(d2)">
</head>
<body>
        <div id="log">FAILED (This TC requires JavaScript enabled)</div>
        <div id="test">Test</div>

        <script src="scripts/check-style-sheet.js" type="module"></script>
</body></html>
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
  "source_name": "html/semantics/scripting-1/the-script-element/execution-timing/106-external-module-import.html"
}
```
