# html/semantics/embedded-content/the-img-element/img-src-in-synthetic-document.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-img-element/img-src-in-synthetic-document.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://github.com/whatwg/html/issues/9855">
<link rel=help href="https://html.spec.whatwg.org/#reflecting-content-attributes-in-idl-attributes">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<script>
test(() => {
  const doc = document.implementation.createHTMLDocument('');
  const img = doc.createElement('img');
  img.setAttribute('src', '/test');
  doc.body.appendChild(img);
  assert_equals(img.src, '/test');
}, 'HTMLImageElement.src should return the string from the attribute in about:blank documents.');
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
  "source_name": "html/semantics/embedded-content/the-img-element/img-src-in-synthetic-document.html"
}
```
