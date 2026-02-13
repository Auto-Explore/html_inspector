# html/semantics/scripting-1/the-script-element/module/crossorigin-import-same.sub.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/crossorigin-import-same.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
    <title>html-script-module-crossOrigin-root-WithCORS</title>
    <script src="crossorigin-common.js"></script>
</head>
<body>
    <h1>html-script-module-crossOrigin-root-WithCORS</h1>
    <script type="module" crossorigin>

        import { foo } from "http://{{domains[www2]}}:{{ports[http][0]}}/html/semantics/scripting-1/the-script-element/module/crossorigin-scripterror.js?pipe=header(Access-Control-Allow-Origin,*)";

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
  "source_name": "html/semantics/scripting-1/the-script-element/module/crossorigin-import-same.sub.html"
}
```
