# html/browsers/history/the-history-interface/traverse_the_history_write_after_load_1.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-history-interface/traverse_the_history_write_after_load_1.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Traverse the history after document.write after the load event</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
  var t = async_test();
  started = false;
  pages = []
  start_test_wait = t.step_func(
    function() {
      check_result = t.step_func(
        function() {
          if (pages.length < 3) {
              setTimeout(check_result, 500);
              return
          }
          assert_array_equals(pages, [2, 3, 1], "Pages opened during history navigation");
          t.done();
        }
      )
      setTimeout(check_result, 500);
    }
  );
  t.step(function() {
    win = window.open("history_entry.html?urls=traverse_the_history_write_after_load_1-1.html");
    t.add_cleanup(function() {win.close()});
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
  "source_name": "html/browsers/history/the-history-interface/traverse_the_history_write_after_load_1.html"
}
```
