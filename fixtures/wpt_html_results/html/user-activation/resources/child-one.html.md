# html/user-activation/resources/child-one.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/user-activation/resources/child-one.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<body style="background: yellow;">
  <script>
    window.top.postMessage(JSON.stringify({
        "type": "child-one-loaded",
        "isActive": navigator.userActivation.isActive,
        "hasBeenActive": navigator.userActivation.hasBeenActive
    }), "*");

    window.addEventListener("click", event => {
        window.top.postMessage(JSON.stringify({
            "type": "child-one-clicked",
            "isActive": navigator.userActivation.isActive,
            "hasBeenActive": navigator.userActivation.hasBeenActive
        }), "*");
    });

    window.addEventListener("message", event => {
        var msg = JSON.parse(event.data);
        if (msg.type == "report") {
            window.top.postMessage(JSON.stringify({
                "type": "child-one-report",
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
  "source_name": "html/user-activation/resources/child-one.html"
}
```
