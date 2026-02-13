# html/semantics/embedded-content/the-iframe-element/support/iframe_sandbox_027.htm

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/support/iframe_sandbox_027.htm",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head><title>XMLHttpRequest</title>
</head>
<body>
    <script type="text/javascript">
        xhrRequest = new XMLHttpRequest();

        xhrRequest.onreadystatechange = function () {
            if (xhrRequest.readyState == 4 && xhrRequest.status == 200) {
            //xhr successful
            parent.window.postMessage("access to window.XMLHttpRequest", "*");
            }
        }

        xhrRequest.open("GET", "standalone-pass.htm", true);
        xhrRequest.send();

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
        "byte_end": 109,
        "byte_start": 78,
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/support/iframe_sandbox_027.htm"
}
```
