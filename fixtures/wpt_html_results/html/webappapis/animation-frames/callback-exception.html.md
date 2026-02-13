# html/webappapis/animation-frames/callback-exception.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/animation-frames/callback-exception.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>requestAnimationFrame callback exception reported to error handler</title>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
    <link rel="help" href="https://w3c.github.io/web-performance/specs/RequestAnimationFrame/Overview.html#dom-windowanimationtiming-requestanimationframe"/>
  </head>
  <body>
    <div id="log"></div>
    <script>
      var custom_exception = 'requestAnimationFrameException';
      setup({allow_uncaught_exception : true});
      async_test(function (t) {
        addEventListener("error",function(e) {
          t.step(function() {
             assert_equals(e.error.message, custom_exception);
             t.done();
          })
        });
        window.requestAnimationFrame(function () {
          throw new Error(custom_exception);
        });
      }, "requestAnimationFrame callback exceptions are reported to error handler");
    </script>
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
  "source_name": "html/webappapis/animation-frames/callback-exception.html"
}
```
