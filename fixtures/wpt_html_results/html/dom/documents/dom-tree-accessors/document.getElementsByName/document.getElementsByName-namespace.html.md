# html/dom/documents/dom-tree-accessors/document.getElementsByName/document.getElementsByName-namespace.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/documents/dom-tree-accessors/document.getElementsByName/document.getElementsByName-namespace.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>getElementsByName and foreign namespaces</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-document-getelementsbyname">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<div id="test">
<p name="math"><math name="math">
<mi>a</mi>
<mo>+</mo>
<mi>b</mi>
</math>
<p name="svg"><svg width="300" height="100" name="svg">
<rect width="300" height="100" fill="rgb(0,0,255)"/>
</svg>
</div>
<script>
test(function() {
  var ps = document.getElementById("test")
                   .getElementsByTagName("p");
  assert_equals(document.getElementsByName("math").length, 1);
  assert_equals(document.getElementsByName("math")[0], ps[0]);
  assert_equals(document.getElementsByName("svg").length, 1);
  assert_equals(document.getElementsByName("svg")[0], ps[1]);
});
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
  "source_name": "html/dom/documents/dom-tree-accessors/document.getElementsByName/document.getElementsByName-namespace.html"
}
```
