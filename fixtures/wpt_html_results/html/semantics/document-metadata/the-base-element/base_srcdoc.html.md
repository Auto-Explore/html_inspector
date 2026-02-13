# html/semantics/document-metadata/the-base-element/base_srcdoc.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-base-element/base_srcdoc.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>base element in srcdoc document should resolve against its fallback base URI</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<iframe srcdoc=""></iframe>
<script>
var t = async_test();
addEventListener("load", t.step_func_done(function() {
  var doc = frames[0].document;
  var b = doc.createElement("base");
  b.setAttribute("href", "test");
  var newBaseValue = location.href.replace(/\/[^/]*$/, "/") + "test";
  assert_equals(b.href, newBaseValue);
  assert_equals(doc.baseURI, location.href);
  doc.head.appendChild(b);
  assert_equals(doc.baseURI, newBaseValue);
}));
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
  "source_name": "html/semantics/document-metadata/the-base-element/base_srcdoc.html"
}
```
