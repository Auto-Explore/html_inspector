# html/syntax/parsing/template/clearing-the-stack-back-to-a-given-context/clearing-stack-back-to-a-table-row-context.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing/template/clearing-the-stack-back-to-a-given-context/clearing-stack-back-to-a-table-row-context.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>HTML Templates: Clearing stack back to a table row context</title>
<meta name="author" title="Sergey G. Grekhov" href="mailto:sgrekhov@unipro.ru">
<meta name="assert" content="Clearing the stack back to a table row context must be aborted if the current node is template">
<link rel="help" href="http://www.w3.org/TR/2013/WD-html-templates-20130214/#clearing-the-stack">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/html/resources/common.js"></script>
</head>
<body>
<div id="log"></div>
<script type="text/javascript">

function doTest(doc, templateInnerHTML, id, tagName, elementId) {

    doc.body.innerHTML = '' +
        '<table id="tbl">' +
        '<tr id="tr">' +
        '<template id="tmpl1">' +
        // When parser meets <th>, <td>, </tr>, stack must be cleared
        // back to table row context.
        // But <template> tag should abort this
        templateInnerHTML +
        '</template>' +
        '<td id="td">' +
        '</td>' +
        '</tr>' +
        '</table>';

    var table = doc.querySelector('#tbl');
    var tr = doc.querySelector('#tr');
    var td = doc.querySelector('#td');
    var template = doc.querySelector('#tmpl1');

    assert_equals(table.rows.length, 1, 'Wrong number of table rows');
    assert_equals(table.rows[0].cells.length, 1, 'Wrong number of table cells');
    assert_equals(template.parentNode, tr, 'Wrong template parent');
    if (id !== null) {
        assert_not_equals(template.content.querySelector('#' + id), null,
                    'Element should present in the template content');
    }
    if (tagName !== null) {
        assert_equals(template.content.querySelector('#' + id).tagName, tagName,
                'Wrong element in the template content');
    }
    if (elementId) {
        assert_equals(doc.querySelector('#' + elementId), null,
                'Table should have no element with ID ' + elementId);
    }
}


var doc = newHTMLDocument();
var parameters = [
    ['Clearing stack back to a table row context. Test <th>',
     doc, '<th id="th1">Table header</th>', 'th1', 'TH', 'th1'],

    ['Clearing stack back to a table row context. Test <td>',
     doc, '<td id="td1">Table cell</td>', 'td1', 'TD', 'td1'],

     ['Clearing stack back to a table row context. Test </tr>',
      doc, '</tr>', null, null]
];

// Clearing stack back to a table body context.
generate_tests(doTest, parameters);

</script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.attr.href.not_allowed",
      "message": "Attribute “href” not allowed on element “meta” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 183,
        "byte_start": 104,
        "col": 1,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.meta.missing_content",
      "message": "Element “meta” is missing one or more of the following attributes: “content”, “property”.",
      "severity": "Warning",
      "span": {
        "byte_end": 183,
        "byte_start": 104,
        "col": 1,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 631,
        "byte_start": 600,
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
  "source_name": "html/syntax/parsing/template/clearing-the-stack-back-to-a-given-context/clearing-stack-back-to-a-table-row-context.html"
}
```
