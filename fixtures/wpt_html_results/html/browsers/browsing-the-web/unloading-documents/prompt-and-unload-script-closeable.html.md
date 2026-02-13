# html/browsers/browsing-the-web/unloading-documents/prompt-and-unload-script-closeable.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/unloading-documents/prompt-and-unload-script-closeable.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>beforeunload and unload events fire after window.close() in script-closeable browsing context</title>
<meta name="timeout" content="long">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
beforeunload_fired = false;
var t = async_test();

onload = t.step_func(function() {
  window.close();
});

onbeforeunload = t.step_func(function() {
  beforeunload_fired = true;
});

onunload = t.step_func(function() {
  assert_true(beforeunload_fired);
  t.done()
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
  "source_name": "html/browsers/browsing-the-web/unloading-documents/prompt-and-unload-script-closeable.html"
}
```
