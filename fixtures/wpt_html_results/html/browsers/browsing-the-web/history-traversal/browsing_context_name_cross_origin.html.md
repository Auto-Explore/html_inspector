# html/browsers/browsing-the-web/history-traversal/browsing_context_name_cross_origin.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/history-traversal/browsing_context_name_cross_origin.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Restoring window.name on cross-origin history traversal</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<pre id="step_log"></pre>

<script>
var t = async_test();
t.step(() => {
  var win = window.open("browsing_context_name_cross_origin-0.html");
  t.add_cleanup(() => win.close());
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
  "source_name": "html/browsers/browsing-the-web/history-traversal/browsing_context_name_cross_origin.html"
}
```
