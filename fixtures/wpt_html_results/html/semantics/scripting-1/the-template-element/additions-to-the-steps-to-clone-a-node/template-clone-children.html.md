# html/semantics/scripting-1/the-template-element/additions-to-the-steps-to-clone-a-node/template-clone-children.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-template-element/additions-to-the-steps-to-clone-a-node/template-clone-children.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>HTML Templates: Clone template node: All the children of template content are copied if 'copy children flag' set to true</title>
<meta name="author" title="Sergey G. Grekhov" href="mailto:sgrekhov@unipro.ru">
<meta name="assert" content="Clone template node: all the children of template content are copied if 'copy children flag' set to true">
<link rel="help" href="http://www.w3.org/TR/2013/WD-html-templates-20130214/#node-clone-additions">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src='/html/resources/common.js'></script>
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
    var copy = template.cloneNode(true);

    assert_not_equals(copy.content, undefined, 'Template clone content attribute should not be undefined');
    assert_not_equals(copy.content, null, 'Template clone content attribute should not be null');

    assert_equals(copy.content.childNodes.length, 2,
            'Wrong number of template content\'s copy child nodes');
    assert_not_equals(copy.content.querySelector('#div1'), null,
            'Template child node should be copied');
    assert_not_equals(copy.content.querySelector('#div2'), null,
            'Template child node should be copied');

}, 'Clone template node. Test call to cloneNode(true)');



test(function () {
    var doc = newHTMLDocument();
    doc.body.innerHTML = '<template id="tmpl1">' +
        '<div id="div1">This is div inside template</div>' +
        '<div id="div2">This is another div inside template</div>' +
        '</template>';

    var template = doc.querySelector('#tmpl1');
    var copy = template.cloneNode();

    assert_not_equals(copy.content, undefined, 'Template clone content attribute should not be undefined');
    assert_not_equals(copy.content, null, 'Template clone content attribute should not be null');

    assert_equals(copy.content.childNodes.length, 0,
            'Wrong number of template content\'s copy child nodes');

}, 'Clone template node. Test call to cloneNode() with the default parameter '
    + '(false by default)');



test(function () {
    var doc = newHTMLDocument();
    doc.body.innerHTML = '<template id="tmpl1">' +
        '<div id="div1">This is div inside template</div>' +
        '<div id="div2">This is another div inside template</div>' +
        '</template>';

    var template = doc.querySelector('#tmpl1');
    var copy = template.cloneNode(false);

    assert_not_equals(copy.content, undefined, 'Template clone content attribute is undefined');
    assert_not_equals(copy.content, null, 'Template clone content attribute is null');

    assert_equals(copy.content.childNodes.length, 0,
            'Wrong number of template content\'s copy child nodes');

}, 'Clone template node. Test call to cloneNode(false)');


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
        "byte_end": 245,
        "byte_start": 166,
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
        "byte_end": 245,
        "byte_start": 166,
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
        "byte_end": 705,
        "byte_start": 674,
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
  "source_name": "html/semantics/scripting-1/the-template-element/additions-to-the-steps-to-clone-a-node/template-clone-children.html"
}
```
