# html/semantics/scripting-1/the-template-element/template-element/template-content-node-document.html

Counts:
- errors: 0
- warnings: 6
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-template-element/template-element/template-content-node-document.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>HTML Templates: Node document of the template content attribute must be template contents owner</title>
<meta name="author" title="Sergey G. Grekhov" href="mailto:sgrekhov@unipro.ru">
<meta name="author" title="Aleksei Yu. Semenov" href="mailto:a.semenov@unipro.ru">
<meta name="assert" content="Node document of the template content attribute must be template contents owner">
<link rel="help" href="http://www.w3.org/TR/2013/WD-html-templates-20130214/#template-element">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src='/html/resources/common.js'></script>
</head>
<body>
<div id="log"></div>
<script type="text/javascript">

test(function() {
    var doc = newHTMLDocument();
    var template = doc.createElement('template');
    var nestedTemplate = doc.createElement('template');
    template.appendChild(nestedTemplate);

    assert_equals(nestedTemplate.content.ownerDocument, template.content.ownerDocument,
            'Wrong node document of the template content attribute');

}, 'Node document of the template content attribute must be template contents owner. ' +
    'Nested template element created by createElement');


test(function() {
    var doc = newHTMLDocument();
    doc.body.innerHTML = '<template><template></template></template>';
    var template = doc.querySelector('template');
    var nestedTemplate = template.content.querySelector('template');

    assert_equals(nestedTemplate.content.ownerDocument, template.content.ownerDocument,
            'Wrong node document of the template content attribute');

}, 'Node document of the template content attribute must be template contents owner. ' +
    'Nested template element created by innerHTML');

testInIFrame('../resources/two-templates.html', function(context) {
    var doc = context.iframes[0].contentDocument;

    var template1 = doc.querySelector('#template1');
    var template2 = doc.querySelector('#template2');

    // when there is a browsing context, template contents owner is only accessible via template.content.ownerDocument
    // because template contents owner is bounded to document
    // verify that multiple templates share the same instance of template contents owner

    assert_equals(template1.content.ownerDocument, template2.content.ownerDocument,
            'Wrong node document of the template content attribute');
}, 'Node document of the template content attribute must be template contents owner. ' +
    'Load HTML file with multiple template elements');

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
        "byte_end": 220,
        "byte_start": 141,
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
        "byte_end": 220,
        "byte_start": 141,
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
        "byte_end": 303,
        "byte_start": 221,
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
        "byte_end": 303,
        "byte_start": 221,
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
        "byte_end": 734,
        "byte_start": 703,
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
  "source_name": "html/semantics/scripting-1/the-template-element/template-element/template-content-node-document.html"
}
```
