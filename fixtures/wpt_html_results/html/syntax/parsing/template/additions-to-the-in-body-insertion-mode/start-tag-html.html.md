# html/syntax/parsing/template/additions-to-the-in-body-insertion-mode/start-tag-html.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing/template/additions-to-the-in-body-insertion-mode/start-tag-html.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>HTML Templates: In body insertion mode: A start tag whose tag name is html</title>
<meta name="author" title="Sergey G. Grekhov" href="mailto:sgrekhov@unipro.ru">
<meta name="assert" content="If HTML parser is in 'in body' insertion mode and meets HTML start tag, then for each attribute on the token, check to see if the attribute is already present on the top element of the stack of open elements. If it is not, add the attribute and its corresponding value to that element">
<link rel="help" href="http://www.w3.org/TR/2013/WD-html-templates-20130214/#in-body-addition">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/html/resources/common.js"></script>
</head>
<body>
<div id="log"></div>
<script type="text/javascript">

// test <template><html class="htmlClass"></html></template><html id="htmlId" tabindex="5">
// id attribute should be added to root <html> element
// tabindex attribute should not be modified
//class attribute should be ignored
testInIFrame('/html/semantics/scripting-1/the-template-element/resources/html-start-tag.html', function(context) {
    var doc = context.iframes[0].contentDocument;

    var template = doc.body.querySelector('template');

    var html = doc.documentElement;

    assert_equals(html.getAttribute('tabindex'), '5', 'Attribute should be accessible');
    assert_equals(html.getAttribute('id'), 'htmlId',
            'Attribute \'id\' should be added and accessible');
    assert_false(html.hasAttribute('class'), 'Attribute \'class\' should be ignored');
    assert_equals(template.content.childNodes.length, 0, 'Template should not contain HTML element');


}, 'In body insertion mode: html start tag should add only absent attributes');

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
        "byte_end": 199,
        "byte_start": 120,
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
        "byte_end": 199,
        "byte_start": 120,
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
        "byte_end": 835,
        "byte_start": 804,
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
  "source_name": "html/syntax/parsing/template/additions-to-the-in-body-insertion-mode/start-tag-html.html"
}
```
