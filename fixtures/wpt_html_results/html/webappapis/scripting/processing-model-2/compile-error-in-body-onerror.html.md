# html/webappapis/scripting/processing-model-2/compile-error-in-body-onerror.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/processing-model-2/compile-error-in-body-onerror.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
 <head>
  <title>window.onerror - compile error in &lt;body onerror></title>
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
  <script>
    setup({allow_uncaught_exception:true});
    var t = async_test();
    var ran = false;
    window.onerror = t.step_func(function(){
        ran = true;
    });
  </script>
 </head>
 <body onerror="{"><!-- sets the event handler to null before compiling -->
  <div id=log></div>
  <script>
   for(;) {}
  </script>
  <script>
    t.step(function(){
        assert_false(ran, 'ran');
        t.done();
    });
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
  "source_name": "html/webappapis/scripting/processing-model-2/compile-error-in-body-onerror.html"
}
```
