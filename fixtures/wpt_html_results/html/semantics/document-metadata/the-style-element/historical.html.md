# html/semantics/document-metadata/the-style-element/historical.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-style-element/historical.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Historical style element features should not be supported</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
function t(property) {
  test(function() {
    assert_false(property in document.createElement('style'));
  }, 'style.' + property + ' should not be supported');
}
// added in https://github.com/whatwg/html/commit/29cf39d2163cfc85b67409f4e10390619ffb2b40
// removed in https://github.com/whatwg/html/commit/c2a3b2a2e3db49c14b486a5e99acf7d10cfe8443
t('scoped');
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
  "source_name": "html/semantics/document-metadata/the-style-element/historical.html"
}
```
