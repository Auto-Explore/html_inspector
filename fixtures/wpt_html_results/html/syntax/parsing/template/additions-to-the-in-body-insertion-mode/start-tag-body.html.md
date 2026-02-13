# html/syntax/parsing/template/additions-to-the-in-body-insertion-mode/start-tag-body.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing/template/additions-to-the-in-body-insertion-mode/start-tag-body.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>HTML Templates: In body insertion mode: Template contains a start tag whose tag name is body</title>
<meta name="author" title="Sergey G. Grekhov" href="mailto:sgrekhov@unipro.ru">
<meta name="assert" content="If the stack of open elements has a template element in html scope then ignore <body> the token. (fragment or template contents case)">
<link rel="help" href="http://www.w3.org/TR/2013/WD-html-templates-20130214/#in-body-addition">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/html/resources/common.js"></script>
</head>
<body>
<div id="log"></div>
<script type="text/javascript">


test(function () {
    var doc = newHTMLDocument();

    doc.body.innerHTML = '<template id="tmpl"><body></template>';

    var template = doc.querySelector('#tmpl');

    assert_equals(template.content.childNodes.length, 0, 'Element must be ignored');

}, 'In body insertion mode: Template contains a start tag whose tag name is body.'
    + 'Test <body> tag only');



test(function () {
    var doc = newHTMLDocument();

    doc.body.innerHTML = '<template id="tmpl"><body>Body text content</body></template>';

    var template = doc.querySelector('#tmpl');

    assert_equals(template.content.querySelector('body'), null,
            '<body> element must be ignored');
    assert_equals(template.content.childNodes.length, 1, 'Text shouldn\'t be ignored');
    assert_equals(template.content.firstChild.nodeType, Node.TEXT_NODE,
            'Text shouldn\'t be ignored');

}, 'In body insertion mode: Template contains a start tag whose tag name is body. '
    + 'Test <body> tag containing some text');



test(function () {
    var doc = newHTMLDocument();

    doc.body.innerHTML = '<template id="tmpl"><body>'
        + '<div id="div1">DIV 1</div>'
        + '<div id="div2">DIV 2</div>'
        + '</body></template>';

    var template = doc.querySelector('#tmpl');

    assert_equals(template.content.querySelector('body'), null,
            '<body> element must be ignored');
    assert_equals(template.content.childNodes.length, 2,
            'Only body tag should be ignored');
    assert_not_equals(template.content.querySelector('#div1'), null,
            'Children of <body tag shouldn\'t be ignored');
    assert_not_equals(template.content.querySelector('#div2'), null,
            'Children of <body tag shouldn\'t be ignored');

}, 'In body insertion mode: Template contains a start tag whose tag name is body. '
    + 'Test <body> tag containing some other elements');



test(function () {
    var doc = newHTMLDocument();

    doc.body.innerHTML = '<template id="tmpl1"><template id="tmpl2"><body>'
        + '<div id="div1">DIV 1</div>'
        + '<div id="div2">DIV 2</div>'
        + '</body></template></template>';

    var template = doc.querySelector('#tmpl1').content.querySelector('#tmpl2');

    assert_equals(template.content.querySelector('body'), null,
            '<body> element must be ignored');
    assert_equals(template.content.childNodes.length, 2,
            'Only body tag should be ignored');
    assert_not_equals(template.content.querySelector('#div1'), null,
            'Children of <body tag shouldn\'t be ignored');
    assert_not_equals(template.content.querySelector('#div2'), null,
            'Children of <body tag shouldn\'t be ignored');

}, 'In body insertion mode: Template contains a start tag whose tag name is body. '
    + 'Test nested template tag containing <body> tag with some other elements');

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
        "byte_end": 217,
        "byte_start": 138,
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
        "byte_end": 217,
        "byte_start": 138,
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
        "byte_end": 702,
        "byte_start": 671,
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
  "source_name": "html/syntax/parsing/template/additions-to-the-in-body-insertion-mode/start-tag-body.html"
}
```
