# html/obsolete/requirements-for-implementations/other-elements-attributes-and-apis/heading-obsolete-attributes-01.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/obsolete/requirements-for-implementations/other-elements-attributes-and-apis/heading-obsolete-attributes-01.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>HTMLHeadingElement: obsolete attribute reflecting</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-hx-align">
<link rel="help" href="https://webidl.spec.whatwg.org/#es-DOMString">
<link rel="help" href="http://www.ecma-international.org/publications/files/ECMA-ST/ECMA-262.pdf#page=57">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
test(function() {
  var el = document.createElement("h7");
  el.align = "left";
  assert_equals(el.align, "left");
  assert_false(el.hasAttribute("align"));
  assert_equals(el.getAttribute("align"), null);
}, "IDL attributes for HTMLHeadingElement should not apply to h7.")
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
  "source_name": "html/obsolete/requirements-for-implementations/other-elements-attributes-and-apis/heading-obsolete-attributes-01.html"
}
```
