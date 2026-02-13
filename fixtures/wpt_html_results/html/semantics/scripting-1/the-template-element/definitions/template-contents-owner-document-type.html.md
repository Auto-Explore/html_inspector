# html/semantics/scripting-1/the-template-element/definitions/template-contents-owner-document-type.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-template-element/definitions/template-contents-owner-document-type.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>HTML Templates: The template contents owner document type is HTML document</title>
<meta name="author" title="Sergey G. Grekhov" href="mailto:sgrekhov@unipro.ru">
<meta name="assert" content="The template contents owner document type is HTML document, if template is declared in HTML document">
<link rel="help" href="http://www.w3.org/TR/2013/WD-html-templates-20130214/#definitions">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src='/html/resources/common.js'></script>
</head>
<body>
<div id="log"></div>
<script type="text/javascript">

testInIFrame('../resources/template-contents.html', function(context) {
    var doc = context.iframes[0].contentDocument;
    var template = doc.querySelector('template');
    var content_owner = template.content.ownerDocument;

    assert_class_string(content_owner, 'Document',
            'Template content owner should be a document');
    assert_equals(content_owner.createElement('DIV').localName, 'div',
            'Template content owner should be an HTML document');

}, 'The template contents owner document type is HTML document ' +
    '(case when document has browsing context and the template ' +
    'is created by HTML parser)');


testInIFrame('../resources/template-contents.html', function(context) {
    var doc = context.iframes[0].contentDocument;
    var template = doc.createElement('template');
    var content_owner = template.content.ownerDocument;
    var div = doc.createElement('DIV');
    template.appendChild(div);

    doc.body.appendChild(template);

    assert_class_string(content_owner, 'Document',
            'Template content owner should be a document');
    assert_equals(div.localName, 'div',
            'Template content owner should be an HTML document');

}, 'The template contents owner document type is HTML document ' +
    '(case when document has browsing context and the template ' +
    'is created by createElement())');


test(function() {
    var doc = newHTMLDocument();
    var template = doc.createElement('template');
    var content_owner = template.content.ownerDocument;
    var div = doc.createElement('DIV');
    template.appendChild(div);

    doc.body.appendChild(template);

    assert_class_string(content_owner, 'Document',
            'Template content owner should be a document');
    assert_equals(div.localName, 'div',
            'Template content owner should be an HTML document');

}, 'The template contents owner document type is HTML document ' +
    '(case when document has no browsing context and the template is created ' +
    'by createElement())');

test(function() {
    var doc = newHTMLDocument();
    doc.body.innerHTML = '<template><div>Hello!</div></template>';
    var template = doc.querySelector('template');
    var content_owner = template.content.ownerDocument;

    assert_class_string(content_owner, 'Document',
            'Template content owner should be a document');
    assert_equals(content_owner.createElement('DIV').localName, 'div',
            'Template content owner should be an HTML document');

}, 'The template contents owner document type is HTML document ' +
    '(case when document has no browsing context and the template is created via innerHTML)');
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
        "byte_end": 199,
        "byte_start": 120,
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
        "byte_end": 199,
        "byte_start": 120,
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
        "byte_end": 646,
        "byte_start": 615,
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
  "source_name": "html/semantics/scripting-1/the-template-element/definitions/template-contents-owner-document-type.html"
}
```
