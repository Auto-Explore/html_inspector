# html/syntax/parsing/template/creating-an-element-for-the-token/template-owner-document.html

Counts:
- errors: 0
- warnings: 6
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/parsing/template/creating-an-element-for-the-token/template-owner-document.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>HTML Templates: ownerDocument property of the element in template</title>
<meta name="timeout" content="long">
<meta name="author" title="Sergey G. Grekhov" href="mailto:sgrekhov@unipro.ru">
<meta name="author" title="Aleksei Yu. Semenov" href="mailto:a.semenov@unipro.ru">
<meta name="assert" content="ownerDocument property of the element appended to template must be set to the template contents owner of the ownerDocument of the template element">
<link rel="help" href="http://www.w3.org/TR/2013/WD-html-templates-20130214/#creating-an-element-for-a-token">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/html/resources/common.js"></script>
</head>
<body>
<div id="log"></div>
<script type="text/javascript">


test(function () {
    var doc = newHTMLDocument();
    doc.body.innerHTML = '<div><template id="tmpl1"><div id="div">DIV</div></template></div>';

    var template = doc.querySelector('#tmpl1');

    var div = template.content.querySelector('#div');

    assert_equals(div.ownerDocument, template.content.ownerDocument,
            'Wrong ownerDocument of the element in template');

}, 'Test ownerDocument property of the element in a template. '
    + 'Current DOCUMENT has no browsing context. Test template element inside the div');



test(function () {
    var doc = newHTMLDocument();
    doc.body.innerHTML = '<template id="tmpl1"><div id="div">DIV</div></template>';

    var template = doc.querySelector('#tmpl1');

    var div = template.content.querySelector('#div');

    assert_equals(div.ownerDocument, template.content.ownerDocument,
            'Wrong ownerDocument of the element in template');

}, 'Test ownerDocument property of the element in a template. '
    + 'Current DOCUMENT has no browsing context. Test template element '
    + 'in the root of the body');



test(function () {
    var doc = newHTMLDocument();
    doc.head.innerHTML = '<template id="tmpl1"><div id="div">DIV</div></template>';

    var template = doc.querySelector('#tmpl1');

    var div = template.content.querySelector('#div');

    assert_equals(div.ownerDocument, template.content.ownerDocument,
            'Wrong ownerDocument of the element in template');

}, 'Test ownerDocument property of the element in a template. '
    + 'Current DOCUMENT has no browsing context. Test template element '
    + 'in the root of the head');



test(function () {
    var doc = newHTMLDocument();
    doc.body.innerHTML = '<template id="tmpl1">'
        + '<template id="tmpl2"><div id="div">DIV</div></template></template>';

    var template = doc.querySelector('#tmpl1');

    var nestedTemplate = template.content.querySelector('#tmpl2');

    assert_equals(nestedTemplate.ownerDocument, template.content.ownerDocument,
            'Wrong nested template owner document');

    var div = nestedTemplate.content.querySelector('#div');

    assert_equals(div.ownerDocument, nestedTemplate.content.ownerDocument,
            'Wrong div ownerDocument');

}, 'Test ownerDocument property of the element in a nested template');



testInIFrame('/html/semantics/scripting-1/the-template-element/resources/template-contents.html', function(context) {
    var doc = context.iframes[0].contentDocument;

    var template = doc.querySelector('template');

    var div = template.content.querySelector('div');

    assert_equals(div.ownerDocument, template.content.ownerDocument,
            'Wrong ownerDocument of the element in template');

}, 'Test ownerDocument property of the element in a template. '
   + 'Load HTML document from a file, current DOCUMENT has browsing context');



