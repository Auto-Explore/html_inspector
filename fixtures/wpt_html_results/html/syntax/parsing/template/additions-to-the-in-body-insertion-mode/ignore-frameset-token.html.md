# html/syntax/parsing/template/additions-to-the-in-body-insertion-mode/ignore-frameset-token.html

Counts:
- errors: 0
- warnings: 6
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing/template/additions-to-the-in-body-insertion-mode/ignore-frameset-token.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>HTML Templates: In body insertion mode: parser should ignore FRAMESET token</title>
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
 * when parser is in "template content" mode and meets <frameset> tag it should be switched to
 * "in body" insertion mode.
 * According to https://html.spec.whatwg.org/multipage/#parsing-main-inbody
 * this token (FRAMESET) should be ignored
 */

test(function() {
    var doc = newHTMLDocument();
    var template = doc.createElement('template');

    template.innerHTML = '<frameset cols="25%,*,25%">'
        + '<frame src="frame_a.htm">'
        + '<frame src="frame_b.htm">' + '<frame src="frame_c.htm">'
        + '</frameset>';

    doc.body.appendChild(template);

    assert_equals(template.content.childNodes.length, 0,
            'Template cannot contain FRAMESET element');

}, 'Ignore frameset token. Test FRAMESET element assigned to template innerHTML');


test(function() {
    var doc = newHTMLDocument();
    var template = doc.createElement('template');

    template.innerHTML = '<div id="div1">Some text</div>'
        + '<frameset cols="25%,*,25%">'
        + '<frame src="frame_a.htm">'
        + '<frame src="frame_b.htm">'
        + '<frame src="frame_c.htm">'
        + '</frameset>';

    doc.body.appendChild(template);

    assert_equals(template.content.childNodes.length, 1,
            'Template cannot contain FRAMESET element');
    assert_not_equals(template.content.querySelector('#div1'), null,
            'Template should contain valid element');

}, 'Ignore frameset token. '
    + 'Test FRAMESET element and some valid element before it, assigned '
    + 'to the template\'s innerHTML');


test(function() {
    var doc = newHTMLDocument();
    var template = doc.createElement('template');

    template.innerHTML = '<frameset cols="25%,*,25%">'
        + '<frame src="frame_a.htm">'
        + '<frame src="frame_b.htm">'
        + '<frame src="frame_c.htm">'
        + '</frameset><div id="div1">Some text</div>';

    doc.body.appendChild(template);

    assert_equals(template.content.childNodes.length, 1,
            'Template cannot contain FRAMESET element');
    assert_not_equals(template.content.querySelector('#div1'), null,
            'Template should contain valid element');

}, 'Ignore frameset token. '
    + 'Test FRAMESET element and some valid element after it, assigned '
    + 'to the template\'s innerHTML');


test(function() {
    var doc = newHTMLDocument();
    var template = doc.createElement('template');

    template.innerHTML = '<template id="t2">'
        + '<frameset cols="25%,*,25%">'
        + '<frame src="frame_a.htm">'
        + '<frame src="frame_b.htm">'
        + '<frame src="frame_c.htm">'
        + '</frameset></template>';

    doc.body.appendChild(template);

    assert_equals(template.content.childNodes.length, 1,
            'Template should contain nested template');
    assert_not_equals(template.content.querySelector('#t2'), null,
            'Template should contain nested element');

    var nestedTemplate = template.content.querySelector('#t2');

    assert_equals(nestedTemplate.content.childNodes.length, 0,
            'Template cannot contain FRAMESET element');

}, 'Ignore frameset token. '
    + 'Test FRAMESET tag inside template tag assigned to another template\'s innerHTML');


testInIFrame('/html/semantics/scripting-1/the-template-element/resources/template-contents-frameset.html', function(context) {
    var doc = context.iframes[0].contentDocument;

    var template = doc.body.querySelector('template');

    assert_equals(template.content.childNodes.length, 0,
            'Template cannot contain FRAMESET element');
}, 'Ignore frameset token. Test loading a HTML file with FRAMESET tag inside template');

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
        "byte_end": 200,
        "byte_start": 121,
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
        "byte_end": 200,
        "byte_start": 121,
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
        "byte_start": 201,
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
        "byte_start": 201,
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
        "byte_end": 717,
        "byte_start": 686,
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
  "source_name": "html/syntax/parsing/template/additions-to-the-in-body-insertion-mode/ignore-frameset-token.html"
}
```
