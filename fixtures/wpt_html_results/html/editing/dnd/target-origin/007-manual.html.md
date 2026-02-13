# html/editing/dnd/target-origin/007-manual.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/target-origin/007-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>Data URI does not match its own URL</title>
  </head>
  <body>
    <!--
Sets an absolute URL pointing to the data URI.
The script origin will in fact be inherited from the parent page, which is actually the same data URI.
That part works.
However, when it comes to matching against it, it will not match, as the global identifier used as the
script origin does not match because the origin does not match the scheme/host/port tuple required.
    -->
    <p>Load the following URL in a new tab (copy &amp; paste it into the address bar):</p>
    <noscript><p>Enable JavaScript and reload</p></noscript>

    <script type="text/javascript">
document.write('data:text/html,'+escape(
'<!doctype html>\
<html>\
  <head>\
    <title>Data URI does not match its own URL<\/title>\
    <style type="text/css">\
html, body { margin: 0; padding: 0; }\
div { height: 100px; width: 100px; background: orange; }\
iframe { border: none; height: 150px; width: 150px; }\
    <\/style>\
  <\/head>\
  <body>\
    <script type="text/javascript">\
if( self == top ) {\
  document.body.ondragenter = document.body.ondragleave = document.body.ondragover = document.body.ondrop = function (e) {\
    e.preventDefault();\
    document.body.innerHTML = "FAIL";\
  };\
  document.write("<p>Drag the orange square below over this text, and release it. Pass if this text does not change.<\\\/p>");\
  document.write("<p><iframe src=\\""+location.href+"\\"><\\\/iframe><\\\/p>");\
} else {\
  document.write("<div draggable=\\"true\\"><\\\/div>");\
  document.getElementsByTagName("div")[0].ondragstart = function (e) {\
    e.dataTransfer.effectAllowed = "copy";\
    e.dataTransfer.setData("text","dummy text");\
    e.dataTransfer.allowTargetOrigin(location.href);\
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
        "byte_end": 681,
        "byte_start": 650,
        "col": 5,
        "line": 17
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
  "source_name": "html/editing/dnd/target-origin/007-manual.html"
}
```
