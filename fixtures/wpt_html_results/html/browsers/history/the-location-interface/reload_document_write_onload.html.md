# html/browsers/history/the-location-interface/reload_document_write_onload.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-location-interface/reload_document_write_onload.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Reload document with document.written content written in load event</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
var win = window.open("reload_document_write_onload-1.html");
var t = async_test();

var data = [];

window.onmessage = t.step_func(function(e) {
  data.push(e.data);
  if (data.length < 3) {
    win.location.reload();
  } else {
    setTimeout(t.step_func(function() {
      assert_array_equals(data, ["original", "written", "written"]);
      t.done();
    }), 500);
  }
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
  "source_name": "html/browsers/history/the-location-interface/reload_document_write_onload.html"
}
```
