# html/semantics/document-metadata/the-base-element/base_href_unspecified.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-base-element/base_href_unspecified.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: base_href_unspecified</title>
<link rel="author" title="Intel" href="http://www.intel.com/">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-base-element">
<base id="base" target="_blank">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id="log"></div>
<img id="test" src="/images/blue-100x100.png" style="display:none">

<script>
  var testElement,
      baseElement;

  setup(function () {
    testElement = document.getElementById("test");
    baseElement = document.getElementById("base");
  });

  test(function() {
    assert_equals(baseElement.href, document.location.href, "Return the document base URL if the base element has no href content attribute.");
  }, "The value of the href attribute must be the document's address if it is unspecified");

  test(function() {
    var exp = testElement.src.substring(0, testElement.src.lastIndexOf("/images/blue-100x100.png") + 1);
    assert_true(baseElement.href.indexOf(exp) != -1, "The src attribute of the img element is incorrect.");
  }, "The src attribute of the img element must relative to document's address");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.base.must_come_before_link_or_script",
      "message": "The “base” element must come before any “link” or “script” elements in the document.",
      "severity": "Warning",
      "span": {
        "byte_end": 264,
        "byte_start": 232,
        "col": 1,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 460,
        "byte_start": 393,
        "col": 1,
        "line": 11
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/document-metadata/the-base-element/base_href_unspecified.html"
}
```
