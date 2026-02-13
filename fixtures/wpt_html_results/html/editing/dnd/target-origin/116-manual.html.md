# html/editing/dnd/target-origin/116-manual.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/target-origin/116-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>Origin for dropped files</title>
    <style type="text/css">
div { height: 100px; width: 100px; background: orange; }
iframe { width: 500px; height: 120px; border: none; }
    </style>
    <script type="text/javascript" src="../resources/crossorigin.sub.js"></script>
    <script type="text/javascript">
window.onload = function () {
  var origin = 'http://'+httpHostMain;
  if( location.href.indexOf(origin+'/') ) {
    document.body.innerHTML = 'This must be tested on '+origin+'/';
    return;
  }
  document.getElementsByTagName('span')[0].textContent = 'null (string)';
  var iframe = document.createElement('iframe');
  iframe.src = origin+location.pathname.replace(/[^\/]*$/,'HELPER-showorigin.html');
  document.body.insertBefore(iframe,document.getElementsByTagName('div')[0]);
};
    </script>
  </head>
  <body>

    <noscript><p>Enable JavaScript and reload</p></noscript>
    <p>Drag a small file from your computer onto the blue square and release it. If a prompt appears, accept it. The blue square should be replaced with the text:<br>
    <span></span></p>

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
        "byte_end": 103,
        "byte_start": 80,
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
        "byte_end": 301,
        "byte_start": 232,
        "col": 5,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 346,
        "byte_start": 315,
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
  "source_name": "html/editing/dnd/target-origin/116-manual.html"
}
```
