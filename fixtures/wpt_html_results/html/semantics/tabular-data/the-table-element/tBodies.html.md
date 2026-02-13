# html/semantics/tabular-data/the-table-element/tBodies.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/tabular-data/the-table-element/tBodies.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>HTMLTableElement.tBodies</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
test(function() {
  var text =
    '<html xmlns="http://www.w3.org/1999/xhtml">' +
    '  <head>' +
    '    <title>Virtual Library</title>' +
    '  </head>' +
    '  <body>' +
    '    <table id="mytable" border="1">' +
    '      <tbody>' +
    '        <tr><td>Cell 1</td><td>Cell 2</td></tr>' +
    '        <tr><td>Cell 3</td><td>Cell 4</td></tr>' +
    '      </tbody>' +
    '    </table>' +
    '  </body>' +
    '</html>';

  var parser = new DOMParser();
  var doc = parser.parseFromString(text, "text/xml");

  // import <table>
  var table = doc.documentElement.getElementsByTagName('table')[0];
  var mytable = document.body.appendChild(document.importNode(table, true));

  assert_equals(mytable.tBodies.length, 1);
  var tbody = document.createElement('tbody');
  mytable.appendChild(tbody);
  var tr = tbody.insertRow(-1);
  tr.insertCell(-1).appendChild(document.createTextNode('Cell 5'));
  tr.insertCell(-1).appendChild(document.createTextNode('Cell 6'));
  assert_equals(mytable.tBodies.length, 2);
  assert_equals(mytable.rows.length, 3);
  assert_equals(tr.rowIndex, 2);
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
  "source_name": "html/semantics/tabular-data/the-table-element/tBodies.html"
}
```
