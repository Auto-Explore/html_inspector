# html/syntax/parsing/template/appending-to-a-template/template-child-nodes.html

Counts:
- errors: 0
- warnings: 6
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing/template/appending-to-a-template/template-child-nodes.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>HTML Templates: HTML parser appends child nodes only to the template contents node</title>
<meta name="author" title="Sergey G. Grekhov" href="mailto:sgrekhov@unipro.ru">
<meta name="author" title="Aleksei Yu. Semenov" href="mailto:a.semenov@unipro.ru">
<meta name="assert" content="HTML parser must append template's child nodes only to the template contents node.">
<link rel="help" href="http://www.w3.org/TR/2013/WD-html-templates-20130214/#appending-to-a-template">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/html/resources/common.js"></script>
</head>
<body>
<div id="log"></div>
<script type="text/javascript">


test(function () {
    var doc = newHTMLDocument();
    doc.body.innerHTML = '<template id="tmpl1">' +
        '<div id="div1">This is div inside template</div>' +
        '<div id="div2">This is another div inside template</div>' +
        '</template>';

    var template = doc.querySelector('#tmpl1');

    assert_equals(template.childNodes.length, 0, 'Wrong number of template child nodes');
    assert_equals(template.content.childNodes.length, 2,
            'Wrong number of template content child nodes');

    assert_not_equals(template.content.querySelector('#div1'), null,
            'Element is absent in the template content');
    assert_not_equals(template.content.querySelector('#div2'), null,
            'Element is absent in the template content');

}, 'Template child nodes must be appended to template content node');



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

    assert_equals(template.childNodes.length, 0,
            'Wrong number of template child nodes');
    assert_equals(template.content.childNodes.length, 3,
            'Wrong number of template content child nodes');

    assert_not_equals(template.content.querySelector('#div1'), null,
            'Element is absent in the template content');
    assert_not_equals(template.content.querySelector('#div2'), null,
            'Element is absent in the template content');

    var nestedTemplate = template.content.querySelector('#tmpl2');

    assert_equals(nestedTemplate.childNodes.length, 0,
            'Wrong number of template child nodes');
    assert_equals(nestedTemplate.content.childNodes.length, 2,
            'Wrong number of nested template content child nodes');

    assert_not_equals(nestedTemplate.content.querySelector('#div3'), null,
            'Element is absent in the template content');
    assert_not_equals(nestedTemplate.content.querySelector('#div4'), null,
            'Element is absent in the template content');

}, 'Template child nodes must be appended to template content. Test nested template');



testInIFrame('/html/semantics/scripting-1/the-template-element/resources/template-contents.html', function(context) {
    var doc = context.iframes[0].contentDocument;

    var template = doc.querySelector('template');

    assert_equals(template.childNodes.length, 0, 'Wrong number of template child nodes');

    assert_not_equals(template.content.querySelector('div'), null,
            'Element is absent in the template content');

}, 'Template child nodes must be appended to template content node. '
    + 'Load HTML document from a file');


testInIFrame('/html/semantics/scripting-1/the-template-element/resources/template-contents-nested.html', function(context) {
    var doc = context.iframes[0].contentDocument;

    var template = doc.querySelector('template');

    assert_equals(template.childNodes.length, 0, 'Wrong number of template child nodes');

    var nestedTemplate = template.content.querySelector('template');

    assert_not_equals(nestedTemplate, null,
            'Element is absent in the template content');

    assert_equals(nestedTemplate.childNodes.length, 0,
            'Wrong number of template child nodes');

    assert_not_equals(nestedTemplate.content.querySelector('div'), null,
            'Element is absent in the template content');

}, 'Template child nodes must be appended to nested template content node. '
    + 'Load HTML document from a file');

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
        "byte_end": 207,
        "byte_start": 128,
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
        "byte_end": 207,
        "byte_start": 128,
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
        "byte_start": 208,
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
        "byte_start": 208,
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
        "byte_end": 731,
        "byte_start": 700,
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
  "source_name": "html/syntax/parsing/template/appending-to-a-template/template-child-nodes.html"
}
```
