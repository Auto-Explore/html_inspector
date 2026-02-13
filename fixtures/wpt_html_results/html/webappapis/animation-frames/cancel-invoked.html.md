# html/webappapis/animation-frames/cancel-invoked.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/animation-frames/cancel-invoked.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>cancelAnimationFrame does nothing</title>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
    <link rel="help" href="https://w3c.github.io/web-performance/specs/RequestAnimationFrame/Overview.html#dom-windowanimationtiming-cancelanimationframe"/>
  </head>
  <body>
    <div id="log"></div>
    <script>
      test(function (t) {
        window.cancelAnimationFrame(42);
        assert_true(true);
      }, "cancelAnimationFrame does nothing if there is no callback with the given handle");
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
  "source_name": "html/webappapis/animation-frames/cancel-invoked.html"
}
```
