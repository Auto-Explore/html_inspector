# html/dom/documents/dom-tree-accessors/nameditem-03.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/documents/dom-tree-accessors/nameditem-03.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Named items: applets</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-document-nameditem">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<applet name=test1></applet>
<script>
test(function() {
  var applet = document.getElementsByTagName("applet")[0];
  assert_equals(applet.name, undefined);

  assert_false("test1" in document, '"test1" in document should be false');
  assert_equals(document.test1, undefined);
}, "applet elements are (mostly) gone");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.element.applet.obsolete",
      "message": "The “applet” element is obsolete. Use “embed” or “object” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 373,
        "byte_start": 354,
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
  "source_name": "html/dom/documents/dom-tree-accessors/nameditem-03.html"
}
```
