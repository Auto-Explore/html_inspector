# html/semantics/scripting-1/the-template-element/additions-to-parsing-xhtml-documents/template-child-nodes.html

Counts:
- errors: 0
- warnings: 6
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-template-element/additions-to-parsing-xhtml-documents/template-child-nodes.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>HTML Templates: Child nodes of template element in XHTML documents</title>
<meta name="author" title="Sergey G. Grekhov" href="mailto:sgrekhov@unipro.ru">
<meta name="author" title="Aleksei Yu. Semenov" href="a.semenov@unipro.ru">
<meta name="assert" content="Child nodes of template element in XHTML documents are always appended to the template content (instead of template itself)">
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
    doc.body.innerHTML = '<template id="tmpl1">'
            + '<div id="div1">This is div inside template</div>'
            + '<div id="div2">This is another div inside template</div>'
            + '</template>';

    var template = doc.querySelector('#tmpl1');

    assert_equals(template.childNodes.length, 0,
            'Wrong number of template child nodes');
    assert_equals(template.content.childNodes.length, 2,
            'Wrong number of template content child nodes');

}, 'Child nodes of template element in XHTML documents must be appended to template content');



test(function() {
    var doc = newXHTMLDocument();
    doc.body = doc.createElement('body');
    doc.body.innerHTML = '<template id="tmpl1">'
            + '<div id="div1">This is div inside template</div>'
            + '<div id="div2">This is another div inside template</div>'
            + '<template id="tmpl2">'
            + '<div id="div3">This is div inside nested template</div>'
            + '<div id="div4">This is another div inside nested template</div>'
            + '</template>' + '</template>';

    var template = doc.querySelector('#tmpl1');

    assert_equals(template.childNodes.length, 0,
            'Wrong number of template child nodes');
    assert_equals(template.content.childNodes.length, 3,
            'Wrong number of template content child nodes');

    var nestedTemplate = template.content.querySelector('#tmpl2');

    assert_equals(nestedTemplate.childNodes.length, 0,
            'Wrong number of template child nodes');
    assert_equals(nestedTemplate.content.childNodes.length, 2,
            'Wrong number of nested template content child nodes');

}, 'Child nodes of nested template element in XHTML documents must be appended to template content');



testInIFrame('../resources/template-child-nodes-div.xhtml', function(context) {
    var doc = context.iframes[0].contentDocument;

    var template = doc.querySelector('template');

    assert_equals(template.childNodes.length, 0,
            'Wrong number of template child nodes');
    assert_equals(template.content.querySelectorAll('div').length, 2,
            'Wrong number of template content child nodes');

}, 'Child nodes of template element in XHTML documents must be appended to template content. '
    + 'Test loading XHTML document from a file');


testInIFrame('../resources/template-child-nodes-nested.xhtml', function(context) {
    var doc = context.iframes[0].contentDocument;

    var template = doc.querySelector('template');

    assert_equals(template.childNodes.length, 0,
            'Wrong number of template child nodes');

    var nestedTemplate = template.content.querySelector('template');

    assert_equals(nestedTemplate.childNodes.length, 0,
            'Wrong number of template child nodes');

    assert_equals(nestedTemplate.content.querySelectorAll('div').length, 2,
            'Wrong number of template content child nodes');

}, 'Child nodes of nested template element in XHTML documents must be appended to template content. '
    + 'Test loading XHTML document from a file');

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
        "byte_end": 191,
        "byte_start": 112,
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
        "byte_end": 191,
        "byte_start": 112,
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
        "byte_end": 267,
        "byte_start": 192,
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
        "byte_end": 267,
        "byte_start": 192,
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
        "byte_end": 749,
        "byte_start": 718,
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
  "source_name": "html/semantics/scripting-1/the-template-element/additions-to-parsing-xhtml-documents/template-child-nodes.html"
}
```
