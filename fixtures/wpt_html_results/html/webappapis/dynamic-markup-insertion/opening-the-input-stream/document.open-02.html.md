# html/webappapis/dynamic-markup-insertion/opening-the-input-stream/document.open-02.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/dynamic-markup-insertion/opening-the-input-stream/document.open-02.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>document.open with three arguments</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-document-open">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
function open() {
  assert_unreached("The call should be redirected to the real window.open")
}
test(function(t) {
  var w;
  t.add_cleanup(function() {try {w.close()} catch(e) {}});
  w = document.open("/resources/testharness.js", "", "");
  assert_true(w instanceof w.Window, "Expected a window");
}, "document.open should redirect to window.open when called with three arguments");

test(function() {
  var parser = new DOMParser();
  var doc = parser.parseFromString("", "text/html");
  assert_equals(doc.defaultView, null);
  assert_throws_dom("INVALID_ACCESS_ERR", function() {
    doc.open("/resources/testharness.js", "", "");
  });
}, "document.open should throw when it has no window and is called with three arguments");
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
  "source_name": "html/webappapis/dynamic-markup-insertion/opening-the-input-stream/document.open-02.html"
}
```
