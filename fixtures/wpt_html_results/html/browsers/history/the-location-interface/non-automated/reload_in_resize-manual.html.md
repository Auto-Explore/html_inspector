# html/browsers/history/the-location-interface/non-automated/reload_in_resize-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-location-interface/non-automated/reload_in_resize-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Reload called from resize event</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<p>Resize the popup window. That window should then close and the result be presented here. If that window doesn't close after resize that's a FAIL.</p>
<div id="log"></div>
<script>
setup({timeout:3600000})
var t = async_test();
var load_count = 0;
var resized = false;
var win = window.open("reload_in_resize-1.html")

flag_resized = t.step_func(function() {
  resized = true;
  setTimeout(do_test, 1000);
});

do_test = t.step_func(function() {
  win.close();
  assert_true(resized, "Resize event happened");
  assert_equals(load_count, 1, "Number of load events");
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
  "source_name": "html/browsers/history/the-location-interface/non-automated/reload_in_resize-manual.html"
}
```
