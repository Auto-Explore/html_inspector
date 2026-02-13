# html/dom/elements/global-attributes/the-anchor-attribute-xml.tentative.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/the-anchor-attribute-xml.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel="help" href="https://github.com/whatwg/html/pull/9144">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<body>
<script>
test(() => {
  const xmlDoc = document.implementation.createDocument(null, 'root', null);
  assert_equals(xmlDoc.contentType, 'application/xml');
  const innerDoc = xmlDoc.documentElement;
  const html = '<div id="target">target</div><div anchor="target">anchored</div>';
  innerDoc.innerHTML = html;
  assert_equals(innerDoc.innerHTML, html);
  const target = innerDoc.children[0];
  const anchored = innerDoc.children[1];

  assert_equals(anchored.anchorElement, target, 'Setting the anchor attribute in XML should work.');

  anchored.anchorElement = target;
  assert_equals(anchored.anchorElement, target, 'Setting element.anchorElement in an XML document should work.');
});
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
  "source_name": "html/dom/elements/global-attributes/the-anchor-attribute-xml.tentative.html"
}
```
