# html/semantics/document-metadata/the-link-element/link-style-error-01.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-link-element/link-style-error-01.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>link: error events</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-link-element">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src=/common/get-host-info.sub.js></script>
<div id="log"></div>
<div id="test">
<script>
var t404 = async_test("Should get an error event for a 404 error.")
t404.step(function() {
  var elt = document.createElement("link");
  elt.onerror = t404.step_func(function() {
    assert_true(true, "Got error event for 404 error.")
    t404.step_timeout(function() { t404.done() }, 0);
  })
  elt.onload = t404.unreached_func("load event should not be fired");
  elt.rel = "stylesheet";
  elt.href = "nonexistent_stylesheet.css";
  document.getElementsByTagName("head")[0].appendChild(elt);
})

var tUnsupported = async_test("Should get an error event for an unsupported URL.")
tUnsupported.step(function() {
  var elt = document.createElement("link");
  elt.onerror = tUnsupported.step_func(function() {
    assert_true(true, "Got error event for unsupported URL.")
    tUnsupported.step_timeout(function() { tUnsupported.done() }, 0);
  })
  elt.onload = tUnsupported.unreached_func("load event should not be fired");
  elt.rel = "stylesheet";
  elt.href = "nonexistent:stylesheet.css";
  document.getElementsByTagName("head")[0].appendChild(elt);
});
</script>
<script src=resources/link-style-error.js></script>
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
  "source_name": "html/semantics/document-metadata/the-link-element/link-style-error-01.html"
}
```
