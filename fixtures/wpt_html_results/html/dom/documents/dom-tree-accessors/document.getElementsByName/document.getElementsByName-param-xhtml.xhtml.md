# html/dom/documents/dom-tree-accessors/document.getElementsByName/document.getElementsByName-param-xhtml.xhtml

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/documents/dom-tree-accessors/document.getElementsByName/document.getElementsByName-param-xhtml.xhtml",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
<title>getElementsByName and the param element</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com"/>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-document-getelementsbyname"/>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
</head>
<body>
<div id="log"></div>
<div id="test">
<param name="test1"/>
<object>
<param name="test2"/>
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
</body>
</html>
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
        "byte_end": 449,
        "byte_start": 428,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.object.data.missing",
      "message": "Element “object” is missing required attribute “data”.",
      "severity": "Warning",
      "span": {
        "byte_end": 458,
        "byte_start": 450,
        "col": 1,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.element.param.obsolete",
      "message": "The “param” element is obsolete. Use the “data” attribute of the “object” element to set the URL of the external resource.",
      "severity": "Warning",
      "span": {
        "byte_end": 480,
        "byte_start": 459,
        "col": 1,
        "line": 14
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
  "source_name": "html/dom/documents/dom-tree-accessors/document.getElementsByName/document.getElementsByName-param-xhtml.xhtml"
}
```
