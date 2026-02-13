# html/user-activation/resources/child-two.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/user-activation/resources/child-two.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<body style="background: lightgrey;">
  <script>
    window.top.postMessage(JSON.stringify({
        "type": "child-two-loaded",
        "isActive": navigator.userActivation.isActive,
        "hasBeenActive": navigator.userActivation.hasBeenActive
    }), "*");

    window.addEventListener("click", event => {
        window.top.postMessage(JSON.stringify({
            "type": "child-two-clicked",
            "isActive": navigator.userActivation.isActive,
            "hasBeenActive": navigator.userActivation.hasBeenActive
        }), "*");
    });

    window.addEventListener("message", event => {
        var msg = JSON.parse(event.data);
        if (msg.type == "report") {
            window.top.postMessage(JSON.stringify({
                "type": "child-two-report",
                "isActive": navigator.userActivation.isActive,
                "hasBeenActive": navigator.userActivation.hasBeenActive
            }), "*");
        }
    });
  </script>
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/user-activation/resources/child-two.html"
}
```
