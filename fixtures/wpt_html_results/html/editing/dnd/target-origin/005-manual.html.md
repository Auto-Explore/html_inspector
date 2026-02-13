# html/editing/dnd/target-origin/005-manual.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/target-origin/005-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>JavaScript URI does not match absolute HTTP URL</title>
  </head>
  <body>
    <p>Load the following URL in a new tab (copy &amp; paste it into the address bar):</p>
    <noscript><p>Enable JavaScript and reload</p></noscript>

    <script type="text/javascript">
document.write('javascript:unescape("'+escape(
'<!doctype html>\
<html>\
  <head>\
    <title>JavaScript does not match absolute HTTP URL<\/title>\
    <style type=\'text/css\'>\
iframe { border: none; height: 150px; width: 150px; }\
    <\/style>\
    <script type=\'text/javascript\'>\
window.onload = function () {\
  document.body.ondragenter = document.body.ondragleave = document.body.ondragover = document.body.ondrop = function (e) {\
    e.preventDefault();\
    document.body.innerHTML = \'FAIL\';\
  };\
};\
    <\/script>\
  <\/head>\
  <body>\
    <p>Drag the orange square below over this text, and release it. Pass if this text does not change.<\/p>\
    <p><iframe src=\''+location.href.replace(/\.html$/,'-1.html')+'\'><\/iframe><\/p>\
  <\/body>\
<\/html>')+'")');

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
        "byte_end": 306,
        "byte_start": 275,
        "col": 5,
        "line": 10
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
  "source_name": "html/editing/dnd/target-origin/005-manual.html"
}
```
