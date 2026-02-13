# html/browsers/history/joint-session-history/joint-session-history-remove-iframe.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/joint-session-history/joint-session-history-remove-iframe.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>Joint session history length does not include entries from a removed iframe.</title>
<meta name="timeout" content="long">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<iframe id="frame" src="about:blank"></iframe>
<script>
async_test(function(t) {
    t.step_timeout(() => {
        var child = document.getElementById("frame");
        var old_history_len = history.length;
        child.onload = t.step_func_done(() => {
            assert_equals(old_history_len, history.length);
            document.body.removeChild(document.getElementById("frame"));
            assert_equals(old_history_len, history.length);
        })
        child.src = "joint-session-history-filler.html";
    }, 1000);
});
</script>
</body>
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
  "source_name": "html/browsers/history/joint-session-history/joint-session-history-remove-iframe.html"
}
```
