# html/browsers/sandboxing/sandbox-allow-same-origin.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/sandboxing/sandbox-allow-same-origin.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
    <title>DOM access in sandbox="allow-same-origin" iframe</title>
    <link rel="author" title="Kinuko Yasuda" href="mailto:kinuko@chromium.org">
    <link rel="help" href="http://www.w3.org/html/wg/drafts/html/master/browsers.html#sandboxing">
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>

  <body>
    <h1>DOM access in sandbox="allow-same-origin" iframe</h1>
    <script type="text/javascript">
      var t = async_test("DOM access in sandbox='allow-same-origin' iframe is allowed")
      var called = 0;
      function calledFromIframe() {
        called++;
      }
      function loaded() {
        assert_equals(document.getElementById('sandboxedframe').contentWindow.document.getElementById('inner').innerHTML, 'foo');
        assert_equals(called, 0);
        t.done();
      }
    </script>

    <iframe src="/html/browsers/sandboxing/inner-iframe.html" style="visibility:hidden;display:none" sandbox="allow-same-origin" id="sandboxedframe" onload="loaded();"></iframe>

    <div id="log"></div>
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
        "byte_end": 510,
        "byte_start": 479,
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
  "source_name": "html/browsers/sandboxing/sandbox-allow-same-origin.html"
}
```
