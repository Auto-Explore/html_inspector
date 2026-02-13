# html/browsers/history/the-history-interface/non-automated/traverse_the_session_history_unload_prompt_1-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-history-interface/non-automated/traverse_the_session_history_unload_prompt_1-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Traversing the history, unload event is fired on doucment</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
  var t = async_test();
  started = false;
  pages = []
  timer = null;
  unload_ran = false;
  start_test_wait = t.step_func(
    function() {
      clearTimeout(timer);
      timer = setTimeout(t.step_func(
        function() {
          try {
            assert_array_equals(pages, [2], "Pages opened during history navigation");
            assert_true(unload_ran, "Unload event handler ran");
            t.done();
          } finally {
           // win.close();
          }
        }
      ), 500);
    }
  );
  t.step(function() {win = window.open("history_entry.html?urls=traverse_the_history_unload_prompt_1-1.html");
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
  "source_name": "html/browsers/history/the-history-interface/non-automated/traverse_the_session_history_unload_prompt_1-manual.html"
}
```
