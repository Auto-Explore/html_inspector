# html/obsolete/requirements-for-implementations/other-elements-attributes-and-apis/document-color-02.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/obsolete/requirements-for-implementations/other-elements-attributes-and-apis/document-color-02.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>document: fg/bg/link/vlink/alink-color</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-document-fgcolor">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-body-text">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
test(function() {
  assert_equals(document.fgColor, document.body.text);
  assert_equals(document.bgColor, document.body.bgColor);
  assert_equals(document.linkColor, document.body.link);
  assert_equals(document.vlinkColor, document.body.vLink);
  assert_equals(document.alinkColor, document.body.aLink);
})
test(function() {
  document.fgColor = null;
  assert_equals(document.fgColor, "");
  assert_equals(document.body.text, "");
  assert_equals(document.body.getAttribute("text"), "");
})
test(function() {
  document.fgColor = "blue";
  assert_equals(document.fgColor, "blue");
  assert_equals(document.body.text, "blue");
  assert_equals(document.body.getAttribute("text"), "blue");
})
test(function() {
  document.bgColor = "green";
  assert_equals(document.bgColor, "green");
  assert_equals(document.body.bgColor, "green");
  assert_equals(document.body.getAttribute("bgcolor"), "green");
})
test(function() {
  document.linkColor = "red";
  assert_equals(document.linkColor, "red");
  assert_equals(document.body.link, "red");
  assert_equals(document.body.getAttribute("link"), "red");
})
test(function() {
  document.vlinkColor = "yellow";
  assert_equals(document.vlinkColor, "yellow");
  assert_equals(document.body.vLink, "yellow");
  assert_equals(document.body.getAttribute("vlink"), "yellow");
})
test(function() {
  document.alinkColor = "silver";
  assert_equals(document.alinkColor, "silver");
  assert_equals(document.body.aLink, "silver");
  assert_equals(document.body.getAttribute("alink"), "silver");
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
  "source_name": "html/obsolete/requirements-for-implementations/other-elements-attributes-and-apis/document-color-02.html"
}
```
