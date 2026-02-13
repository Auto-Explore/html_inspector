# html/browsers/history/the-history-interface/traverse_the_history_write_onload_2.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-history-interface/traverse_the_history_write_onload_2.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Traverse the history back and forward when a history entry is written in the load event</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
  var t = async_test();
  started = false;
  pages = []
  timer = setInterval(
    function() {
      if (pages.length < 5)
        return;
      clearInterval(timer);
      setTimeout(t.step_func(
        function() {
          //The pass condition here is based on the idea that the spec is wrong and browsers are right
          assert_array_equals(pages, [3, 4, 2, 3, 4], "Pages opened durning history navigation");
          t.done();
        }
      ), 500);
    }, 50);
  t.step(function() {
    win = window.open("history_entry.html?urls=history_forward-1.html,traverse_the_history_write_onload_2-1.html");
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
  "source_name": "html/browsers/history/the-history-interface/traverse_the_history_write_onload_2.html"
}
```
