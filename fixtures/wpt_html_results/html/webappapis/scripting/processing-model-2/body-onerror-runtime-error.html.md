# html/webappapis/scripting/processing-model-2/body-onerror-runtime-error.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/processing-model-2/body-onerror-runtime-error.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
 <head>
  <title>&lt;body onerror> - runtime error in &lt;script></title>
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
 </head>
  <script>
    setup({allow_uncaught_exception:true});
    var t = async_test();
    var t_col = async_test(document.title+' (column)');
    var ran = false;
  </script>
 <body onerror="
    t.step(function(){
        ran = true;
        assert_equals(typeof event, 'string', 'first arg');
        assert_equals(source, location.href, 'second arg');
        assert_equals(typeof lineno, 'number', 'third arg');
    });
    t_col.step(function(){
        assert_equals(typeof colno, 'number', 'fourth arg');
    });
    ">
  <div id=log></div>
  <script>
    undefined_variable;
  </script>
  <script>
    t.step(function(){
        assert_true(ran, 'ran');
        t.done();
    });
    t_col.step(function(){
        t_col.done();
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
  "source_name": "html/webappapis/scripting/processing-model-2/body-onerror-runtime-error.html"
}
```
