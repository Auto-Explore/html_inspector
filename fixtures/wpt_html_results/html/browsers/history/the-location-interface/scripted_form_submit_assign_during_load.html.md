# html/browsers/history/the-location-interface/scripted_form_submit_assign_during_load.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-location-interface/scripted_form_submit_assign_during_load.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Assignment to location with form submit during load</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<p>The popup blocker must be disabled for this test</p>
<div id="log"></div>
<script>
setup({timeout:3600000});
var t = async_test();
var win = window.open("scripted_form_submit_assign_during_load-1.html");

var history_length;
test_initial_history_length = t.step_func(function(new_length) {
  history_length = new_length;
  assert_equals(history_length, 1, "Should have inital history");
});
test_assign_during_load = t.step_func(function(new_length) {
  assert_equals(new_length, history_length, "Assigning to Location during load should replace");
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
  "source_name": "html/browsers/history/the-location-interface/scripted_form_submit_assign_during_load.html"
}
```
