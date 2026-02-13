# html/browsers/browsing-the-web/history-traversal/history-traversal-navigate-parent-while-child-loading.html

Counts:
- errors: 2
- warnings: 0
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/history-traversal/history-traversal-navigate-parent-while-child-loading.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<iframe id="i"></iframe>
<body>
<script>
async_test(t => {
  let starting_history_length = history.length;
  let iframe_url = (new URL("/common/blank.html", location.href)).href;
  i.src = iframe_url;

  history.pushState("a", "", "#a");
  assert_equals(history.length, starting_history_length + 1, "First history length");

  i.onload = t.step_func(() => {
    assert_equals(history.length, starting_history_length + 1, "Second history length");
    assert_equals(i.contentWindow.location.href, iframe_url);
    assert_equals(location.hash, "#a");
    history.back();
    // Wait a while for a back navigation. Since all of the possible outcomes
    // are either same-document or navigating to about:blank, this doesn't need
    // to wait terribly long.
    t.step_timeout(t.step_func_done(() => {
      assert_equals(location.hash, "", "top frame should have navigated back");
      assert_equals(i.contentWindow.location.href, iframe_url, "iframe should not have navigated");
    }), 100);
  });
}, "pushState() in parent while child is doing initial navigation, then go back");
</script>
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.body.already_open",
      "message": "Start tag “body” seen but an element of the same type was already open.",
      "severity": "Error",
      "span": {
        "byte_end": 153,
        "byte_start": 147,
        "col": 1,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.parser.cannot_recover",
      "message": "Cannot recover after last error. Any further errors will be ignored.",
      "severity": "Error",
      "span": {
        "byte_end": 153,
        "byte_start": 147,
        "col": 1,
        "line": 5
      }
    }
  ],
  "source_name": "html/browsers/browsing-the-web/history-traversal/history-traversal-navigate-parent-while-child-loading.html"
}
```
