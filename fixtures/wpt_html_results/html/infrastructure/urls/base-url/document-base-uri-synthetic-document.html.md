# html/infrastructure/urls/base-url/document-base-uri-synthetic-document.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/infrastructure/urls/base-url/document-base-uri-synthetic-document.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel=author href="mailto:jarhar@chromium.org">
<link rel=help href="https://bugs.chromium.org/p/chromium/issues/detail?id=1486750">
<link rel=help href="https://html.spec.whatwg.org/multipage/urls-and-fetching.html#document-base-url">
<link rel=help href="https://html.spec.whatwg.org/multipage/urls-and-fetching.html#fallback-base-url">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<body>
<script>
test(() => {
  const doc1 = document.implementation.createHTMLDocument();
  assert_equals(doc1.baseURI, 'about:blank', 'document.implementation.createHTMLDocument()');

  const doc2 = document.implementation.createDocument('', '');
  assert_equals(doc2.baseURI, 'about:blank', 'document.implementation.createDocument("", "")');

  const doc3 = new Document();
  assert_equals(doc3.baseURI, 'about:blank', 'new Document()');
}, 'Synthetic documents should return about:blank for document.baseURI.');

test(() => {
  const doc = document.implementation.createHTMLDocument();
  assert_equals(doc.baseURI, 'about:blank', 'baseURI should be about:blank without a <base>.');

  const base = doc.createElement('base');
  base.href = '/foo';
  doc.head.appendChild(base);
  assert_equals(doc.baseURI, 'about:blank', '<base> with relative URL should not change the about:blank baseURI.');

  base.href = 'http://example.com/';
  assert_equals(doc.baseURI, 'http://example.com/', '<base> with complete URL should replace the about:blank baseURI.');
}, 'Synthetic documents should incorporate <base> href URLs correctly.');
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
  "source_name": "html/infrastructure/urls/base-url/document-base-uri-synthetic-document.html"
}
```
