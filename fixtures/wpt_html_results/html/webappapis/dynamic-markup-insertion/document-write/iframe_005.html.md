# html/webappapis/dynamic-markup-insertion/document-write/iframe_005.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/dynamic-markup-insertion/document-write/iframe_005.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>document.write external script into iframe write back into parent</title>
<script src="/resources/testharness.js"></script><script src="/resources/testharnessreport.js"></script>
<iframe id="test"></iframe>
<script>
var t = async_test();
var iframe = document.getElementById("test");
var order = [];
t.step(function() {
  order.push(1);
  var s = "<script src='iframe_005.js'></script" + ">";
  iframe.contentDocument.write(s);
  iframe.contentDocument.close();
  order.push(2);
  assert_array_equals(order, [1,2])
}
);
addEventListener("load", function() {
    t.step(function() {
      assert_array_equals(order, [1,2,3,4,5])
    });
    t.done();
}, false);
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
  "source_name": "html/webappapis/dynamic-markup-insertion/document-write/iframe_005.html"
}
```
