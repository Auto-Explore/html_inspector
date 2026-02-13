# html/semantics/scripting-1/the-script-element/execution-timing/pages/helloworld-postMessage.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/execution-timing/pages/helloworld-postMessage.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html><head>
  <title> TC component </title>
</head>
<body>

  <p>This page should appear in popup or frame</p>

  <script type="text/javascript">
  var target = opener || top;
  var id = location.search?' '+location.search.substring(1) : '';
  target.log('frame/popup script'+id);
  window.onload=function(){
    target.log('load event inside frame/popup script'+id);
    target.postMessage('msg evt frame/popup script'+id, '*');
  }
  </script>

</body></html>
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
        "byte_end": 162,
        "byte_start": 131,
        "col": 3,
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
  "source_name": "html/semantics/scripting-1/the-script-element/execution-timing/pages/helloworld-postMessage.html"
}
```
