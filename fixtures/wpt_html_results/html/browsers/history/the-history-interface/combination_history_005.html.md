# html/browsers/history/the-history-interface/combination_history_005.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-history-interface/combination_history_005.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html>
  <head>
    <title>combination_history_005(After calling of forward method, check length.)</title>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>
  <body>
    <div id="log"></div>
    <script>
    var t = async_test("After calling of forward method, check length");

    var last;
    var fired = false;
    t.step(function () {
      window.history.pushState(1, document.title, '?x=1');
      window.history.pushState(2, document.title, '?x=2');
      last = window.history.length;

      window.history.back();
    });

    window.addEventListener('popstate', t.step_func(function(e) {
      if(fired) {
        assert_equals(e.state, 2, "state");
        assert_equals(window.history.length, last, "last");
        t.done();
      }
      fired = true;
      window.history.forward();
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
  "source_name": "html/browsers/history/the-history-interface/combination_history_005.html"
}
```
