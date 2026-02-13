# html/editing/dnd/target-origin/HELPER-mustallow.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/target-origin/HELPER-mustallow.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>Must be allowed</title>
    <style type="text/css">
html { background: blue; }
html, body { margin: 0; padding: 0; height: 100%; width: 100%; }
    </style>
  </head>
  <body>
    <script type="text/javascript">
if( location.search && location.search.indexOf('domain') != -1 ) {
  document.domain = location.hostname.replace(/^[^.]+\./,'');
}
var seentypes = {};
document.body.ondragenter = document.body.ondragover = document.body.ondrop = function (e) {
  e.preventDefault();
  if( e.type == 'drop' ) {
    document.body.innerHTML = ( seentypes.dragenter && seentypes.dragover && e.dataTransfer.getData('text') == 'dummy text' ) ? 'PASS' : 'FAIL';
  } else {
    seentypes[e.type] = true;
  }
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
      "code": "html.style.type.unnecessary",
      "message": "The “type” attribute for the “style” element is not needed and should be omitted.",
      "severity": "Warning",
      "span": {
        "byte_end": 94,
        "byte_start": 71,
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
        "byte_end": 254,
        "byte_start": 223,
        "col": 5,
        "line": 11
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
  "source_name": "html/editing/dnd/target-origin/HELPER-mustallow.html"
}
```
