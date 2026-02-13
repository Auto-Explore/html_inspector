# html/dom/documents/dom-tree-accessors/document.getElementsByName/document.getElementsByName-same.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/documents/dom-tree-accessors/document.getElementsByName/document.getElementsByName-same.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Calling getElementsByName with the same argument</title>
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
  var list1 = document.getElementsByName("abcd");
  var list2 = document.getElementsByName("abcd");
  assert_true(list1 === list2 || list1 !== list2);
}, "The user agent may return the same object as the object returned by the earlier call.");
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
        "byte_end": 402,
        "byte_start": 385,
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
  "source_name": "html/dom/documents/dom-tree-accessors/document.getElementsByName/document.getElementsByName-same.html"
}
```
