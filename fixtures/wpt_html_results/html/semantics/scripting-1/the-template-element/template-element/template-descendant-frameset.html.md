# html/semantics/scripting-1/the-template-element/template-element/template-descendant-frameset.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-template-element/template-element/template-descendant-frameset.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>HTML Templates: Template element as a descendant of the frameset element.</title>
<meta name="author" title="Aleksei Yu. Semenov" href="mailto:a.semenov@unipro.ru">
<meta name="assert" content="Template element can not be a descendant of the frameset element">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#parsing-main-inframeset">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src='/html/resources/common.js'></script>
</head>
<body>
<div id="log"></div>
<script type="text/javascript">

testInIFrame('../resources/template-descendant-frameset.html', function(context) {
    var doc = context.iframes[0].contentDocument;

    var frameset = doc.querySelector('frameset');

    assert_equals(frameset.querySelector('template'), null,
        'Template element should not be a descendant of the frameset element');

}, 'Template element as a descendant of the frameset element. Test loading from a file');


testInIFrame('../resources/template-descendant-frameset.html', function(context) {
    var doc = context.iframes[0].contentDocument;

    var frameset = doc.querySelector('frameset');

    frameset.innerHTML = '';
    assert_equals(doc.querySelector('template'), null,
            'Initial conditions are not satisfied');

    frameset.innerHTML = '<template>some text</template>';

    assert_equals(frameset.querySelector('template'), null,
        'Template element should not be a descendant of the frameset element');

}, 'Template element as a descendant of the frameset element. '
    + 'Test template element is assigned to frameset\'s innerHTML)');


testInIFrame('../resources/template-descendant-frameset.html', function(context) {
    var doc = context.iframes[0].contentDocument;

    var frameset = doc.querySelector('frameset');

    var template = doc.createElement('template');
    frameset.appendChild(template);

    assert_equals(frameset.querySelectorAll('template').length, 1,
        'Template element should be a descendant of the frameset element');

}, 'Template element as a descendant of the frameset element. '
    + 'Test template element appended to frameset by appendChild()');


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
        "byte_end": 201,
        "byte_start": 119,
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
        "byte_end": 201,
        "byte_start": 119,
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
        "byte_end": 610,
        "byte_start": 579,
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
  "source_name": "html/semantics/scripting-1/the-template-element/template-element/template-descendant-frameset.html"
}
```
