# html/syntax/serializing-xml-fragments/outerHTML.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/serializing-xml-fragments/outerHTML.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
    <title>HTML Test: element.outerHTML to verify XML fragment serialization algorithm</title>
    <link rel="author" title="Intel" href="http://www.intel.com/">
    <link rel="help" href="https://w3c.github.io/DOM-Parsing/#dfn-concept-serialize-xml">
    <link rel="help" href="https://w3c.github.io/DOM-Parsing/#widl-Element-outerHTML">
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
    <script src="../../resources/common.js"></script>
  </head>
  <body>
    <div id="log"></div>
    <script>
      test(function() {
        var doc = document.implementation.createDocument(null, "");
        assert_equals(doc.contentType, "application/xml");
        var html_ns = "http://www.w3.org/1999/xhtml";
        const non_void_elements = HTML5_ELEMENTS.filter(el => !HTML5_VOID_ELEMENTS.includes(el));
        non_void_elements.forEach(function(ele) {
          test(function() {
            var e = doc.createElementNS(html_ns, ele);
            assert_equals(e.outerHTML,
                          `<${ele} xmlns="${html_ns}"></${ele}>`,
                          ele + " node created." );
          }, "Node for " + ele);
        });
        HTML5_VOID_ELEMENTS.forEach(function(ele) {
          test(function() {
            var e = doc.createElementNS(html_ns, ele);
            assert_equals(e.outerHTML,
                          `<${ele} xmlns="${html_ns}" />`,
                          ele + " node created." );
          }, "Node for " + ele);
        });
      }, document.title);
    </script>
  </body>
</html>
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
  "source_name": "html/syntax/serializing-xml-fragments/outerHTML.html"
}
```
