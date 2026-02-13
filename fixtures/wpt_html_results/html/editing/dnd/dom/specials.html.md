# html/editing/dnd/dom/specials.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/dom/specials.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>Exposing drag &amp; drop events on document and window</title>
    <script type="text/javascript" src="/resources/testharness.js"></script>
    <script type="text/javascript" src="/resources/testharnessreport.js"></script>
  </head>
  <body>

    <div id="log">Enable script and reload</div>
    <script type="text/javascript">
var allEvents = ['ondragstart','ondrag','ondragover','ondragenter','ondragleave','ondrop','ondragend'];
var allObjects = [['window',window],['document',document],['HTMLElement',document.createElement('div')]];
var fails = [];
for( var i = 0; i < allObjects.length; i++ ) {
  for( var j = 0; j < allEvents.length; j++ ) {
    test(function () {
      assert_true(allEvents[j] in allObjects[i][1]);
    }, allEvents[j] + ' in ' + allObjects[i][0]);
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
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 173,
        "byte_start": 110,
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
        "byte_end": 256,
        "byte_start": 187,
        "col": 5,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 370,
        "byte_start": 339,
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
  "source_name": "html/editing/dnd/dom/specials.html"
}
```
