# html/semantics/scripting-1/the-template-element/serializing-html-templates/outerhtml.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-template-element/serializing-html-templates/outerhtml.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>HTML Templates: serialize template contents instead of template element</title>
<meta name="author" title="Aleksei Yu. Semenov" href="a.semenov@unipro.ru">
<meta name="assert" content="template contents should be serialized instead of template element if serializing template element">
<link rel="help" href="http://www.w3.org/TR/2013/WD-html-templates-20130214/#serializing-html-templates">
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

    var div = doc.createElement('div');
    div.setAttribute('id', 'div1');
    div.innerHTML = 'some text';
    template.content.appendChild(div);

    assert_equals(template.outerHTML, '<template><div id="div1">some text</div></template>',
       'template element is serialized incorrectly');

}, 'Template contents should be serialized instead of template element if serializing template element');



test(function () {
    var doc = newHTMLDocument();
    var template = doc.createElement('template');
    var nestedTemplate = doc.createElement('template');

    template.content.appendChild(nestedTemplate);

    var div = doc.createElement('div');
    div.setAttribute('id', 'div1');
    div.innerHTML = 'some text';
    nestedTemplate.content.appendChild(div);

    assert_equals(template.outerHTML, '<template><template><div id="div1">some text</div></template></template>',
       'template element is serialized incorrectly');


}, 'Template contents should be serialized instead of template element if serializing template element. '
    + 'Test nested template');


test(function () {
    var doc = newHTMLDocument();
    var template = doc.createElement('template');

    var div = doc.createElement('div');
    div.setAttribute('id', 'div1');
    div.innerHTML = 'some text';
    template.content.appendChild(div);
    doc.body.appendChild(template);

    assert_equals(doc.documentElement.outerHTML, '<html><head><title>Test Document</title></head><body><template><div id="div1">some text</div></template></body></html>',
       'template element is serialized incorrectly');

}, 'Template contents should be serialized instead of template element if serializing template element. '
    + 'Test serializing whole document');

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
        "byte_end": 192,
        "byte_start": 117,
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
        "byte_end": 192,
        "byte_start": 117,
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
        "byte_end": 652,
        "byte_start": 621,
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
  "source_name": "html/semantics/scripting-1/the-template-element/serializing-html-templates/outerhtml.html"
}
```
