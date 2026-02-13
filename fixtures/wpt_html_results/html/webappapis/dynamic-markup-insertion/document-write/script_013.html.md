# html/webappapis/dynamic-markup-insertion/document-write/script_013.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/dynamic-markup-insertion/document-write/script_013.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>document.write</title>
<script src="/resources/testharness.js"></script><script src="/resources/testharnessreport.js"></script>
<script>
var t = async_test();
t.step(function() {
  var s = "<script src='013.js'><" + "/script></svg>]]><path></svg>";
  for (var i=0; i<s.length; i++) {
    document.write(s[i]);
  }
});
</script><script>
t.step(function() {
  assert_equals(document.body.childNodes[0].nodeType, document.ELEMENT_NODE);
  assert_equals(document.body.childNodes[0].localName, "svg");
  assert_equals(document.body.childNodes[0].childNodes[0].nodeType, document.TEXT_NODE);
  assert_equals(document.body.childNodes[0].childNodes[0].data, "</svg>");
  assert_equals(document.body.childNodes[0].childNodes[1].nodeType, document.ELEMENT_NODE);
  assert_equals(document.body.childNodes[0].childNodes[1].localName, "path");
}
);
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
  "source_name": "html/webappapis/dynamic-markup-insertion/document-write/script_013.html"
}
```
