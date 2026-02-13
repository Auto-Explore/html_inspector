# html/semantics/scripting-1/the-template-element/template-element/template-content.html

Counts:
- errors: 0
- warnings: 6
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-template-element/template-element/template-content.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>HTML Templates: HTML elements in template content</title>
<meta name="author" title="Sergey G. Grekhov" href="mailto:sgrekhov@unipro.ru">
<meta name="author" title="Aleksei Yu. Semenov" href="a.semenov@unipro.ru">
<meta name="assert" content="Template may contain any element, except the html element, the head element, the body element, or the frameset element">
<link rel="help" href="http://www.w3.org/TR/2013/WD-html-templates-20130214/#template-element">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src='/html/resources/common.js'></script>
</head>
<body>
<div id="log"></div>
<script type="text/javascript">

HTML5_ELEMENTS.forEach(function(value) {
    if (value !== 'body' && value !== 'html' && value !== 'head' && value !== 'frameset') {

        test(function() {
            var doc = newHTMLDocument();
            var template = doc.createElement('template');
            var element = doc.createElement(value);
            template.content.appendChild(element);
            var valueToTest = template.content.querySelector(value);

            doc.body.appendChild(template);

            assert_not_equals(valueToTest, null);
        }, 'Template may contain ' + value + ' element');

    }
});



var parameters = [];

HTML5_ELEMENTS.forEach(function(value) {
    if (value !== 'body' && value !== 'html' && value !== 'head' && value !== 'frameset') {

        test(function() {
            var doc = newHTMLDocument();

            if (isVoidElement(value)) {
                doc.body.innerHTML = '<template><' + value + '/></template>';
            } else {
                doc.body.innerHTML = '<template><' + value + '></' + value + '></template>';
            }

            var template = doc.querySelector('template');
            var element = template.content.querySelector(value);

            assert_not_equals(element, null);
        }, 'Template may contain ' + value + ' element. '
             + 'The template element and contents are added via body.innerHTML');

    }
});

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
        "byte_end": 174,
        "byte_start": 95,
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
        "byte_end": 174,
        "byte_start": 95,
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
        "byte_end": 250,
        "byte_start": 175,
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
        "byte_end": 250,
        "byte_start": 175,
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
        "byte_end": 720,
        "byte_start": 689,
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
  "source_name": "html/semantics/scripting-1/the-template-element/template-element/template-content.html"
}
```
