# html/semantics/scripting-1/the-template-element/innerhtml-on-templates/innerhtml.html

Counts:
- errors: 0
- warnings: 6
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-template-element/innerhtml-on-templates/innerhtml.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>HTML Templates: innerHTML of template element replaces all referenced by the content attribute</title>
<meta name="author" title="Sergey G. Grekhov" href="mailto:sgrekhov@unipro.ru">
<meta name="author" title="Aleksei Yu. Semenov" href="a.semenov@unipro.ru">
<meta name="assert" content="innerHTML of template element replaces all referenced by the content attribute">
<link rel="help" href="http://www.w3.org/TR/2013/WD-html-templates-20130214/#innerhtml-on-templates">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src='/html/resources/common.js'></script>
</head>
<body>
<div id="log"></div>
<script type="text/javascript">

test(function () {
    var doc = newHTMLDocument();
    var template = doc.createElement('template');

    var div1 = doc.createElement('div');
    div1.setAttribute('id', 'div1');
    template.content.appendChild(div1);

    assert_not_equals(template.content.querySelector('#div1'), null,
            'Element should present in template content');

    template.innerHTML = '<div id="div2"></div>';

    assert_equals(template.content.querySelector('#div1'), null,
            'Template content should be replaced by innerHTML');
    assert_not_equals(template.content.querySelector('#div2'), null,
            'Element should present in template content');

}, 'innerHTML of template element replaces all referenced by the content attribute');


test(function () {
    var doc = newHTMLDocument();
    var template = doc.createElement('template');

    var div1 = doc.createElement('div');
    div1.setAttribute('id', 'div1');
    template.content.appendChild(div1);

    assert_not_equals(template.content.querySelector('#div1'), null,
            'Element should present in template content');

    template.innerHTML = '';

    assert_false(template.content.hasChildNodes(),
            'Template content should be removed by innerHTML');

}, 'innerHTML of template element replaces all referenced by the content attribute. '
    + 'Test empty HTML string');


test(function () {
    var doc = newHTMLDocument();
    var template = doc.createElement('template');
    var nestedTemplate = doc.createElement('template');

    template.content.appendChild(nestedTemplate);

    var div1 = doc.createElement('div');
    div1.setAttribute('id', 'div1');
    nestedTemplate.content.appendChild(div1);

    assert_not_equals(nestedTemplate.content.querySelector('#div1'), null,
            'Element should present in template content');

    nestedTemplate.innerHTML = '<div id="div2"></div>';

    assert_equals(nestedTemplate.content.querySelector('#div1'), null,
            'Template content should be replaced by innerHTML');
    assert_not_equals(nestedTemplate.content.querySelector('#div2'), null,
            'Element should present in template content');

}, 'innerHTML of template element replaces all referenced by the content attribute. '
    + 'Test nested template');


testInIFrame('../resources/template-contents.html', function(context) {
    var doc = context.iframes[0].contentDocument;

    var template = doc.querySelector('template');
    assert_not_equals(template.content.querySelector('div'), null,
           'Div element should present in template content');

    template.innerHTML = '<span>span internals</span>';

    assert_equals(template.content.querySelector('div'), null,
           'div element should be replaced by span in template content');

    assert_not_equals(template.content.querySelector('span'), null,
           'span element should present in template content');


}, 'innerHTML of template element replaces all referenced by the content attribute. '
    + 'Test loading of HTML document from a file');


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
        "byte_end": 219,
        "byte_start": 140,
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
        "byte_end": 219,
        "byte_start": 140,
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
        "byte_end": 295,
        "byte_start": 220,
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
        "byte_end": 295,
        "byte_start": 220,
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
  "source_name": "html/semantics/scripting-1/the-template-element/innerhtml-on-templates/innerhtml.html"
}
```
