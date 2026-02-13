# html/webappapis/dynamic-markup-insertion/document-write/iframe_007.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/dynamic-markup-insertion/document-write/iframe_007.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>document.write comment into iframe</title>
<script src="/resources/testharness.js"></script><script src="/resources/testharnessreport.js"></script>
<iframe id="test"></iframe>
<script>
test(function() {
  var iframe = document.getElementById("test");
  var s = "<!--Filler-->";
  for (var i=0; i<s.length; i++) {
    iframe.contentDocument.write(s);
  }
  iframe.contentDocument.close();
  assert_equals(iframe.contentDocument.childNodes[0].nodeType, document.COMMENT_NODE);
  assert_equals(iframe.contentDocument.childNodes[0].data, "Filler");
});
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
  "source_name": "html/webappapis/dynamic-markup-insertion/document-write/iframe_007.html"
}
```
