# html/browsers/history/the-location-interface/location_assign_about_blank.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-location-interface/location_assign_about_blank.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>location.assign with initial about:blank browsing context</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<iframe></iframe>
<script>
var t = async_test();
var history_length;
onload = t.step_func(function() {
  setTimeout(t.step_func(function() {
    var iframe = document.getElementsByTagName("iframe")[0];
    iframe.onload = t.step_func(function() {
        setTimeout(t.step_func(function() {
          assert_equals(history.length, history_length);
          t.done();
        }), 100);
    });
    history_length = history.length;
    iframe.src = "location_assign_about_blank-1.html"
  }), 100);
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
  "source_name": "html/browsers/history/the-location-interface/location_assign_about_blank.html"
}
```
