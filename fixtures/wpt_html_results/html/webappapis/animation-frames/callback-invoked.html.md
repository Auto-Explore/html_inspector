# html/webappapis/animation-frames/callback-invoked.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/animation-frames/callback-invoked.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>requestAnimationFrame must be triggered once</title>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
    <link rel="help" href="https://w3c.github.io/web-performance/specs/RequestAnimationFrame/Overview.html#dom-windowanimationtiming-requestanimationframe"/>
  </head>
  <body>
    <div id="log"></div>
    <script>
      async_test(function (t) {
        assert_false(document.hidden, "document.hidden must exist and be false to run this test properly");
        window.requestAnimationFrame(t.step_func_done());
      }, "requestAnimationFrame callback is invoked at least once before the timeout");
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
  "source_name": "html/webappapis/animation-frames/callback-invoked.html"
}
```
