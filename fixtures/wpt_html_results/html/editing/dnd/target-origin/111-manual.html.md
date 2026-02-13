# html/editing/dnd/target-origin/111-manual.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/target-origin/111-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>Origin for file: to http:</title>
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
    <title>Origin for file: to http:<\/title>\
    <style type="text/css">\
div { height: 100px; width: 100px; background: orange; }\
iframe { width: 500px; height: 120px; border: none; }\
    <\/style>\
    <script type="text/javascript">\
window.onload = function () {\
  var origin = "null (string)";\
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
  document.getElementsByTagName('a')[0].href = 'data:text/html,'+escape(datastr);
};
    </script>
  </head>
  <body>

    <noscript><p>Enable JavaScript and reload</p></noscript>
    <p><a href="">Download the linked file to your disk</a>, and open it locally. Follow further instructions in that file.</p>

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
        "byte_end": 150,
        "byte_start": 81,
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
        "byte_end": 195,
        "byte_start": 164,
        "col": 5,
        "line": 6
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
  "source_name": "html/editing/dnd/target-origin/111-manual.html"
}
```
