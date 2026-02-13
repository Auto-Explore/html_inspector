# html/dom/documents/dom-tree-accessors/document.getElementsByName/document.getElementsByName-param.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/documents/dom-tree-accessors/document.getElementsByName/document.getElementsByName-param.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>getElementsByName and the param element</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-document-getelementsbyname">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<div id="test">
<param name="test1">
<object>
<param name="test2">
</object>
</div>
<script>
test(function() {
  assert_equals(document.getElementsByName("test1").length, 1);
  assert_equals(document.getElementsByName("test1")[0],
                document.getElementsByTagName("param")[0]);
  assert_equals(document.getElementsByName("test2").length, 1);
  assert_equals(document.getElementsByName("test2")[0],
                document.getElementsByTagName("param")[1]);
});
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.element.param.obsolete",
      "message": "The “param” element is obsolete. Use the “data” attribute of the “object” element to set the URL of the external resource.",
      "severity": "Warning",
      "span": {
        "byte_end": 396,
        "byte_start": 376,
        "col": 1,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 405,
        "byte_start": 397,
        "col": 1,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.element.param.obsolete",
      "message": "The “param” element is obsolete. Use the “data” attribute of the “object” element to set the URL of the external resource.",
      "severity": "Warning",
      "span": {
        "byte_end": 426,
        "byte_start": 406,
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
  "source_name": "html/dom/documents/dom-tree-accessors/document.getElementsByName/document.getElementsByName-param.html"
}
```
