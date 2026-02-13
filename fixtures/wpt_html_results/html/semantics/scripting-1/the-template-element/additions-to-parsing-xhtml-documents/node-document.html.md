# html/semantics/scripting-1/the-template-element/additions-to-parsing-xhtml-documents/node-document.html

Counts:
- errors: 0
- warnings: 6
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-template-element/additions-to-parsing-xhtml-documents/node-document.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>HTML Templates: Parsing XHTML: Node's node document</title>
<meta name="author" title="Sergey G. Grekhov" href="mailto:sgrekhov@unipro.ru">
<meta name="author" title="Aleksei Yu. Semenov" href="mailto:a.semenov@unipro.ru">
<meta name="assert" content="Parsing XHTML: Node's node document must be set to that of the element to which it will be appended">
<link rel="help" href="http://www.w3.org/TR/2013/WD-html-templates-20130214/#parsing-xhtml-documents">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src='/html/resources/common.js'></script>
</head>
<body>
<div id="log"></div>
<script type="text/javascript">



test(function() {
    var doc = newXHTMLDocument();
    doc.body = doc.createElement('body');
    doc.body.innerHTML = '<template id="tmpl"></template>';

    var template = doc.querySelector('#tmpl');

    assert_not_equals(template, null, 'Template element should not be null');
    assert_not_equals(template.content, undefined,
            'Content attribute of template element should not be undefined');
    assert_not_equals(template.content, null,
            'Content attribute of template element should not be null');

    assert_equals(template.ownerDocument, doc.body.ownerDocument,
            'Wrong template node owner document');
    var ownerDoc = template.content.ownerDocument;
    assert_not_equals(ownerDoc, doc, 'Wrong template content owner document');
    assert_not_equals(ownerDoc, document, 'Wrong template content owner document');
    assert_equals(ownerDoc.defaultView, null,
            'Template content owner document should not have a browsing context');

}, 'Parsing XHTML: Node\'s node document must be set to that of the element '
    + 'to which it will be appended. Test empty template');



test(function() {
    var doc = newXHTMLDocument();

    doc.body = doc.createElement('body');
    doc.body.innerHTML = '<template id="tmpl"><div>Div content</div></template>';

    var template = doc.querySelector('#tmpl');

    assert_equals(template.ownerDocument, doc.body.ownerDocument,
            'Wrong template node owner document');

    assert_not_equals(template, null, 'Template element should not be null');
    assert_not_equals(template.content, undefined,
            'Content attribute of template element should not be undefined');
    assert_not_equals(template.content, null,
            'Content attribute of template element should not be null');

    var div = template.content.querySelector('div');
    assert_equals(template.content.ownerDocument, div.ownerDocument,
            'Wrong DIV node owner document');

}, 'Parsing XHTML: Node\'s node document must be set to that of the element '
    + 'to which it will be appended. Test not empty template');



test(function() {
    var doc = newXHTMLDocument();
    doc.body = doc.createElement('body');
    doc.body.innerHTML = ''
            + '<template id="tmpl"><div>Div content</div> And some more text'
            + '<template id="tmpl2"><div>Template content</div></template>'
            + '</template>';

    var template = doc.querySelector('#tmpl');
    assert_not_equals(template, null, 'Template element should not be null');
    assert_equals(template.ownerDocument, doc, 'Wrong template node owner document');
    assert_not_equals(template.content, undefined,
            'Content attribute of template element should not be undefined');
    assert_not_equals(template.content, null,
            'Content attribute of template element should not be null');

    var nestedTemplate = template.content.querySelector('#tmpl2');
    assert_not_equals(nestedTemplate, null, 'Nested template element should not be null');
    assert_not_equals(nestedTemplate.content, undefined,
            'Content attribute of nested template element should not be undefined');
    assert_not_equals(nestedTemplate.content, null,
            'Content attribute of nested template element should not be null');

    assert_equals(nestedTemplate.ownerDocument, template.content.ownerDocument,
            'Wrong nested template node owner document');


    var div = nestedTemplate.content.querySelector('div');
    assert_equals(nestedTemplate.content.ownerDocument, div.ownerDocument,
            'Wrong DIV node owner document');

}, 'Parsing XHTML: Node\'s node document must be set to that of the element '
    + 'to which it will be appended. Test nested templates');



testInIFrame('../resources/template-child-nodes-div.xhtml', function(context) {
    var doc = context.iframes[0].contentDocument;

    var template = doc.querySelector('template');

    assert_equals(template.ownerDocument, doc, 'Wrong template node owner document');

    assert_not_equals(template.content, undefined,
            'Content attribute of template element should not be undefined');
    assert_not_equals(template.content, null,
            'Content attribute of template element should not be null');

    var div = template.content.querySelector('div');
    assert_equals(template.content.ownerDocument, div.ownerDocument,
            'Wrong DIV node owner document');

}, 'Parsing XHTML: Node\'s node document must be set to that of the element '
    + 'to which it will be appended. Test loading XHTML document from a file');



testInIFrame('../resources/template-child-nodes-nested.xhtml', function(context) {
    var doc = context.iframes[0].contentDocument;

    var template = doc.querySelector('template');

    assert_equals(template.ownerDocument, doc, 'Wrong template node owner document');

    var nestedTemplate = template.content.querySelector('template');

    assert_equals(nestedTemplate.ownerDocument, template.content.ownerDocument,
            'Wrong template node owner document');

    var div = nestedTemplate.content.querySelector('div');
    assert_equals(nestedTemplate.content.ownerDocument, div.ownerDocument,
            'Wrong DIV node owner document');

}, 'Parsing XHTML: Node\'s node document must be set to that of the element '
    + 'to which it will be appended. Test loading of XHTML document '
    + 'with nested templates from a file');

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
      "code": "html.attr.href.not_allowed",
      "message": "Attribute “href” not allowed on element “meta” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 259,
        "byte_start": 177,
        "col": 1,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.meta.missing_content",
      "message": "Element “meta” is missing one or more of the following attributes: “content”, “property”.",
      "severity": "Warning",
      "span": {
        "byte_end": 259,
        "byte_start": 177,
        "col": 1,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 717,
        "byte_start": 686,
        "col": 1,
        "line": 15
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
  "source_name": "html/semantics/scripting-1/the-template-element/additions-to-parsing-xhtml-documents/node-document.html"
}
```
