# html/webappapis/scripting/processing-model-2/runtime-error-in-window-onerror.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/processing-model-2/runtime-error-in-window-onerror.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
 <head>
  <title>runtime error in window.onerror</title>
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
 </head>
 <body>
 <div id=log></div>
  <script>
    setup({allow_uncaught_exception:true});
    var t = async_test();
    var ran = 0;
    window.onerror = function(){
        ran++;
        undefined_variable_in_onerror;
    };
  </script>
  <script>
    undefined_variable;
  </script>
  <script>
    t.step(function(){
        assert_equals(ran, 1, 'ran');
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
  "source_name": "html/webappapis/scripting/processing-model-2/runtime-error-in-window-onerror.html"
}
```
