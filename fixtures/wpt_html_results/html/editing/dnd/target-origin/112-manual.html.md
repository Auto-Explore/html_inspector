# html/editing/dnd/target-origin/112-manual.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/target-origin/112-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>Origin for data: with inherited http origin to http:</title>
    <style type="text/css">
html, body, iframe { display: block; width: 100%; height: 100%; border: none; margin: 0; padding: 0; }
    </style>
    <script type="text/javascript" src="../resources/crossorigin.sub.js"></script>
    <script type="text/javascript">
window.onload = function () {
  var origin = 'http://'+httpHostMain;
  if( location.href.indexOf(origin+'/') ) {
    document.body.innerHTML = 'This must be tested on '+origin+'/';
    return;
  }
  var datastr =
'<!doctype html>\
<html>\
  <head>\
    <title>Origin for data: with inherited http origin to http:<\/title>\
    <style type="text/css">\
div { height: 100px; width: 100px; background: orange; }\
iframe { width: 500px; height: 120px; border: none; }\
    <\/style>\
    <script type="text/javascript">\
window.onload = function () {\
  var origin = "http://'+httpHostMain+'";\
  document.getElementsByTagName("div")[0].ondragstart = function (e) {\
    e.dataTransfer.effectAllowed = "copy";\
    e.dataTransfer.setData("text","dummy text");\
  };\
  document.getElementsByTagName("span")[0].textContent = origin;\
  var iframe = document.createElement("iframe");\
  iframe.src = "'+location.href.replace(/[^\/]*$/,'HELPER-showorigin.html')+'";\
  document.body.insertBefore(iframe,document.getElementsByTagName("div")[0]);\
};\
    <\/script>\
  <\/head>\
  <body>\
    <p>Drag the orange square onto the blue square and release it. The blue square should be replaced with the text:<br>\
    <span><\/span><\/p>\
    <div draggable="true"></div>\
  <\/body>\
<\/html>';
  var iframe = document.createElement('iframe');
  iframe.src = 'data:text/html,'+escape(datastr);
  document.body.appendChild(iframe);
};
    </script>
  </head>
  <body>

    <noscript><p>Enable JavaScript and reload</p></noscript>

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
        "byte_end": 131,
        "byte_start": 108,
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
        "byte_end": 321,
        "byte_start": 252,
        "col": 5,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 366,
        "byte_start": 335,
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
  "source_name": "html/editing/dnd/target-origin/112-manual.html"
}
```
