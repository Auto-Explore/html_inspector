# html/syntax/parsing/template/additions-to-the-in-body-insertion-mode/template-end-tag-without-start-one.html

Counts:
- errors: 0
- warnings: 6
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing/template/additions-to-the-in-body-insertion-mode/template-end-tag-without-start-one.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>HTML Templates: 'In body' insertion mode: Template end tag without start one. Element should be ignored</title>
<meta name="author" title="Sergey G. Grekhov" href="mailto:sgrekhov@unipro.ru">
<meta name="author" title="Aleksei Yu. Semenov" href="a.semenov@unipro.ru">
<meta name="assert" content="If parser in 'in body' insertion mode meets template end tag and if the stack of open elements has no template element in html scope, then this is a parse error; ignore the token">
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

    doc.body.innerHTML = '</template>';

    assert_equals(doc.body.childNodes.length, 0, 'Element must be ignored');

}, '</template> tag in HTML body without start one should be ignored');



test(function () {
    var doc = newHTMLDocument();

    doc.body.innerHTML = '<template id="tmpl"></template></template>';

    assert_equals(doc.body.childNodes.length, 1, 'Element must be ignored');
    assert_not_equals(doc.querySelector('#tmpl'), null,
            'Element should present it document body');

}, '</template> tag in HTML body without start one should be ignored. '
    + 'Test valid <template> element and </template> tag after it');



test(function () {
    var doc = newHTMLDocument();

    doc.body.innerHTML = '</template><template id="tmpl"></template>';

    assert_equals(doc.body.childNodes.length, 1, 'Element must be ignored');
    assert_not_equals(doc.querySelector('#tmpl'), null,
            'Element should present it document body');

}, '</template> tag in HTML body without start one should be ignored. '
    + 'Test valid <template> element and </template> tag before it');



test(function () {
    var doc = newHTMLDocument();

    doc.body.innerHTML = '</template><template id="tmpl"></template><title></title>';

    assert_equals(doc.body.childNodes.length, 2, 'Element must be ignored');
    assert_not_equals(doc.querySelector('#tmpl'), null,
            'Valid element should present it document body');
    assert_not_equals(doc.querySelector('title'), null,
            'Valid title element should present it document body');

}, '</template> tag in HTML body without start one should be ignored. '
    + 'Test valid <template> element, <title> element and </template> tag before them');



test(function () {
    var doc = newHTMLDocument();

    doc.body.innerHTML = '<template id="tmpl"></template><title></title></template>';

    assert_equals(doc.body.childNodes.length, 2, 'Element must be ignored');
    assert_not_equals(doc.querySelector('#tmpl'), null,
            'Valid element should present it document body');
    assert_not_equals(doc.querySelector('title'), null,
            'Valid title element should present it document body');

}, '</template> tag in HTML body without start one should be ignored. '
    + 'Test valid <template> element, <title> element and </template> tag after them');


testInIFrame('/html/semantics/scripting-1/the-template-element/resources/end-template-tag-in-body.html', function(context) {
    var doc = context.iframes[0].contentDocument;

    assert_equals(doc.body.querySelector('template'), null,
            '</template> must be ignored');
    assert_not_equals(doc.body.querySelector('div'), null,
            'Valid element should present it document body');

}, '</template> tag in HTML body without start one should be ignored. '
    + 'Test HTML document loaded from file');


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
        "byte_end": 228,
        "byte_start": 149,
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
        "byte_end": 228,
        "byte_start": 149,
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
        "byte_end": 304,
        "byte_start": 229,
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
        "byte_end": 304,
        "byte_start": 229,
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
        "byte_end": 834,
        "byte_start": 803,
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
  "source_name": "html/syntax/parsing/template/additions-to-the-in-body-insertion-mode/template-end-tag-without-start-one.html"
}
```
