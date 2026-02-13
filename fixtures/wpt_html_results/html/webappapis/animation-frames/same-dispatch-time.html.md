# html/webappapis/animation-frames/same-dispatch-time.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/animation-frames/same-dispatch-time.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>requestAnimationFrame in queue get the same timestamp</title>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
    <link rel="help" href="http://w3c.github.io/animation-timing/#dfn-invoke-callbacks-algorithm"/>
  </head>
  <body>
    <div id="log"></div>
    <script>
      async_test(function (t) {
        var a = 0, b = 0;

        /* REASONING:
        * These two methods that will be called with a timestamp. Because
        * they execute right after eachother, they're added to the same
        * queue and SHOULD be timestamped with the same value.
        */
        requestAnimationFrame(t.step_func(function() { a = arguments[0]; }));
        requestAnimationFrame(t.step_func(function() {
          b = arguments[0];
          assert_not_equals(a, 0);
          assert_not_equals(b, 0);
          assert_equals(a, b);
          t.done();
        }));
      }, "requestAnimationFrame will timestamp events in the same queue with the same time");
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
  "source_name": "html/webappapis/animation-frames/same-dispatch-time.html"
}
```
