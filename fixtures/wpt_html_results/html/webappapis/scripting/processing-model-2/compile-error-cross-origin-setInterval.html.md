# html/webappapis/scripting/processing-model-2/compile-error-cross-origin-setInterval.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/processing-model-2/compile-error-cross-origin-setInterval.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
 <head>
  <title>window.onerror - compile error in cross-origin setInterval</title>
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
 </head>
 <body>
  <div id=log></div>
  <script>
    setup({allow_uncaught_exception:true});
    var t = async_test();
    var ran = false;
    var interval;
    window.addEventListener('error', t.step_func(e => {
        clearInterval(interval);
        ran = true;
        assert_equals(e.error.constructor, SyntaxError);
    }));
    var script = document.createElement('script');
    script.src = location.href.replace('://', '://www1.').replace(/\/[^\/]+$/, '/support/syntax-error-in-setInterval.js');
    document.body.appendChild(script);
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
  "source_name": "html/webappapis/scripting/processing-model-2/compile-error-cross-origin-setInterval.html"
}
```
