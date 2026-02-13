# html/syntax/parsing/template/additions-to-the-in-body-insertion-mode/ignore-body-token.html

Counts:
- errors: 0
- warnings: 6
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing/template/additions-to-the-in-body-insertion-mode/ignore-body-token.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>HTML Templates: In body insertion mode: parser should ignore BODY token</title>
<meta name="author" title="Sergey G. Grekhov" href="mailto:sgrekhov@unipro.ru">
<meta name="author" title="Aleksei Yu. Semenov" href="mailto:a.semenov@unipro.ru">
<meta name="assert" content="http://www.w3.org/TR/2013/WD-html-templates-20130214/#in-body-addition">
<link rel="help" href="http://www.w3.org/TR/2013/WD-html-templates-20130214/#in-body-addition">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/html/resources/common.js"></script>
</head>
<body>
<div id="log"></div>
<script type="text/javascript">

/*
 * According to http://www.w3.org/TR/2013/WD-html-templates-20130214/#template-contents-insertion-mode
 * when parser is in "template content" mode and meets <body> tag it should be switched to
 * "in body" insertion mode.
 * According to http://www.w3.org/TR/2013/WD-html-templates-20130214/#in-body-addition
 * this token (BODY) should be ignored
 */


test(function() {
    var doc = newHTMLDocument();
    var template = doc.createElement('template');

    template.innerHTML = '<body></body>';

    doc.body.appendChild(template);

    assert_equals(template.content.childNodes.length, 0,
            'Template cannot contain BODY element');

}, 'Ignore BODY token. Test empty BODY element assigned to template innerHTML');


test(function() {
    var doc = newHTMLDocument();
    var template = doc.createElement('template');

    template.innerHTML = '<body><div>Some content</div></body>';

    doc.body.appendChild(template);

    assert_equals(template.content.childNodes.length, 1,
            'Wrong number of template content children');
    assert_equals(template.content.firstChild.nodeName, 'DIV',
            'Template should contain children of ignored BODY element');

}, 'Ignore BODY token. Test not empty BODY element assigned to template innerHTML');


test(function() {
    var doc = newHTMLDocument();
    var template = doc.createElement('template');

    template.innerHTML = '<body><div <div id="div1">Some content</div></body><div id="div2">Some valid content</div>';

    doc.body.appendChild(template);

    assert_equals(template.content.childNodes.length, 2,
            'Wrong number of template content children');
    assert_not_equals(template.content.querySelector('#div1'), null,
            'Template should contain children of the ignored BODY element');
    assert_not_equals(template.content.querySelector('#div2'), null,
            'Template should contain valid element');

}, 'Ignore BODY token. '
        + 'Test BODY element and some valid element after BODY tag assigned to template innerHTML');


test(function() {
    var doc = newHTMLDocument();
    var template = doc.createElement('template');

    template.innerHTML = '<div id="div1">Some valid content</div><body><div id="div2">Some content</div></body>';

    doc.body.appendChild(template);

    assert_equals(template.content.childNodes.length, 2,
            'Template cannot contain BODY element');
    assert_not_equals(template.content.querySelector('#div1'), null,
            'Template should contain valid element');
    assert_not_equals(template.content.querySelector('#div2'), null,
            'Template should contain children of the ignored BODY element');

}, 'Ignore BODY token. '
        + 'Test BODY element and some valid element before BODY tag assigned to template innerHTML');


test(function() {
    var doc = newHTMLDocument();
    var template = doc.createElement('template');

    template.innerHTML = '<template id="t2"><body><span>Body!<span></body></template>';

    doc.body.appendChild(template);

    assert_equals(template.content.childNodes.length, 1,
            'Template should contain nested template');
    assert_not_equals(template.content.querySelector('#t2'), null,
            'Template should contain nested element');

    var nestedTemplate = template.content.querySelector('#t2');

    assert_equals(nestedTemplate.content.childNodes.length, 1,
            'Template cannot contain BODY element');
    assert_equals(nestedTemplate.content.firstChild.nodeName, 'SPAN',
            'Template cannot contain BODY element');

}, 'Ignore BODY token. '
    + 'Test template with not empty BODY element inside assigned to another '
    + 'template\'s innerHTML');


testInIFrame('/html/semantics/scripting-1/the-template-element/resources/template-contents-body.html', function(context) {
    var doc = context.iframes[0].contentDocument;

    var template = doc.body.querySelector('template');

    assert_equals(template.content.childNodes.length, 0,
            'Template cannot contain BODY element');

}, 'Ignore BODY token. '
    + 'Test loading a HTML file with BODY tag inside template');

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
        "byte_end": 196,
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
        "byte_end": 196,
        "byte_start": 117,
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
        "byte_end": 279,
        "byte_start": 197,
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
        "byte_end": 279,
        "byte_start": 197,
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
  "source_name": "html/syntax/parsing/template/additions-to-the-in-body-insertion-mode/ignore-body-token.html"
}
```
