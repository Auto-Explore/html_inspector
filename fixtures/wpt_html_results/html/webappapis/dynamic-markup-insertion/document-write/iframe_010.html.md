# html/webappapis/dynamic-markup-insertion/document-write/iframe_010.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/dynamic-markup-insertion/document-write/iframe_010.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>document.write plaintext</title>
<script src="/resources/testharness.js"></script><script src="/resources/testharnessreport.js"></script>
<iframe id="test"></iframe>
<script>
var t = async_test();
var iframe = document.getElementById("test");

function check_dom() {
  assert_equals(iframe.contentDocument.body.childNodes[0].localName, "plaintext")
  assert_equals(iframe.contentDocument.body.childNodes[0].textContent, "Filler ")
  assert_equals(iframe.contentDocument.body.childNodes[1].localName, "table")
}

t.step(function() {
  var s = "<script>document.write('<table><plaintext>Filler '); document.close(); top.t.step(function() {top.check_dom()})</script" + ">";
  for (var i=0; i<s.length; i++) {
    iframe.contentDocument.write(s[i]);
  }
  t.done();
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
  "source_name": "html/webappapis/dynamic-markup-insertion/document-write/iframe_010.html"
}
```
