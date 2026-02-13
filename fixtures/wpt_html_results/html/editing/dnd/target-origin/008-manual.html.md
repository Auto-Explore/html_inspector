# html/editing/dnd/target-origin/008-manual.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/target-origin/008-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>Data URI does match *</title>
  </head>
  <body>
    <!--
* allows any URL at all, so it should work
    -->
    <p>Load the following URL in a new tab (copy &amp; paste it into the address bar):</p>
    <noscript><p>Enable JavaScript and reload</p></noscript>

    <script type="text/javascript">
document.write('data:text/html,'+escape(
'<!doctype html>\
<html>\
  <head>\
    <title>Data URI does match *<\/title>\
    <style type="text/css">\
html, body { margin: 0; padding: 0; }\
div { height: 100px; width: 100px; background: orange; }\
iframe { border: none; height: 150px; width: 150px; }\
    <\/style>\
  <\/head>\
  <body>\
    <script type="text/javascript">\
var seentypes = {};\
if( self == top ) {\
  document.body.ondragenter = document.body.ondragover = document.body.ondrop = function (e) {\
    e.preventDefault();\
    if( e.type == "drop" ) {\
      document.body.innerHTML = ( seentypes.dragenter && seentypes.dragover ) ? "PASS" : "FAIL";\
    } else {\
      seentypes[e.type] = true;\
    }\
  };\
  document.write("<p>Drag the orange square below over this text, and release it. Fail if this text does not change.<\\\/p>");\
  document.write("<p><iframe src=\\""+location.href+"\\"><\\\/iframe><\\\/p>");\
} else {\
  document.write("<div draggable=\\"true\\"><\\\/div>");\
  document.getElementsByTagName("div")[0].ondragstart = function (e) {\
    e.dataTransfer.effectAllowed = "copy";\
    e.dataTransfer.setData("text","dummy text");\
    e.dataTransfer.allowTargetOrigin("*");\
  };\
}\
    <\/script>\
  <\/body>\
<\/html>'));

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
        "byte_end": 340,
        "byte_start": 309,
        "col": 5,
        "line": 13
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
  "source_name": "html/editing/dnd/target-origin/008-manual.html"
}
```
