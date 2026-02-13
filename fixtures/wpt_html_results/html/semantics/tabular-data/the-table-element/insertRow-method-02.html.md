# html/semantics/tabular-data/the-table-element/insertRow-method-02.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/tabular-data/the-table-element/insertRow-method-02.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>insertRow(): Empty table</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-table-insertrow">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<div id="test">
<table></table>
</div>
<script>
var HTML = "http://www.w3.org/1999/xhtml";
test(function() {
  var table = document.getElementById("test").getElementsByTagName("table")[0];
  test(function() {
    assert_equals(table.childNodes.length, 0);
    assert_equals(table.rows.length, 0);
  }, "table should start out empty")

  var tr;
  test(function() {
    tr = table.insertRow(0);
    assert_equals(tr.localName, "tr");
    assert_equals(tr.namespaceURI, HTML);
  }, "insertRow should insert a tr element")

  var tbody = tr.parentNode;
  test(function() {
    assert_equals(tbody.localName, "tbody");
    assert_equals(tbody.namespaceURI, HTML);
    assert_equals(tbody.parentNode, table);
  }, "insertRow should insert a tbody element")
});
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
  "source_name": "html/semantics/tabular-data/the-table-element/insertRow-method-02.html"
}
```
