# html/semantics/scripting-1/the-script-element/module/errorhandling-parseerror-root.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/errorhandling-parseerror-root.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
<head>
    <title>html-script-module-errorHandling-parseError-Root</title>
    <script src="errorhandling-parseerror-common.js"></script>
</head>
<body>
    <script type="module">

        // Parse error in a root module
        1A

    </script>
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/errorhandling-parseerror-root.html"
}
```
