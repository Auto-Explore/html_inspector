# html/semantics/scripting-1/the-template-element/template-element/node-document-changes.html

Counts:
- errors: 0
- warnings: 6
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-template-element/template-element/node-document-changes.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>HTML Templates: When node's document changes its owner document should be changed</title>
<meta name="author" title="Sergey G. Grekhov" href="mailto:sgrekhov@unipro.ru">
<meta name="author" title="Aleksei Yu. Semenov" href="mailto:a.semenov@unipro.ru">
<meta name="assert" content="When a template element's node document changes, the template element's content DocumentFragment must be adopted into the new node document's template contents owner document">
<link rel="help" href="http://www.w3.org/TR/2013/WD-html-templates-20130214/#template-element">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src='/html/resources/common.js'></script>
</head>
<body>
<div id="log"></div>
<script type="text/javascript">

test(function() {
    var doc1 = newHTMLDocument();
    var template = doc1.createElement('template');

    assert_equals(template.ownerDocument, doc1, 'Wrong template node owner document');
    assert_not_equals(template.content.ownerDocument, doc1,
            'Wrong template content owner document');

    var doc2 = newHTMLDocument();
    var template2 = doc2.createElement('template');
    doc2.body.appendChild(template);

    assert_equals(template.ownerDocument, template2.ownerDocument,
            'Template node owner document should be changed');
    assert_equals(template.content.ownerDocument, template2.content.ownerDocument,
            'Template content owner document should be changed');

}, 'Changing of template element\'s node document. ' +
    'Test that ownerDocument of an empty template and its content changes');

test(function() {
    var doc1 = newHTMLDocument();
    doc1.body.innerHTML = '<template id="tmpl"><div>Div content</div> And some more text</template>';

    var template = doc1.querySelector('#tmpl');

    assert_equals(template.ownerDocument, doc1,
            'Wrong template node owner document');
    assert_not_equals(template.content.ownerDocument, doc1,
            'Wrong template content owner document');

    var doc2 = newHTMLDocument();
    var template2 = doc2.createElement('template');
    doc2.body.appendChild(template);

    assert_equals(template.ownerDocument, template2.ownerDocument,
            'Template node owner document should be changed');
    assert_equals(template.content.ownerDocument, template2.content.ownerDocument,
            'Template content owner document should be changed');

    assert_equals(template.content.querySelector('div').ownerDocument,
            template2.content.ownerDocument,
            'Template content descendants owner document should be changed');

}, 'Changing of template element\'s node document. ' +
    'Test that ownerDocument of a not empty template and its content changes');

test(function() {
    var doc1 = newHTMLDocument();
    doc1.body.innerHTML = ''
            + '<template id="tmpl"><div>Div content</div> And some more text'
            + '<template id="tmpl2"><div>Template content</div></template>'
            + '</template>';

    var template = doc1.querySelector('#tmpl');

    assert_equals(template.ownerDocument, doc1, 'Wrong template node owner document');
    assert_not_equals(template.content.ownerDocument, doc1,
            'Wrong template content owner document');

    var nestedTemplate = template.content.querySelector('#tmpl2');

    assert_equals(nestedTemplate.ownerDocument, template.content.ownerDocument,
            'Wrong nested template node owner document');
    assert_equals(nestedTemplate.content.ownerDocument, template.content.ownerDocument,
            'Wrong nested template content owner document');

    var doc2 = newHTMLDocument();
    var template2 = doc2.createElement('template');
    doc2.body.appendChild(template);

    assert_equals(template.ownerDocument, template2.ownerDocument,
            'Template node owner document should be changed');
    assert_equals(template.content.ownerDocument, template2.content.ownerDocument,
            'Template content owner document should be changed');
    assert_equals(template.content.querySelector('div').ownerDocument,
            template2.content.ownerDocument,
            'Template content descendants owner document should be changed');

    assert_equals(nestedTemplate.ownerDocument,
            template2.content.ownerDocument,
            'Nested template node owner document should be changed');
    assert_equals(nestedTemplate.content.ownerDocument,
            template2.content.ownerDocument,
            'Nested template content owner document should be changed');
    assert_equals(nestedTemplate.content.querySelector('div').ownerDocument,
            template2.content.ownerDocument,
            'Owner document of the nested template content descendants should be changed');

}, 'Changing of template element\'s node document. ' +
    'Test that ownerDocument of nested template and its content changes');

