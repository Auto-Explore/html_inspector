# html/dom/elements/global-attributes/dataset.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/dataset.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>dataset: should exist and work on HTML and SVG elements, but not random elements</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
var div = document.createElement("div");
test(function() {
  assert_true(div.dataset instanceof DOMStringMap);
}, "HTML elements should have a .dataset");
test(function() {
  assert_false("foo" in div.dataset);
  assert_equals(div.dataset.foo, undefined);
}, "Should return 'undefined' before setting an attribute")
test(function() {
  div.setAttribute("data-foo", "value");
  assert_true("foo" in div.dataset);
  assert_equals(div.dataset.foo, "value");
}, "Should return 'value' if that's the value")
test(function() {
  div.setAttribute("data-foo", "");
  assert_true("foo" in div.dataset);
  assert_equals(div.dataset.foo, "");
}, "Should return the empty string if that's the value")
test(function() {
  div.removeAttribute("data-foo");
  assert_false("foo" in div.dataset);
  assert_equals(div.dataset.foo, undefined);
}, "Should return 'undefined' after removing an attribute")
test(function() {
  assert_equals(document.createElementNS("test", "test").dataset, undefined);
}, "Should not have a .dataset on random elements");
test(function() {
  var svg = document.createElementNS("http://www.w3.org/2000/svg", "svg")
  assert_true(svg.dataset instanceof DOMStringMap);
}, "SVG elements should have a .dataset");
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
  "source_name": "html/dom/elements/global-attributes/dataset.html"
}
```
