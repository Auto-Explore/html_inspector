# html/browsers/browsing-the-web/history-traversal/api-availability.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/history-traversal/api-availability.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>API availability following history traversal</title>
<meta charset=utf-8>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<p>Test requires popup blocker disabled</p>
<div id=log></div>
<script>
var t = async_test();
var hasNavigated = false;
var child;
t.step(function() {
  child = window.open("resources/api-availability-1.html");
  t.add_cleanup(function() {
    child.close();
  });
});
navigate = t.step_func(function() {
  hasNavigated = true;
  child.location = child.location.href.replace("api-availability-1.html", "api-availability-2.html");
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
  "source_name": "html/browsers/browsing-the-web/history-traversal/api-availability.html"
}
```
