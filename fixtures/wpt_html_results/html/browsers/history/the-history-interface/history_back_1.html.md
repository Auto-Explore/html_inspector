# html/browsers/history/the-history-interface/history_back_1.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-history-interface/history_back_1.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>history.back() with session history</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
  var t = async_test();
  started = false;
  pages = []
  timer = setInterval(
    function() {
      if (pages.length < 2)
        return;
      clearInterval(timer);
      setTimeout(t.step_func(
        function() {
          assert_array_equals(pages, [2, 1], "Pages opened during history navigation");
          t.done();
        }
      ), 500);
    }, 50);
  t.step(function() {
    win = window.open("history_entry.html?urls=history_back-1.html");
    t.add_cleanup(function() { win.close(); });
  });
</script>
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
  "source_name": "html/browsers/history/the-history-interface/history_back_1.html"
}
```
