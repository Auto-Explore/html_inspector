# html/dom/documents/dom-tree-accessors/document.getElementsByName/document.getElementsByName-null-undef.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/documents/dom-tree-accessors/document.getElementsByName/document.getElementsByName-null-undef.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Calling getElementsByName with null and undefined</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-document-getelementsbyname">
<link rel="help" href="https://webidl.spec.whatwg.org/#es-DOMString">
<link rel="help" href="http://www.ecma-international.org/publications/files/ECMA-ST/ECMA-262.pdf#page=57">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
test(function() {
  var n = document.createElement("div");
  n.setAttribute("name", "null");

  document.body.appendChild(n);
  this.add_cleanup(function() { document.body.removeChild(n) });

  assert_equals(document.getElementsByName(null)[0], n);
}, "getElementsByName(null)");

test(function() {
  var u = document.createElement("div");
  u.setAttribute("name", "undefined");

  document.body.appendChild(u);
  this.add_cleanup(function() { document.body.removeChild(u) });

  assert_equals(document.getElementsByName(undefined)[0], u);
}, "getElementsByName(undefined)");
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
  "source_name": "html/dom/documents/dom-tree-accessors/document.getElementsByName/document.getElementsByName-null-undef.html"
}
```
