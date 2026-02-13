# html/browsers/history/the-history-interface/combination_history_006.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-history-interface/combination_history_006.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html>
  <head>
    <title>combination_history_006(After calling of go method, check length.)</title>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>
  <body>
    <div id="log"></div>
    <script>
    var t = async_test("After calling of go method, check length");

    var last;
    t.step(function () {
      window.history.pushState(1, document.title, '?x=1');
      window.history.pushState(2, document.title, '?x=2');

      last = window.history.length;

      window.history.go(-1);
    });

    window.addEventListener('popstate', t.step_func(function(e) {
      assert_equals(e.state, 1, "state");
      assert_equals(window.history.length, last, "last");
      t.done();
    }), false);
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
  "source_name": "html/browsers/history/the-history-interface/combination_history_006.html"
}
```
