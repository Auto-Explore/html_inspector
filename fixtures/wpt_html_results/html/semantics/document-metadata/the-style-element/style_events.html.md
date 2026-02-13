# html/semantics/document-metadata/the-style-element/style_events.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-style-element/style_events.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8">
    <title>HTML Test: The style events</title>
    <link rel="author" title="Intel" href="http://www.intel.com/">
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#the-style-element">
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
    <script>
      var tLoad = async_test("If the style is loaded successfully, the 'load' event must be fired");
      var tError = async_test("If the style is loaded unsuccessfully, the 'error' event must be fired");

      function onstyleload(e) {
        tLoad.done();
      }

      function onstyleerror(e) {
        tError.done();
      }
    </script>
    <style onload="onstyleload()">
      #test {
        height: 100px;
        width: 100px;
      }
    </style>
    <style onerror="onstyleerror()">
      @import url(nonexistent.css);
    </style>
  </head>
  <body>
    <div id="log"></div>
    <div id="test"></div>
  </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/document-metadata/the-style-element/style_events.html"
}
```
