# html/editing/dnd/target-origin/HELPER-showorigin.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/target-origin/HELPER-showorigin.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>Readout of .origin</title>
    <style type="text/css">
html, body { margin: 0; padding: 0; }
div { height: 100px; width: 100px; background: blue; }
    </style>
    <script type="text/javascript">
window.onload = function () {
  var blue = document.getElementsByTagName('div')[0];
  blue.ondragenter = blue.ondragover = function (e) {
    e.preventDefault();
  };
  blue.ondrop = function (e) {
    e.preventDefault();
    if( e.dataTransfer.origin === 'null' ) {
      document.body.innerHTML = 'null (string)';
    } else {
      document.body.innerHTML = e.dataTransfer.origin;
    }
  };
};
    </script>
  </head>
  <body>

    <div></div>

  </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.type.unnecessary",
      "message": "The “type” attribute for the “style” element is not needed and should be omitted.",
      "severity": "Warning",
      "span": {
        "byte_end": 97,
        "byte_start": 74,
        "col": 5,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 239,
        "byte_start": 208,
        "col": 5,
        "line": 9
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
  "source_name": "html/editing/dnd/target-origin/HELPER-showorigin.html"
}
```
