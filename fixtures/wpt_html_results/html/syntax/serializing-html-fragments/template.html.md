# html/syntax/serializing-html-fragments/template.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/serializing-html-fragments/template.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<body>
<script>
test(() => {
  document.body.insertAdjacentHTML('afterbegin', '<div><template><table><td></table></template></div>');
  let t = document.querySelector('template');
  assert_equals(t.innerHTML, '<table><tbody><tr><td></td></tr></tbody></table>');
  assert_equals(t.parentNode.innerHTML, '<template><table><tbody><tr><td></td></tr></tbody></table></template>');
}, 'Template element content is correctly serialized');

test(() => {
  let t = document.createElement('template');
  let c = t.content.appendChild(document.createElementNS("xx", "div"));
  c.setAttributeNS('http://www.w3.org/XML/1998/namespace', 'xml:lang', 'en-us');
  c.setAttributeNS('uri2', 'p:attr', 'v');
  let doc = new Document();
  doc.adoptNode(t.content);
  assert_equals(t.innerHTML, '<div xml:lang="en-us" p:attr="v"></div>');
}, 'HTML fragment serialization algorithm should be applied to the template content');
</script>
</body>
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
  "source_name": "html/syntax/serializing-html-fragments/template.html"
}
```
