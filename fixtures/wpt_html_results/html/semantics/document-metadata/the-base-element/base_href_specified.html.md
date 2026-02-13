# html/semantics/document-metadata/the-base-element/base_href_specified.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-base-element/base_href_specified.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: base_href_specified</title>
<link rel="author" title="Intel" href="http://www.intel.com/">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-base-element">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/get-host-info.sub.js"></script>
<base id="base">
<div id="log"></div>
<img id="test" src="test.ico" style="display:none">

<script>
  var testElement;
  var baseElement;

  var otherOrigin = get_host_info().HTTP_REMOTE_ORIGIN;

  setup(function() {
    testElement = document.getElementById("test");
    baseElement = document.getElementById("base");

    baseElement.setAttribute("href", otherOrigin);
  });

  test(function() {
    assert_equals(baseElement.href, otherOrigin + "/", "The href attribute of the base element is incorrect.");
  }, "The href attribute of the base element is specified");

  test(function() {
    assert_equals(testElement.src, otherOrigin + "/test.ico", "The src attribute of the img element is incorrect.");
  }, "The src attribute of the img element must relative to the href attribute of the base element");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.base.missing_href_or_target",
      "message": "Element “base” is missing one or more of the following attributes: “href”, “target”.",
      "severity": "Warning",
      "span": {
        "byte_end": 405,
        "byte_start": 389,
        "col": 1,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.base.must_come_before_link_or_script",
      "message": "The “base” element must come before any “link” or “script” elements in the document.",
      "severity": "Warning",
      "span": {
        "byte_end": 405,
        "byte_start": 389,
        "col": 1,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 478,
        "byte_start": 427,
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
  "source_name": "html/semantics/document-metadata/the-base-element/base_href_specified.html"
}
```
