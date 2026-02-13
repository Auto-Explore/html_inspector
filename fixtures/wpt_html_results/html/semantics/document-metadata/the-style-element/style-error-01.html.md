# html/semantics/document-metadata/the-style-element/style-error-01.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-style-element/style-error-01.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>style: error events</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-style-element">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<div id="test">
<script>
//var t404 = async_test("Should get an error event for a 404 error.")
//t404.step(function() {
//  var elt = document.createElement("style");
//  elt.onerror = t404.step_func(function() {
//    assert_true(true, "Got error event for 404 error.")
//    t404.done()
//  })
//  elt.appendChild(
//    document.createTextNode('@import 404 error;'));
//  document.getElementsByTagName("head")[0].appendChild(elt);
//})
var tText = async_test("Should get an error event for a text/plain response.")
tText.step(function() {
  var elt = document.createElement("style");
  elt.onerror = tText.step_func(function() {
    assert_true(true, "Got error event for 404 error.")
    tText.done()
  })
  elt.appendChild(
    document.createTextNode('@import "support/css-red.txt";'));
  document.getElementsByTagName("head")[0].appendChild(elt);
})
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
  "source_name": "html/semantics/document-metadata/the-style-element/style-error-01.html"
}
```
