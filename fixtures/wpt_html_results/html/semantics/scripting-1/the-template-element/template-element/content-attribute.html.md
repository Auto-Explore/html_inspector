# html/semantics/scripting-1/the-template-element/template-element/content-attribute.html

Counts:
- errors: 0
- warnings: 6
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-template-element/template-element/content-attribute.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>HTML Templates: Content attribute of template element is read-only</title>
<meta name="author" title="Sergey G. Grekhov" href="mailto:sgrekhov@unipro.ru">
<meta name="author" title="Aleksei Yu. Semenov" href="mailto:a.semenov@unipro.ru">
<meta name="assert" content="Content attribute of template element is read-only">
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

    assert_readonly(template, 'content',
            'Content attribute of template element should be read-only');

}, 'Content attribute of template element is read-only. ' +
    'Test empty template');


test(function() {
    var doc = newHTMLDocument();
    var template = doc.createElement('template');
    var el1 = doc.createElement('div');
    var el2 = doc.createElement('span');
    el1.appendChild(el2);

    template.content.appendChild(el1);

    assert_readonly(template, 'content',
            'Content attribute of template element should be read-only');

}, 'Content attribute of template element is read-only. ' +
    'Test not empty template populated by appendchild()');


test(function() {
    var doc = newHTMLDocument();
    doc.body.innerHTML = '<template>Text<div>DIV</div></template>';

    var template = doc.querySelector('template');

    assert_readonly(template, 'content',
            'Content attribute of template element should be read-only');

}, 'Content attribute of template element is read-only. ' +
    'Test not empty template populated by innerHTML');


test(function() {
    var doc = newHTMLDocument();
    doc.body.innerHTML = '<template id="template1" content="Some text as a content"></template>';

    var template = doc.querySelector('#template1');

    assert_readonly(template, 'content',
            'Content attribute of template element should be read-only');

}, 'Content attribute of template element is read-only. ' +
    'Test that custom content attribute named \'content\' doesn\'t ' +
    'make content IDL attribute writable');


test(function() {
    var doc = newHTMLDocument();
    doc.body.innerHTML = '<template id="template1" content="<div id=div1>Div content</div>"></template>';

    var template = doc.querySelector('#template1');

    assert_readonly(template, 'content',
            'Content attribute of template element should be read-only');

    assert_equals(template.content.childNodes.length, 0,
            'Content attribute of template element should be read-only');

}, 'Content attribute of template element is read-only. ' +
    'Test that custom content attribute named \'content\' doesn\'t ' +
    'affect content IDL attribute');


testInIFrame('../resources/template-contents-attribute.html', function(context) {
    var doc = context.iframes[0].contentDocument;

    var template = doc.body.querySelector('template');

    assert_readonly(template, 'content',
            'Content attribute of template element should be read-only');

}, 'Content attribute of template element is read-only. '
    + 'Text value of content attribute of template tag should be ignored, '
    + 'when loading document from a file');


testInIFrame('../resources/template-contents.html', function(context) {
    var doc = context.iframes[0].contentDocument;

    var template = doc.body.querySelector('template');

    assert_readonly(template, 'content',
            'Content attribute of template element should be read-only');

}, 'Content attribute of template element is read-only. '
    + 'Test content attribute of a document loaded from a file');

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
        "byte_end": 274,
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
        "byte_end": 274,
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
        "byte_end": 676,
        "byte_start": 645,
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
  "source_name": "html/semantics/scripting-1/the-template-element/template-element/content-attribute.html"
}
```
