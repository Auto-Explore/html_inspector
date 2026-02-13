# html/browsers/history/the-history-interface/non-automated/traverse_the_history_unload_prompt_2-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-history-interface/non-automated/traverse_the_history_unload_prompt_2-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Traversing the history, prompt in before unload, navigation allowed</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
  setup({timeout:3600000});
  var t = async_test();
  started = false;
  pages = []
  timer = null;
  beforeunload_ran = false;
  start_test_wait = t.step_func(
    function() {
      clearTimeout(timer);
      timer = setTimeout(t.step_func(
        function() {
          try {
            assert_true(beforeunload_ran, "beforeunload event handler ran");
            assert_array_equals(pages, [2,1], "Pages opened during history navigation");
            t.done();
          } finally {
            win.close();
          }
        }
      ), 500);
    }
  );
  t.step(function() {win = window.open("history_entry.html?urls=traverse_the_history_unload_prompt_2-1.html");
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
  "source_name": "html/browsers/history/the-history-interface/non-automated/traverse_the_history_unload_prompt_2-manual.html"
}
```
