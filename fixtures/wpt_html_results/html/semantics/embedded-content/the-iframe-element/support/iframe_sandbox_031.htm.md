# html/semantics/embedded-content/the-iframe-element/support/iframe_sandbox_031.htm

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/support/iframe_sandbox_031.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head><title>Page with access to localStorage and sessionStorage</title>
</head>
<body>
    <script type="text/javascript">
        try
        {
            if (window.localStorage && window.sessionStorage) {
                parent.window.postMessage("access to window.localStorage and window.sessionStorage", "*");
            }
        }
        catch(e)
        {
            parent.window.postMessage("no access to window.localStorage and window.sessionStorage", "*");
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
        "byte_end": 146,
        "byte_start": 115,
        "col": 5,
        "line": 6
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/support/iframe_sandbox_031.htm"
}
```
