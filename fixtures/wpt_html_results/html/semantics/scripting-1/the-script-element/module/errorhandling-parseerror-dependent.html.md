# html/semantics/scripting-1/the-script-element/module/errorhandling-parseerror-dependent.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/errorhandling-parseerror-dependent.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
<head>
    <title>html-script-module-errorHandling-parseError-Dependent</title>
    <script src="errorhandling-parseerror-common.js"></script>
</head>
<body>
    <script type="module" onerror="errorHandler(event);">

        // No parse errors in the root module, just in the dependent module
        import test from "./errorhandling-parseerror-dependent.js";
        document._errorReported = "shouldn't have run dependent module";

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
  "source_name": "html/semantics/scripting-1/the-script-element/module/errorhandling-parseerror-dependent.html"
}
```
