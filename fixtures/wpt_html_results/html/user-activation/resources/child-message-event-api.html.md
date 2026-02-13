# html/user-activation/resources/child-message-event-api.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/user-activation/resources/child-message-event-api.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<body style="background: red;">
  <script>
    window.parent.postMessage("child-loaded",
                              {targetOrigin: "*", includeUserActivation: true});
    window.addEventListener("click", event => {
      window.parent.postMessage("child-clicked",
                                {targetOrigin: "*", includeUserActivation: true});
      var win = window.open('404.html');
      win.close();
    });

    window.addEventListener("message", event => {
      if (event.data == "report") {
        window.parent.postMessage("child-report",
                                  {targetOrigin: "*", includeUserActivation: true});
      }
      if (event.data == "report-no-activation") {
        window.parent.postMessage("child-report-no-activation",
                                  {targetOrigin: "*", includeUserActivation: false});
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
  "source_name": "html/user-activation/resources/child-message-event-api.html"
}
```
