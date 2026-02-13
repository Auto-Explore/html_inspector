# html/browsers/history/the-history-interface/traverse_the_history_1.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-history-interface/traverse_the_history_1.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Multiple history traversals from the same task</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
  var t = async_test();
  started = false;
  pages = []
  timer = null;
  start_test_wait = t.step_func(
    function() {
      clearTimeout(timer);
      timer = setTimeout(t.step_func(
        function() {
          assert_array_equals(pages, [4, 2], "Pages opened during history navigation");
          t.done();
        }
      ), 500);
    }
  );
  t.step(function() {win = window.open("history_entry.html?urls=history_entry.html,history_entry.html,traverse_the_history_1-1.html");
  t.add_cleanup(() => { win.close() });
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
  "source_name": "html/browsers/history/the-history-interface/traverse_the_history_1.html"
}
```
