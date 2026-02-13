# html/webappapis/scripting/processing-model-2/runtime-error-cross-origin.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/processing-model-2/runtime-error-cross-origin.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
 <head>
  <title>window.onerror - runtime error in &lt;script src=//www1...></title>
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
        assert_equals(a, 'Script error.', 'first arg');
        assert_equals(b, '', 'second arg');
        assert_equals(c, 0, 'third arg');
    });
    var script = document.createElement('script');
    script.src = location.href.replace('://', '://www1.').replace(/\/[^\/]+$/, '/support/undefined-variable.js');
    document.body.appendChild(script);
    onload = function(){
        t.step(function(){
            assert_true(ran, 'ran');
            t.done();
        });
        t_col.step(function(){
            assert_equals(col_value, 0, 'fourth arg');
            t_col.done();
        });
    };
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
  "source_name": "html/webappapis/scripting/processing-model-2/runtime-error-cross-origin.html"
}
```
