# html/user-activation/resources/propagation-crossorigin-child.sub.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/user-activation/resources/propagation-crossorigin-child.sub.html",
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
        "type": "child-crossorigin-loaded",
        "isActive": navigator.userActivation.isActive,
        "hasBeenActive": navigator.userActivation.hasBeenActive
    }), "*");

    window.addEventListener("click", event => {
        window.top.postMessage(JSON.stringify({
            "type": "child-crossorigin-report",
            "isActive": navigator.userActivation.isActive,
            "hasBeenActive": navigator.userActivation.hasBeenActive
        }), "*");
    });
  </script>
</head>
<body style="background: lightgreen;">
  <!-- The midpoint of this frame should be outside the grandchild frame. -->
  <div style="height: 75px;">Cross-origin child frame</div>
  <iframe id="child2" width="270px" height="30px"
          src="http://{{hosts[][]}}:{{ports[http][1]}}/html/user-activation/resources/child-two.html">
  </iframe>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.iframe.src.invalid",
      "message": "Bad value “http://{{hosts[][]}}:{{ports[http][1]}}/html/user-activation/resources/child-two.html” for attribute “src” on element “iframe”.",
      "severity": "Warning",
      "span": {
        "byte_end": 909,
        "byte_start": 759,
        "col": 3,
        "line": 23
      }
    },
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
  "source_name": "html/user-activation/resources/propagation-crossorigin-child.sub.html"
}
```
