# html/user-activation/resources/consumption-sameorigin-child.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/user-activation/resources/consumption-sameorigin-child.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
  <script>
    window.top.postMessage(JSON.stringify({
        "type": "child-sameorigin-loaded",
        "isActive": navigator.userActivation.isActive,
        "hasBeenActive": navigator.userActivation.hasBeenActive
    }), "*");

    window.addEventListener("click", event => {
        window.open().close();

        window.top.postMessage(JSON.stringify({
            "type": "child-sameorigin-report",
            "isActive": navigator.userActivation.isActive,
            "hasBeenActive": navigator.userActivation.hasBeenActive
        }), "*");
    });
  </script>
</head>
<body style="background: lightgreen;">
  <!-- The midpoint of this frame should be outside the grandchild frame. -->
  <div style="height: 75px;">Same-origin child frame</div>
  <iframe id="child2" width="270px" height="30px"
          src="child-two.html">
  </iframe>
</body>
</html>
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
  "source_name": "html/user-activation/resources/consumption-sameorigin-child.html"
}
```
