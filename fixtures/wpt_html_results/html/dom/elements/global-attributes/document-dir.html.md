# html/dom/elements/global-attributes/document-dir.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/document-dir.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html dir="LTR">
<title>document.dir</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-document-dir">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#reflect">
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
test(function() {
  assert_equals(document.dir, "ltr");
  assert_equals(document.documentElement.getAttribute("dir"), "LTR");
}, "Markup attribute")
test(function() {
  document.dir = "x-garbage";
  assert_equals(document.dir, "");
  assert_equals(document.documentElement.getAttribute("dir"), "x-garbage");
}, "Setting the idl attribute to a garbage value")
test(function() {
  document.dir = "";
  assert_true(document.documentElement.hasAttribute("dir"), "Attribute should still be around");
  assert_equals(document.dir, "");
  assert_equals(document.documentElement.getAttribute("dir"), "");
}, "Setting the idl attribute to the empty string")
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
  "source_name": "html/dom/elements/global-attributes/document-dir.html"
}
```
