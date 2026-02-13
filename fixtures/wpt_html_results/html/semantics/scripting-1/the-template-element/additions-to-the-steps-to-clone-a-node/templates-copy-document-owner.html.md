# html/semantics/scripting-1/the-template-element/additions-to-the-steps-to-clone-a-node/templates-copy-document-owner.html

Counts:
- errors: 0
- warnings: 6
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-template-element/additions-to-the-steps-to-clone-a-node/templates-copy-document-owner.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>HTML Templates: ownerDocument of cloned template content is set to template content owner</title>
<meta name="author" title="Sergey G. Grekhov" href="mailto:sgrekhov@unipro.ru">
<meta name="author" title="Aleksei Yu. Semenov" href="a.semenov@unipro.ru">
<meta name="assert" content="ownerDocument of cloned template content is set to template content owner">
<link rel="help" href="http://www.w3.org/TR/2013/WD-html-templates-20130214/#node-clone-additions">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src='/html/resources/common.js'></script>
</head>
<body>
<div id="log"></div>
<script type="text/javascript">

function checkOwnerDocument(node, doc) {
    if ((node !== null) && (node !== undefined)) {
        assert_equals(node.ownerDocument, doc,
                'Wrong ownerDocument of the template copy\'s node ' + node.nodeName);
        for (var i = 0; i < node.childNodes.length; i++) {
            if (node.childNodes[i].nodeType === Node.ELEMENT_NODE) {
                checkOwnerDocument(node.childNodes[i], doc);
                if (node.childNodes[i].nodeName === 'TEMPLATE') {
                    checkOwnerDocument(node.childNodes[i].content, doc);
                }
            }
        }
    }
}


test(function () {
    var doc = newHTMLDocument();
    doc.body.innerHTML = '<template id="tmpl1">' +
        '<div id="div1">This is div inside template</div>' +
        '<div id="div2">This is another div inside template</div>' +
        '</template>';

    var template = doc.querySelector('#tmpl1');
    var copy = template.cloneNode(true);

    assert_equals(copy.content.childNodes.length, 2,
            'Wrong number of template content\'s copy child nodes');
    checkOwnerDocument(copy.content, template.content.ownerDocument);

}, 'ownerDocument of cloned template content is set to template content owner. '
       + 'Test cloning with children');



test(function () {
    var doc = newHTMLDocument();
    doc.body.innerHTML = '<template id="tmpl1">' +
        '<div id="div1">This is div inside template</div>' +
        '<div id="div2">This is another div inside template</div>' +
        '</template>';

    var template = doc.querySelector('#tmpl1');
    var copy = template.cloneNode(false);

    assert_equals(copy.content.childNodes.length, 0,
            'Wrong number of template content\'s copy child nodes');
    checkOwnerDocument(copy.content, template.content.ownerDocument);

}, 'ownerDocument of cloned template content is set to template content owner. '
      + 'Test cloning without children');



test(function () {
    var doc = newHTMLDocument();
    doc.body.innerHTML = '<template id="tmpl1">' +
        '<div id="div1">This is div inside template</div>' +
        '<div id="div2">This is another div inside template</div>' +
        '</template>';

    var template = doc.querySelector('#tmpl1');
    var copy = template.cloneNode();

    assert_equals(copy.content.childNodes.length, 0,
            'Wrong number of template content\'s copy child nodes');
    checkOwnerDocument(copy.content, template.content.ownerDocument);

}, 'ownerDocument of cloned template content is set to template content owner. '
      + 'Test cloneNode() with no arguments (false by default)');



test(function () {
    var doc = newHTMLDocument();
    doc.body.innerHTML = '<template id="tmpl1">' +
        '<div id="div1">This is div inside template</div>' +
        '<div id="div2">This is another div inside template</div>' +
        '<template id="tmpl2">' +
        '<div id="div3">This is div inside nested template</div>' +
        '<div id="div4">This is another div inside nested template</div>' +
        '</template>' +
        '</template>';

    var template = doc.querySelector('#tmpl1');
    var copy = template.cloneNode(true);

    assert_equals(copy.content.childNodes.length, 3,
            'Wrong number of template content\'s copy child nodes');
    checkOwnerDocument(copy.content, template.content.ownerDocument);

}, 'ownerDocument of cloned template content is set to template content owner. '
    + 'Test cloning nested template');



testInIFrame('../resources/template-contents.html', function(context) {
    var doc = context.iframes[0].contentDocument;

    var template = doc.body.querySelector('template');
    var copy = template.cloneNode(true);

    checkOwnerDocument(copy.content, template.content.ownerDocument);

}, 'ownerDocument of cloned template content is set to template content owner. '
   + 'Test loading HTML document from file');

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
        "byte_end": 214,
        "byte_start": 135,
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
        "byte_end": 214,
        "byte_start": 135,
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
        "byte_end": 290,
        "byte_start": 215,
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
        "byte_end": 290,
        "byte_start": 215,
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
        "byte_end": 719,
        "byte_start": 688,
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
  "source_name": "html/semantics/scripting-1/the-template-element/additions-to-the-steps-to-clone-a-node/templates-copy-document-owner.html"
}
```
