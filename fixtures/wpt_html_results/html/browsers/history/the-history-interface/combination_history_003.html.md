# html/browsers/history/the-history-interface/combination_history_003.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-history-interface/combination_history_003.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html>
  <head>
    <title>combination_history_003(After calling of pushState and replaceState methods, check length.)</title>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>
  <body>
    <div id="log"></div>
    <script>
    test(function () {
      var first;
      var second;
      var third;
      first = window.history.length;
      window.history.pushState(1, document.title, '?x=1');
      second = window.history.length;
      window.history.replaceState(2, document.title, '?x=2');
      third = window.history.length;

      assert_equals(second - first, 1, "second - first");
      assert_equals(third, second, "third");
    }, "After calling of pushState and replaceState methods, check length");
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
  "source_name": "html/browsers/history/the-history-interface/combination_history_003.html"
}
```
