# html/syntax/parsing/template/additions-to-foster-parenting/template-is-a-foster-parent-element.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing/template/additions-to-foster-parenting/template-is-a-foster-parent-element.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>HTML Templates: Template is a foster parent element</title>
<meta name="author" title="Sergey G. Grekhov" href="mailto:sgrekhov@unipro.ru">
<meta name="assert" content="The last template element with either no table element is below it, or a table element immediately below it, in the stack of open elements is the foster parent element (NOT the template's parent!)">
<link rel="help" href="http://www.w3.org/TR/2013/WD-html-templates-20130214/#foster-parent-addition">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/html/resources/common.js"></script>
</head>
<body>
<div id="log"></div>
<script type="text/javascript">


test(function () {
    var doc = newHTMLDocument();

    doc.body.innerHTML = '' +
    '<div id="tmplParent">' +
        '<template id="tmpl1">' +
            '<table id="tbl">' +
                '<tr><td>Cell 1</td></tr>' +
            // Misplaced <div>. It should be foster parented
            '<div id="orphanDiv">Orphan div content</div>' +
                '<tr><td>Cell 2</td></tr>' +
            '</table>' +
        '</template>' +
    '</div>';

    var template = doc.querySelector('#tmpl1');
    var div = template.content.querySelector('#orphanDiv');

    assert_equals(div.parentNode, template.content, 'Wrong foster parent element');

}, 'Template is a foster parent element. Test <table> immediately below <template>');



test(function () {
    var doc = newHTMLDocument();

    doc.body.innerHTML = '' +
    '<div id="tmplParent">' +
        '<template id="tmpl1">' +
                '<tr><td>Cell 1</td></tr>' +
            // Misplaced <div>. It should be foster parented
            '<div id="orphanDiv">Orphan div content</div>' +
                '<tr><td>Cell 2</td></tr>' +
        '</template>' +
    '</div>';

    var template = doc.querySelector('#tmpl1');
    var div = template.content.querySelector('#orphanDiv');

    assert_equals(div.parentNode, template.content, 'Wrong foster parent element');

}, 'Template is a foster parent element. Test <template> element without <table>');

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
        "byte_end": 176,
        "byte_start": 97,
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
        "byte_end": 176,
        "byte_start": 97,
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
        "byte_end": 730,
        "byte_start": 699,
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
  "source_name": "html/syntax/parsing/template/additions-to-foster-parenting/template-is-a-foster-parent-element.html"
}
```
