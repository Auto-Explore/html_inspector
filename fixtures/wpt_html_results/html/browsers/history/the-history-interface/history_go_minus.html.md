# html/browsers/history/the-history-interface/history_go_minus.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-history-interface/history_go_minus.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html>
  <head>
    <title>history_go_minus</title>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>
  <body>
    <div id="log"></div>
    <script>
    var t = async_test("history go minus");

    t.step(function () {
      window.history.pushState(1, document.title, '?x=1');
      window.history.pushState(2, document.title, '?x=2');

      window.history.go(-1);
    });

    window.addEventListener('popstate', t.step_func(function(e) {
      assert_equals(e.state, 1, "history state");

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
  "source_name": "html/browsers/history/the-history-interface/history_go_minus.html"
}
```
