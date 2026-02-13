# html/syntax/parsing/template/additions-to-the-in-body-insertion-mode/ignore-html-token.html

Counts:
- errors: 0
- warnings: 6
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing/template/additions-to-the-in-body-insertion-mode/ignore-html-token.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>HTML Templates: In body insertion mode: parser should ignore HTML token</title>
<meta name="author" title="Sergey G. Grekhov" href="mailto:sgrekhov@unipro.ru">
<meta name="author" title="Aleksei Yu. Semenov" href="mailto:a.semenov@unipro.ru">
<meta name="assert" content="If parser is in 'in body' insertion mode and meets HTML token it should be ignored">
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
 * when parser is in "template content" mode and meets <html> tag it should be switched to
 * "in body" insertion mode.
 * According to https://html.spec.whatwg.org/multipage/#parsing-main-inbody
 * this token (HTML) should be ignored
 */

test(function() {
    var doc = newHTMLDocument();
    var template = doc.createElement('template');

    template.innerHTML = '<html><body></body></html>';

    doc.body.appendChild(template);

    assert_equals(template.content.childNodes.length, 0,
            'Template cannot contain HTML element');

}, 'Ignore HTML token. Test HTML element assigned to template innerHTML');


test(function() {
    var doc = newHTMLDocument();
    var template = doc.createElement('template');

    template.innerHTML = '<div id="div1">Some text</div><html><body></body></html>';

    doc.body.appendChild(template);

    assert_equals(template.content.childNodes.length, 1,
            'Template cannot contain HTML element');
    assert_not_equals(template.content.querySelector('#div1'), null,
            'Template should contain valid element');

}, 'Ignore HTML token.'
    + 'Test HTML element and some valid element before it, assigned to template innerHTML');


test(function() {
    var doc = newHTMLDocument();
    var template = doc.createElement('template');

    template.innerHTML = '<html><body></body></html><div id="div1">Some text</div>';

    doc.body.appendChild(template);

    assert_equals(template.content.childNodes.length, 1,
            'Template cannot contain HTML element');
    assert_not_equals(template.content.querySelector('#div1'), null,
            'Template should contain valid element');

}, 'Ignore HTML token. '
    + 'Test HEAD element and some valid element after it, assigned to template innerHTML');


test(function() {
    var doc = newHTMLDocument();
    var template = doc.createElement('template');

    template.innerHTML = '<template id="t2"><html><body></body></html></template>';

    doc.body.appendChild(template);

    assert_equals(template.content.childNodes.length, 1,
            'Template should contain nested template');
    assert_not_equals(template.content.querySelector('#t2'), null,
            'Template should contain nested element');

    var nestedTemplate = template.content.querySelector('#t2');

    assert_equals(nestedTemplate.content.childNodes.length, 0,
            'Template cannot contain HTML element');

}, 'Ignore HTML token. '
    + 'Test HTML tag inside template tag assigned to another template\'s innerHTML');


test(function() {
    var doc = newHTMLDocument();
    var template = doc.createElement('template');

    template.innerHTML = '<html><div id="div1">Some text</div></html>';

    doc.body.appendChild(template);

    assert_equals(template.content.childNodes.length, 1,
            'Template cannot contain HTML element');
    assert_not_equals(template.content.querySelector('#div1'), null,
            'Template should contain a valid element');

}, 'Ignore HTML token. Test some valid element inside HTML element');


test(function() {
    var doc = newHTMLDocument();
    var template = doc.createElement('template');

    template.innerHTML = '<html><body><div id="div1">Some text</div><body></html>';

    doc.body.appendChild(template);

    assert_equals(template.content.childNodes.length, 1,
            'Template cannot contain HTML element');
    assert_not_equals(template.content.querySelector('#div1'), null,
            'Template should contain valid element');

}, 'Ignore HTML token. Test valid element inside HTML and BODY elements');


test(function() {
    var doc = newHTMLDocument();
    var template = doc.createElement('template');

    template.innerHTML = '<html><span id="span1">Span</span><body><div id="div1">Some text</div><body></html>';

    doc.body.appendChild(template);

    assert_equals(template.content.childNodes.length, 2,
            'Template cannot contain HTML element');
    assert_not_equals(template.content.querySelector('#div1'), null,
            'Template should contain valid DIV element');

    assert_not_equals(template.content.querySelector('#span1'), null,
            'Template should contain valid SPAN element');

}, 'Ignore HTML token. Test valid element inside and between HTML and BODY elements');


testInIFrame('/html/semantics/scripting-1/the-template-element/resources/template-contents-html.html', function(context) {
    var doc = context.iframes[0].contentDocument;

    var template = doc.body.querySelector('template');

    assert_equals(template.content.childNodes.length, 0,
            'Template cannot contain HTML element');

}, 'Ignore HTML token. Test loading a HTML file with HTML tag inside template');

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
        "byte_end": 713,
        "byte_start": 682,
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
  "source_name": "html/syntax/parsing/template/additions-to-the-in-body-insertion-mode/ignore-html-token.html"
}
```
