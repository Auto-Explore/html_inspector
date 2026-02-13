# html/webappapis/scripting/processing-model-2/compile-error-same-origin.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/processing-model-2/compile-error-same-origin.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
 <head>
  <title>window.onerror - compile error in &lt;script src=...></title>
  <script src="/resources/testharness.js"></script>
  <script src="/resources/testharnessreport.js"></script>
 </head>
 <body>
  <div id=log></div>
  <script>
    setup({allow_uncaught_exception:true});
    var t = async_test();
    var t_col = async_test(document.title+' (column)');
    var ran = false;
    var col_value;
    window.onerror = t.step_func(function(a, b, c, d){
        ran = true;
        col_value = d;
        assert_equals(typeof a, 'string', 'first arg');
        assert_equals(b, document.querySelector('script[src="support/syntax-error.js"]').src, 'second arg');
        assert_equals(typeof c, 'number', 'third arg');
    });
  </script>
  <script src="support/syntax-error.js"></script>
  <script>
    t.step(function(){
        assert_true(ran, 'ran');
        t.done();
    });
    t_col.step(function(){
        assert_equals(typeof col_value, 'number', 'fourth arg');
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
  "source_name": "html/webappapis/scripting/processing-model-2/compile-error-same-origin.html"
}
```
