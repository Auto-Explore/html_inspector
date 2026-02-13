# html/browsers/history/the-location-interface/reload_post_1-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-location-interface/reload_post_1-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Reload document with POST</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
var win = window.open("resources/reload_post_1-1.py");
var t = async_test();
var posted = false;
var reloaded = false;

next = t.step_func(function() {

if (posted && !reloaded) {
  reloaded = true;
  win.location.reload();
} else if (posted && reloaded) {
  t.done();
} else {
  posted = true;
  win.document.forms[0].submit();
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
  "source_name": "html/browsers/history/the-location-interface/reload_post_1-manual.html"
}
```
