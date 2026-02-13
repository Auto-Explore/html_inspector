# html/browsers/history/the-location-interface/assign_after_load.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-location-interface/assign_after_load.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Assignment to location after document is completely loaded</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<iframe></iframe>
<script>
var t = async_test();
var history_length;

onload = t.step_func(function() {
  setTimeout(function() {
    history_length = history.length;
    document.getElementsByTagName("iframe")[0].src = "assign_after_load-1.html";
  }, 100);
});

do_test = t.step_func(function() {
    assert_equals(history.length, history_length + 1);
    t.done();
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
  "source_name": "html/browsers/history/the-location-interface/assign_after_load.html"
}
```
