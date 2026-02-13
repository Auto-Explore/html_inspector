# html/browsers/history/the-location-interface/reload_document_write.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-location-interface/reload_document_write.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Reload document with document.written content</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
var win = window.open("reload_document_write-1.html");
var t = async_test();

window.onmessage = t.step_func(function(e) {
  var initial_value = e.data;
  win.location.reload();
  window.onmessage = t.step_func(function(e) {
    assert_not_equals(e.data, initial_value);
    t.done();
  });
});

add_completion_callback(function() {win.close()});
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
  "source_name": "html/browsers/history/the-location-interface/reload_document_write.html"
}
```
