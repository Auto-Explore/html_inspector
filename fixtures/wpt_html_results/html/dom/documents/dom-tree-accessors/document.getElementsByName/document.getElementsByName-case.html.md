# html/dom/documents/dom-tree-accessors/document.getElementsByName/document.getElementsByName-case.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/documents/dom-tree-accessors/document.getElementsByName/document.getElementsByName-case.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>getElementsByName and case</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-document-getelementsbyname">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<div id="test">
<div name="abcd"></div>
</div>
<script>
test(function() {
  assert_equals(document.getElementsByName("ABCD").length, 0);
  assert_equals(document.getElementsByName("abcd").length, 1);
});
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.div.name.disallowed",
      "message": "Attribute “name” not allowed on element “div” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 380,
        "byte_start": 363,
        "col": 1,
        "line": 9
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
  "source_name": "html/dom/documents/dom-tree-accessors/document.getElementsByName/document.getElementsByName-case.html"
}
```