testInIFrame('../resources/template-contents.html', function(context) {
    var doc1 = context.iframes[0].contentDocument;

    var template = doc1.body.querySelector('template');

    var doc2 = newHTMLDocument();
    var template2 = doc2.createElement('template');
    doc2.body.appendChild(template);

    assert_equals(template.ownerDocument, template2.ownerDocument,
            'Template node owner document should be changed');
    assert_equals(template.content.ownerDocument,
            template2.content.ownerDocument,
            'Template content owner document should be changed');
    assert_equals(template.content.querySelector('div').ownerDocument,
            template2.content.ownerDocument,
            'Template content descendants owner document should be changed');

}, 'Changing of template element\'s node document. ' +
    'Test document loaded from a file');

testInIFrame('../resources/template-contents.html', function(context) {
    var doc1 = context.iframes[0].contentDocument;

    var doc2 = newHTMLDocument();
    var template = doc2.createElement('template');
    var div = doc2.createElement('div');
    template.content.appendChild(div);

    doc1.body.appendChild(template);

    assert_not_equals(template.ownerDocument, doc2,
            'Template node owner document should be changed');
    assert_not_equals(template.content.ownerDocument, doc2,
            'Template content owner document should be changed');
    assert_not_equals(div.ownerDocument, doc2,
            'Template content descendants owner document should be changed');

    assert_equals(template.ownerDocument, doc1,
            'Template node owner document should be changed');
    // doc1 has browsing context so it cannot be template.content.ownerDocument
    assert_not_equals(template.content.ownerDocument, doc1,
            'Template content owner document should be a new document');
    assert_equals(div.ownerDocument, template.content.ownerDocument,
            'Template content descendants owner document should be ' +
            'template content document owner');

}, 'Changing of template element\'s node document. ' +
    'Adobt template element into a document that has a browsing context');

testInIFrame('../resources/template-contents.html', function(context) {
    var doc1 = context.iframes[0].contentDocument;

    var template = doc1.querySelector('template');
    var div = template.content.querySelector('div');
    var templateContentOwner = template.content.ownerDocument;

    var doc2 = document;

    doc2.body.appendChild(template);

    assert_not_equals(template.ownerDocument, doc1,
            'Template node owner document should be changed');
    assert_not_equals(template.content.ownerDocument, templateContentOwner,
            'Template content owner document should be changed');
    assert_not_equals(div.ownerDocument, templateContentOwner,
            'Template content descendants owner document should be changed');

    assert_equals(template.ownerDocument, doc2,
            'Template node owner document should be changed');
    // doc2 has browsing context, so it cannot be template.content.ownerDocument
    assert_not_equals(template.content.ownerDocument, doc2,
            'Template content owner document should be a new document');
    assert_equals(div.ownerDocument, template.content.ownerDocument,
            'Template content descendants owner document should be ' +
            'template content document owner');

}, 'Changing of template element\'s node document. ' +
    'Test the case when both old and new owner documents of template element ' +
    'have browsing context');

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
        "byte_end": 206,
        "byte_start": 127,
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
        "byte_end": 206,
        "byte_start": 127,
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
        "byte_end": 289,
        "byte_start": 207,
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
        "byte_end": 289,
        "byte_start": 207,
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
        "byte_end": 815,
        "byte_start": 784,
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
  "source_name": "html/semantics/scripting-1/the-template-element/template-element/node-document-changes.html"
}
```
