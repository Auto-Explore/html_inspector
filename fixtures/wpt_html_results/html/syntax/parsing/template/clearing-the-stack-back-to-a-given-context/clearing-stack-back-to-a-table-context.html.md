# html/syntax/parsing/template/clearing-the-stack-back-to-a-given-context/clearing-stack-back-to-a-table-context.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing/template/clearing-the-stack-back-to-a-given-context/clearing-stack-back-to-a-table-context.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>HTML Templates: Clearing stack back to a table context</title>
<meta name="author" title="Sergey G. Grekhov" href="mailto:sgrekhov@unipro.ru">
<meta name="assert" content="Clearing the stack back to a table context must be aborted if the current node is template">
<link rel="help" href="http://www.w3.org/TR/2013/WD-html-templates-20130214/#clearing-the-stack">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/html/resources/common.js"></script>
</head>
<body>
<div id="log"></div>
<script type="text/javascript">

function doTest(doc, templateInnerHTML, id, tagName, bodiesNum = null, footerIsNull,
        headerIsNull) {

    doc.body.innerHTML = '' +
        '<table id="tbl">' +
        '<template id="tmpl1">' +
        // When parser meets <caption>, <colgroup>, <tbody>, <tfoot>, <thead>, <col>
        // stack must be cleared back to table context.
        //But <template> tag should abort this process
        templateInnerHTML +
        '</template>' +
        '<tr id="tr">' +
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
    assert_equals(template.parentNode, table, 'Wrong template parent');
    assert_not_equals(template.content.querySelector('#' + id), null,
                'Element should present in the template content');
    assert_equals(doc.querySelector('#tbl').caption, null, 'Table should have no caption');
    assert_equals(template.content.querySelector('#' + id).tagName, tagName,
                'Wrong element in the template content');
    if (bodiesNum !== null) {
        assert_equals(table.tBodies.length, bodiesNum, 'Table should have '
                + bodiesNum + ' body');
    }
    if (footerIsNull) {
        assert_equals(table.tFoot, null, 'Table should have no footer');
    }
    if (headerIsNull) {
        assert_equals(table.tHead, null, 'Table should have no header');
    }
}


var doc = newHTMLDocument();
var parameters = [
    ['Clearing stack back to a table context. Test <caption>',
     doc, '<caption id="caption1">Table caption</caption>', 'caption1', 'CAPTION'],

    ['Clearing stack back to a table context. Test <colgroup>',
     doc, '<colgroup id="colgroup1" width="100%"></colgroup>', 'colgroup1', 'COLGROUP'],

    ['Clearing stack back to a table context. Test <tbody>',
     doc, '<tbody id="tbody1"></tbody>', 'tbody1', 'TBODY', 1],

    ['Clearing stack back to a table context. Test <tfoot>',
     doc, '<tfoot id="tfoot1"></tfoot>', 'tfoot1', 'TFOOT', null, true],

    ['Clearing stack back to a table context. Test <thead>',
     doc, '<thead id="thead1"></thead>', 'thead1', 'THEAD', null, false, true],

    ['Clearing stack back to a table context. Test <col>',
     doc, '<col id="col1" width="100%"/>', 'col1', 'COL']
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
        "byte_end": 179,
        "byte_start": 100,
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
        "byte_end": 179,
        "byte_start": 100,
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
        "byte_end": 623,
        "byte_start": 592,
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
  "source_name": "html/syntax/parsing/template/clearing-the-stack-back-to-a-given-context/clearing-stack-back-to-a-table-context.html"
}
```
