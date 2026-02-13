# html/editing/dnd/target-origin/117-manual.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/target-origin/117-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>Origin for site with document.domain set to a parent domain</title>
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
  var iframe = document.createElement('iframe');
  iframe.src = 'http://'+httpHostAlias+location.pathname.replace(/.html$/,'-1.html');
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
        "byte_end": 138,
        "byte_start": 115,
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
        "byte_end": 328,
        "byte_start": 259,
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
        "byte_end": 373,
        "byte_start": 342,
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
  "source_name": "html/editing/dnd/target-origin/117-manual.html"
}
```
