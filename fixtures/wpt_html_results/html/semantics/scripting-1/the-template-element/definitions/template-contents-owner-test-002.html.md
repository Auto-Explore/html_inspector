# html/semantics/scripting-1/the-template-element/definitions/template-contents-owner-test-002.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-template-element/definitions/template-contents-owner-test-002.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>HTML Templates: The template contents owner document (there's browsing context)</title>
<meta name="author" title="Sergey G. Grekhov" href="mailto:sgrekhov@unipro.ru">
<meta name="assert" content="If template's enclosing document has browsing context, then templates content owner must be a new Document node without browsing context">
<link rel="help" href="http://www.w3.org/TR/2013/WD-html-templates-20130214/#definitions">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src='/html/resources/common.js'></script>
</head>
<body>
<div id="log"></div>
<script type="text/javascript">


testInIFrame(null, function(context) {
    var doc = context.iframes[0].contentDocument;
    var template = doc.createElement('template');

    var div = doc.createElement('div');
    div.setAttribute('id', 'div1');

    template.appendChild(div);

    doc.body.appendChild(template);

    // doc has browsing context. There should be another document as a template
    // content owner
    assert_not_equals(template.content.ownerDocument, doc, 'Wrong template owner document');

}, 'The template contents owner document must be different from template owner document,' +
     ' which has browsing context. Template element is created by createElement()');



testInIFrame(null, function(context) {
    var doc = context.iframes[0].contentDocument;

    doc.body.innerHTML = '<template><div>some text</div></template>';

    var template = doc.querySelector('template');

    // doc has browsing context. There should be another document as a template
    // content owner
    assert_not_equals(template.content.ownerDocument, doc, 'Wrong template owner document');

}, 'The template contents owner document must be different from template owner document,' +
     ' which has browsing context. Template element is created via innerHTML');



testInIFrame('../resources/template-contents.html', function(context) {
    var doc = context.iframes[0].contentDocument;

    var template = doc.querySelector('template');

    // doc has browsing context. There should be another document as a template
    // content owner
    assert_not_equals(template.content.ownerDocument, doc, 'Wrong template owner document');

}, 'The template contents owner document must be different from template owner document,' +
     ' which has browsing context. Template element is created by HTML parser');

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
        "byte_end": 204,
        "byte_start": 125,
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
        "byte_end": 204,
        "byte_start": 125,
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
        "byte_end": 687,
        "byte_start": 656,
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
  "source_name": "html/semantics/scripting-1/the-template-element/definitions/template-contents-owner-test-002.html"
}
```
