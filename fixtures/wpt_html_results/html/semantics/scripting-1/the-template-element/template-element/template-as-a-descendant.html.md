# html/semantics/scripting-1/the-template-element/template-element/template-as-a-descendant.html

Counts:
- errors: 0
- warnings: 6
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-template-element/template-element/template-as-a-descendant.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>HTML Templates: Template element as a descendant of the body element.</title>
<meta name="author" title="Sergey G. Grekhov" href="mailto:sgrekhov@unipro.ru">
<meta name="author" title="Aleksei Yu. Semenov" href="mailto:a.semenov@unipro.ru">
<meta name="assert" content="Template element can be a descendant of the body element">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-template-element">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src='/html/resources/common.js'></script>
</head>
<body>
<div id="log"></div>
<script type="text/javascript">

function templateIsAChild(element) {
    element.innerHTML = '<template>some text</template>';

    assert_not_equals(element.querySelector('template'), null,
        'Template element should be a descendant of the ' + element.tagName + ' element');
}

function templateIsDisallowedAsAChild(element) {
    element.innerHTML = '<template>some text</template>';

    assert_equals(element.querySelector('template'), null,
        'Template element should not be allowed as a descendant of the ' + element.tagName + ' element');
}

function templateIsAnIndirectChild(element) {
    element.innerHTML = '<div><template>some text</template></div>';

    assert_not_equals(element.querySelector('template'), null,
        'Template element should be a descendant of the ' + element.tagName + ' element');
}

function templateIsDisallowedAsAnIndirectChild(element) {
    element.innerHTML = '<div><template>some text</template></div>';

    assert_equals(element.querySelector('template'), null,
        'Template element should not be allowed as indirect descendant of the ' + element.tagName + ' element');
}

function templateIsAnAppendedChild(doc, element) {
    var template = doc.createElement('template');

    element.appendChild(template);

    assert_not_equals(element.querySelector('template'), null,
        'Template element should be a descendant of the ' + element.tagName + ' element');
}

function templateIsAnAppendedIndirectChild(doc, element) {
    var template = doc.createElement('template');
    var div = doc.createElement('div');
    div.appendChild(template);

    element.appendChild(div);

    assert_not_equals(element.querySelector('template'), null,
        'Template element should be a descendant of the ' + element.tagName + ' element');
}

var doc = newHTMLDocument();
var frameset = doc.createElement('frameset');

var parameters = [['Template element as a descendant of the BODY element. ' +
                   'Template element is created by innerHTML',
                   doc.body],
                  ['Template element as a descendant of the HEAD element. ' +
                   'Template element is created by innerHTML',
                   doc.head],
                   ];
// Template element as a descendant of the HEAD and BODY elements
generate_tests(templateIsAChild, parameters);

parameters = [['Template element as a descendant of the FRAMESET element. ' +
               'Template element is created by innerHTML',
               frameset],
               ];
// Template element should be disallowed as a descendant of the FRAMESET elements
generate_tests(templateIsDisallowedAsAChild, parameters);


parameters = [['Template element as an indirect descendant of the BODY element. ' +
               'Template element is created by innerHTML',
               doc.body],
              ['Template element as an indirect descendant of the HEAD element. ' +
               'Template element is created by innerHTML',
               doc.head],
               ];
// Template element as an indirect descendant of the HEAD, BODY and FRAMESET elements
generate_tests(templateIsAnIndirectChild, parameters);

parameters = [['Template element as an indirect descendant of the FRAMESET element. ' +
               'Template element is created by innerHTML',
               frameset],
               ];
// Template element should be disallowed as an indirect descendant of the FRAMESET elements
generate_tests(templateIsDisallowedAsAnIndirectChild, parameters);



parameters = [['Template element as a descendant of the BODY element. ' +
               'Template element is appended by appendChild()',
               doc, doc.body],
              ['Template element as a descendant of the HEAD element. ' +
               'Template element is appended by appendChild()',
               doc, doc.head],
               ['Template element as a descendant of the FRAMESET element. ' +
                'Template element is  appended by appendChild()',
                doc, frameset]
               ];
// Template element as a descendant of the HEAD, BODY and FRAMESET elements
generate_tests(templateIsAnAppendedChild, parameters);



parameters = [['Template element as an indirect descendant of the BODY element. ' +
               'Template element is appended by appendChild()',
               doc, doc.body],
              ['Template element as an indirect descendant of the HEAD element. ' +
               'Template element is appended by appendChild()',
               doc, doc.head],
               ['Template element as an indirect descendant of the FRAMESET element. ' +
                'Template element is  appended by appendChild()',
                doc, frameset]
               ];
// Template element as a descendant of the HEAD, BODY and FRAMESET elements
generate_tests(templateIsAnAppendedIndirectChild, parameters);

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
        "byte_end": 194,
        "byte_start": 115,
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
        "byte_end": 194,
        "byte_start": 115,
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
        "byte_end": 277,
        "byte_start": 195,
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
        "byte_end": 277,
        "byte_start": 195,
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
        "byte_end": 675,
        "byte_start": 644,
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
  "source_name": "html/semantics/scripting-1/the-template-element/template-element/template-as-a-descendant.html"
}
```
