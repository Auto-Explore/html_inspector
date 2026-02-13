# html/webappapis/dynamic-markup-insertion/html-unsafe-methods/setHTMLUnsafe-xml.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/dynamic-markup-insertion/html-unsafe-methods/setHTMLUnsafe-xml.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://github.com/whatwg/html/pull/9538">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<script>
test(() => {
  const xmlDoc = document.implementation.createDocument(null, 'root', null);
  assert_equals(xmlDoc.contentType, 'application/xml');
  // Intentionally unclosed and misnested tags
  xmlDoc.documentElement.setHTMLUnsafe('<p><foo><b><i>test</b></i>');
  assert_equals(xmlDoc.documentElement.innerHTML,
    '<p xmlns="http://www.w3.org/1999/xhtml"><foo><b><i>test</i></b></foo></p>',
    'Element.setHTMLUnsafe should use the HTML parser in XML documents.');
}, 'setHTMLUnsafe should still parse HTML even in XML documents.');

test(() => {
  const svgDoc = document.implementation.createDocument('http://www.w3.org/2000/svg', 'root', null);
  assert_equals(svgDoc.contentType, 'image/svg+xml');
  // Intentionally unclosed and misnested tags
  svgDoc.documentElement.setHTMLUnsafe('<p><foo><b><i>test</b></i>');
  assert_equals(svgDoc.documentElement.innerHTML,
    '<p xmlns="http://www.w3.org/1999/xhtml"><foo><b><i>test</i></b></foo></p>',
    'Element.setHTMLUnsafe should use the HTML parser in SVG documents.');
}, 'setHTMLUnsafe should still parse HTML even in SVG documents.');
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/webappapis/dynamic-markup-insertion/html-unsafe-methods/setHTMLUnsafe-xml.html"
}
```
