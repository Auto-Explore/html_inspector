# html/browsers/history/the-location-interface/non-automated/manual_click_location_replace_during_load-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-location-interface/non-automated/manual_click_location_replace_during_load-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>location.replace with click during load</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<p>The popup blocker must be disabled for this test</p>
<div id="log"></div>
<script>
setup({timeout:3600000});
var t = async_test();
var win = window.open("manual_click_location_replace_during_load-1.html");

var history_length;
do_test = t.step_func(function(new_length) {
  assert_equals(new_length, history_length);
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
  "source_name": "html/browsers/history/the-location-interface/non-automated/manual_click_location_replace_during_load-manual.html"
}
```
