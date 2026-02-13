# html/webappapis/dynamic-markup-insertion/document-write/script_006.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/dynamic-markup-insertion/document-write/script_006.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>document.write external script followed by internal script</title>
<script src="/resources/testharness.js"></script><script src="/resources/testharnessreport.js"></script>
<script>
var t = async_test();
var order = [];
t.step(function() {
  order.push(1);
  document.write("<script src='006.js'><"+"/script><script>t.step(function(){order.push(4)})</script"+">");
  order.push(2);
});
</script>
<script>
t.step(function() {
  order.push(5);
  assert_array_equals(order, [1,2,3,4,5]);
});
t.done();
</script>
<div id="log"></div>
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
  "source_name": "html/webappapis/dynamic-markup-insertion/document-write/script_006.html"
}
```
