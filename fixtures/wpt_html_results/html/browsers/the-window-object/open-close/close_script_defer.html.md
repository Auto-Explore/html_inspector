# html/browsers/the-window-object/open-close/close_script_defer.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-window-object/open-close/close_script_defer.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Running defer script in window.close()</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
var t = async_test();
t.step(function() {
  var w = window.open("close_script_defer-1.html");
  w.document.open()
  w.document.write("<script defer src='callback.js'><\/script>")
  setTimeout(function() {
    w.close();
  }, 1000);
})
setTimeout(function() {t.done();}, 1000)
callback = t.step(function() {assert_unreached()})
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
  "source_name": "html/browsers/the-window-object/open-close/close_script_defer.html"
}
```
