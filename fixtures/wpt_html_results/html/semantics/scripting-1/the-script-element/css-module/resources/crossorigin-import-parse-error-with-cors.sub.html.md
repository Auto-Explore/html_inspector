# html/semantics/scripting-1/the-script-element/css-module/resources/crossorigin-import-parse-error-with-cors.sub.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/css-module/resources/crossorigin-import-parse-error-with-cors.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
    <title>css-module-import-cross-domain-parse-error-WithCORS</title>
    <script src="../../module/crossorigin-common.js"></script>
</head>
<body>
    <h1>css-module-import-cross-domain-parse-error-WithCORS</h1>
    <script type="module" crossorigin>
        import styleSheet from "http://{{domains[www2]}}:{{ports[http][0]}}/html/semantics/scripting-1/the-script-element/css-module/resources/parse-error.css?pipe=header(Access-Control-Allow-Origin,*)" with { type: "css" };
        // Push an event to the log indicating that the script was executed.
        document._log.push(`imported CSS rules count: ${styleSheet.rules.length}`);
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
  "source_name": "html/semantics/scripting-1/the-script-element/css-module/resources/crossorigin-import-parse-error-with-cors.sub.html"
}
```
