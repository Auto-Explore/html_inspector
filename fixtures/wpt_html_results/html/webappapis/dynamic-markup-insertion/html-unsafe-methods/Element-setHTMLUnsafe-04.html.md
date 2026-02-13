# html/webappapis/dynamic-markup-insertion/html-unsafe-methods/Element-setHTMLUnsafe-04.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/dynamic-markup-insertion/html-unsafe-methods/Element-setHTMLUnsafe-04.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>setHTMLUnsafe in HTML</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel=author href="mailto:jarhar@chromium.org">
<!-- This test was adapted from innerhtml-04.html -->
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
function testIsChild(p, c) {
  assert_equals(p.firstChild, c);
  assert_equals(c.parentNode, p);
}
test(function() {
  var p = document.createElement('p');
  var b = p.appendChild(document.createElement('b'));
  var t = b.appendChild(document.createTextNode("foo"));
  testIsChild(p, b);
  testIsChild(b, t);
  assert_equals(t.data, "foo");
  p.setHTMLUnsafe("");
  testIsChild(b, t);
  assert_equals(t.data, "foo");
}, "setHTMLUnsafe should leave the removed children alone.")
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
  "source_name": "html/webappapis/dynamic-markup-insertion/html-unsafe-methods/Element-setHTMLUnsafe-04.html"
}
```
