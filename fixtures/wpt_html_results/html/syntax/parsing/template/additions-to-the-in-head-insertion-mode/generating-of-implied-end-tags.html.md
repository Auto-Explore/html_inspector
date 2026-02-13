# html/syntax/parsing/template/additions-to-the-in-head-insertion-mode/generating-of-implied-end-tags.html

Counts:
- errors: 0
- warnings: 6
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing/template/additions-to-the-in-head-insertion-mode/generating-of-implied-end-tags.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>HTML Templates: When template end tag is met, implied end tags should be generated</title>
<meta name="author" title="Sergey G. Grekhov" href="mailto:sgrekhov@unipro.ru">
<meta name="author" title="Aleksei Yu. Semenov" href="a.semenov@unipro.ru">
<meta name="assert" content="When template end tag is met, implied end tags should be generated">
<link rel="help" href="http://www.w3.org/TR/2013/WD-html-templates-20130214/#in-head-addition">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/html/resources/common.js"></script>
</head>
<body>
<div id="log"></div>
<script type="text/javascript">


test(function () {
    var doc = newHTMLDocument();

    //No end </td></tr></table> tags. Should be added implicitly
    doc.head.innerHTML = '<template id="tpl">'
        + '<table id="tbl"><tr id="tr"><td id="td"></template>';

    var template = doc.querySelector('#tpl');

    assert_not_equals(template, null, 'Template element must be parsed');

    assert_equals(doc.querySelector('#tbl'), null, 'Table element should not be available');
    assert_equals(doc.querySelector('#tr'), null, 'TR element should not be available');
    assert_equals(doc.querySelector('#td'), null, 'TD element should not be available');

    assert_not_equals(template.content.querySelector('#tbl'), null,
            'Template should contain table element');
    assert_not_equals(template.content.querySelector('#tr'), null,
            'Template should contain TR element');
    assert_not_equals(template.content.querySelector('#td'), null,
            'Template should contain TD element');

}, 'Generating of implied end tags. Test table elements');



test(function () {
    var doc = newHTMLDocument();

    //No end </div> tag. Should be added implicitly
    doc.head.innerHTML = '<template id="tpl"><div id="dv">Div content</template>';

    var template = doc.querySelector('#tpl');

    assert_not_equals(template, null, 'Template element must be parsed');

    assert_equals(doc.querySelector('#dv'), null, 'DIV element should not be available');

    assert_not_equals(template.content.querySelector('#dv'), null,
            'Template should contain DIV element');

}, 'Generating of implied end tags. Test DIV element');


test(function () {
    var doc = newHTMLDocument();

    //No end </div> tag. Should be added implicitly after text content
    doc.head.innerHTML = '<template id="tpl">Template text<div id="dv">Div content</template>';

    var template = doc.querySelector('#tpl');

    assert_not_equals(template, null, 'Template element must be parsed');

    assert_equals(doc.querySelector('#dv'), null, 'DIV element should not be available');

    var div = template.content.querySelector('#dv');

    assert_not_equals( div, null, 'Template should contain DIV element');
    assert_equals(div.textContent, 'Div content', 'Wrong template content inner text');

}, 'Generating of implied end tags. Test some text and DIV element');


test(function () {
    var doc = newHTMLDocument();

    // Wrong end tag. Correct end tag must be added implicitly, wrong one ignored
    doc.head.innerHTML = '<template id="tpl"><div id="dv">Div content</span></template>';

    var template = doc.querySelector('#tpl');

    assert_not_equals(template, null, 'Template element must be parsed');

    assert_equals(template.content.childNodes.length, 1,
            'Wrong number of template\'s children');

    assert_equals(doc.querySelector('#dv'), null, 'DIV element should not be available');

    assert_not_equals(template.content.querySelector('#dv'), null,
            'Template should contain DIV element');
    assert_equals(template.content.querySelector('#dv').textContent,
            'Div content', 'Wrong template content inner text');

}, 'Generating of implied end tags. Test wrong end tag');


testInIFrame('/html/semantics/scripting-1/the-template-element/resources/head-template-contents-table-no-end-tag.html', function(context) {
    var doc = context.iframes[0].contentDocument;

    var template = doc.head.querySelector('template');

    assert_not_equals(template, null,
            'Template element must be parsed');

    assert_not_equals(template.content.querySelector('table'), null,
            'Template should contain table element');
    assert_not_equals(template.content.querySelector('tr'), null,
            'Template should contain TR element');
    assert_not_equals(template.content.querySelector('td'), null,
            'Template should contain TD element');

}, 'Generating of implied end tags. Test table elements. Load HTML document from file');


testInIFrame('/html/semantics/scripting-1/the-template-element/resources/head-template-contents-div-no-end-tag.html', function(context) {
    var doc = context.iframes[0].contentDocument;

    var template = doc.head.querySelector('template');

    assert_not_equals(template, null, 'Template element must be parsed');

    var div = template.content.querySelector('div');
    assert_not_equals(div, null, 'Template should contain div element');
    assert_equals(div.textContent, 'Hello, template\n    ', 'Invalid div contents');

}, 'Generating of implied end tags. Test div element. Load HTML document from file');
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
        "byte_end": 283,
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
        "byte_end": 283,
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
        "byte_end": 701,
        "byte_start": 670,
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
  "source_name": "html/syntax/parsing/template/additions-to-the-in-head-insertion-mode/generating-of-implied-end-tags.html"
}
```
