# html/webappapis/dynamic-markup-insertion/html-unsafe-methods/Document-parseHTMLUnsafe-style-attribute.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/dynamic-markup-insertion/html-unsafe-methods/Document-parseHTMLUnsafe-style-attribute.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<link rel=author href="mailto:jarhar@chromium.org">
<!-- This test was adapted from style_attribute_html.html -->
<meta charset=utf-8>
<title>Style attribute in HTML</title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<script>

var div;
setup(function() {
    var input = '<div style="color: red">Foo</div>';
    var doc = Document.parseHTMLUnsafe(input);
    div = doc.querySelector('div');
});

test(function() {
    var style = div.style;
    assert_equals(style.cssText, 'color: red;');
    assert_equals(style.color, 'red');
    assert_equals(div.getAttribute("style"), 'color: red',
                  'Value of style attribute should match the string value that was set');
}, 'Parsing of initial style attribute');

test(function() {
    var style = div.style;
    div.setAttribute('style', 'color:: invalid');
    assert_equals(style.cssText, '');
    assert_equals(style.color, '');
    assert_equals(div.getAttribute('style'), 'color:: invalid',
                  'Value of style attribute should match the string value that was set');
}, 'Parsing of invalid style attribute');

test(function() {
    var style = div.style;
    div.setAttribute('style', 'color: green');
    assert_equals(style.cssText, 'color: green;');
    assert_equals(style.color, 'green');
    assert_equals(div.getAttribute('style'), 'color: green',
                  'Value of style attribute should match the string value that was set');
}, 'Parsing of style attribute');

test(function() {
    var style = div.style;
    style.backgroundColor = 'blue';
    assert_equals(style.cssText, 'color: green; background-color: blue;',
                  'Should not drop the existing style');
    assert_equals(style.color, 'green',
                  'Should not drop the existing style');
    assert_equals(div.getAttribute('style'), 'color: green; background-color: blue;',
                  'Should update style attribute');
}, 'Update style.backgroundColor');

</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/webappapis/dynamic-markup-insertion/html-unsafe-methods/Document-parseHTMLUnsafe-style-attribute.html"
}
```
