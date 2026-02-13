# html/semantics/embedded-content/the-iframe-element/support/iframe_sandbox_029.htm

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/support/iframe_sandbox_029.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head><title>Page with access to document.cookie</title>
</head>
<body>
    <div>Cookie Read: <span id="readCookie"></span></div>
    <script type="text/javascript">
        try
        {
            cookie = document.cookie;
            document.cookie = "name=browser";
            parent.window.postMessage("cookies are R/W", "*");
        }catch(e)
        {
            parent.window.postMessage("cookies are not R/W", "*");
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
        "byte_end": 188,
        "byte_start": 157,
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/support/iframe_sandbox_029.htm"
}
```
