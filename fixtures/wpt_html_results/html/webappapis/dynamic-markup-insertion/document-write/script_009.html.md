# html/webappapis/dynamic-markup-insertion/document-write/script_009.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/dynamic-markup-insertion/document-write/script_009.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>document.write script that document.writes script</title>
<script src="/resources/testharness.js"></script><script src="/resources/testharnessreport.js"></script>
<script>
var t = async_test();
var order = [];
t.step(function() {
  order.push(1);
  document.write("<script>order.push(2); document.write('<script>order.push(3); document.write(\"<script>order.push(4);</script\"+\">\"); order.push(5);</script' + '>'); order.push(6);</script" + ">");
  order.push(7);
});
</script>
<script>
t.step(function() {
  assert_array_equals(order, [1,2,3,4,5,6,7]);
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
  "source_name": "html/webappapis/dynamic-markup-insertion/document-write/script_009.html"
}
```
