# html/syntax/parsing/template/additions-to-the-in-table-insertion-mode/end-tag-table.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing/template/additions-to-the-in-table-insertion-mode/end-tag-table.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>HTML Templates: 'In table' insertion mode: ignore TABLE end tag</title>
<meta name="author" title="Sergey G. Grekhov" href="mailto:sgrekhov@unipro.ru">
<meta name="assert" content="If parser is in 'in table' insertion mode and end tag table is met the ignore this token">
<link rel="help" href="http://www.w3.org/TR/2013/WD-html-templates-20130214/#in-table-addition">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/html/resources/common.js"></script>
</head>
<body>
<div id="log"></div>
<script type="text/javascript">


test(function () {
    var doc = newHTMLDocument();

    doc.body.innerHTML = '<table id="table">'
        + '<template id="template">'
        + '</table>'
        + '</template>'
        + '<tr><td></td></tr>'
        + '</table>';

    var table = doc.querySelector('#table');
    var template = table.querySelector('#template');

    assert_equals(table.childNodes.length, 2, 'Wrong number of table children');
    assert_not_equals(template, null, 'Template element must be parsed');
    assert_equals(table.rows.length, 1, 'Wrong number of table rows');
    assert_equals(template.childNodes.length, 0, 'Wrong number of the template child nodes');
    assert_equals(template.content.childNodes.length, 0,
            'Wrong number of the template child nodes');


}, 'In table insertion mode. Ignore </table> token');

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
        "byte_end": 188,
        "byte_start": 109,
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
        "byte_end": 188,
        "byte_start": 109,
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
        "byte_end": 629,
        "byte_start": 598,
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
  "source_name": "html/syntax/parsing/template/additions-to-the-in-table-insertion-mode/end-tag-table.html"
}
```