testInIFrame('/html/semantics/scripting-1/the-template-element/resources/template-contents-nested.html', function(context) {
    var doc = context.iframes[0].contentDocument;

    var template = doc.querySelector('template');

    var nestedTemplate = template.content.querySelector('template');

    assert_equals(nestedTemplate.ownerDocument, template.content.ownerDocument,
            'Wrong nested template owner document');

    var div = nestedTemplate.content.querySelector('div');

    assert_equals(div.ownerDocument, nestedTemplate.content.ownerDocument,
            'Wrong div ownerDocument');

}, 'Test ownerDocument property of the element in a nested template. '
    + 'Load HTML document from a file, current DOCUMENT has browsing context');



testInIFrame('/html/semantics/scripting-1/the-template-element/resources/two-templates.html', function(context) {
    var doc = context.iframes[0].contentDocument;

    var template1 = doc.querySelector('#template1');
    var div1 = template1.content.querySelector('div');
    var template2 = doc.querySelector('#template2');
    var div2 = template2.content.querySelector('div');

    assert_equals(div1.ownerDocument, template1.content.ownerDocument,
            'Wrong ownerDocument of the element in template');
    assert_equals(div2.ownerDocument, template2.content.ownerDocument,
            'Wrong ownerDocument of the element in template');
    assert_equals(div1.ownerDocument, div2.ownerDocument,
            'Different elements in the same document should share the same template contents owner');

}, 'Test ownerDocument property of two elements in a template. '
    + 'Load HTML document from a file, current DOCUMENT has browsing context');


var parameters = [];

HTML5_ELEMENTS.forEach(function(value) {
    if (value !== 'body' && value !== 'html' && value !== 'head' && value !== 'frameset') {

        var doc = newHTMLDocument();

        if (isVoidElement(value)) {
            doc.body.innerHTML = '<template><' + value + '/></template>';
        } else {
            doc.body.innerHTML = '<template><' + value + '></' + value + '></template>';
        }

        var template = doc.querySelector('template');
        var element = template.content.querySelector(value);

        doc.body.appendChild(template);

        parameters.push([
                'Test ownerDocument for the element ' + value + ' in the template',
                element,
                template
            ]);
    }
});

function compare_owners(element, template) {
    assert_equals(element.ownerDocument, template.content.ownerDocument)
}

// Test ownerDocument property of all HTML5 elements in a template.
// Current DOCUMENT has no browsing context.
generate_tests(compare_owners, parameters);

var context = newContext();
parameters = [];

try {

    HTML5_ELEMENTS.forEach(function(value) {

        if (value !== 'body' && value !== 'html' && value !== 'head' && value !== 'frameset') {

            var doc = newRenderedHTMLDocument(context);

            if (isVoidElement(value)) {
                doc.body.innerHTML = '<template><' + value + '/></template>';
            } else {
                doc.body.innerHTML = '<template><' + value + '></' + value + '></template>';
            }

            var template = doc.querySelector('template');
            var element = template.content.querySelector(value);

            doc.body.appendChild(template);

            parameters.push([
                    'Test ownerDocument for the element ' + value + ' in the template. '
                        + 'Document has browsing context',
                    element,
                    template
                ]);
        }
    });
    generate_tests(compare_owners, parameters,
            'Test ownerDocument property of all HTML5 elements in a template. '
                + 'Current DOCUMENT has browsing context.');

} finally {
    try {
        cleanContext(context);
    } catch (e) {
        //do nothing
    }
}

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
        "byte_end": 227,
        "byte_start": 148,
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
        "byte_end": 227,
        "byte_start": 148,
        "col": 1,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.attr.href.not_allowed",
      "message": "Attribute “href” not allowed on element “meta” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 310,
        "byte_start": 228,
        "col": 1,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.meta.missing_content",
      "message": "Element “meta” is missing one or more of the following attributes: “content”, “property”.",
      "severity": "Warning",
      "span": {
        "byte_end": 310,
        "byte_start": 228,
        "col": 1,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 823,
        "byte_start": 792,
        "col": 1,
        "line": 16
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
  "source_name": "html/syntax/parsing/template/creating-an-element-for-the-token/template-owner-document.html"
}
```
