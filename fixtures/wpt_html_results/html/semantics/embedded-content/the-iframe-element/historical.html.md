# html/semantics/embedded-content/the-iframe-element/historical.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-iframe-element/historical.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Historical iframe element features should not be supported</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<script>
function t(property) {
  test(function() {
    assert_false(property in document.createElement('iframe'));
  }, 'iframe.' + property + ' should not be supported');
}

// added in https://github.com/whatwg/html/commit/f6490f17f577fa3478791b29ad8c2b586418001f
// removed in https://github.com/whatwg/html/commit/1490eba4dba5ab476f0981443a86c01acae01311
t('seamless');

// Added by https://github.com/whatwg/html/pull/2133
// Removed by https://github.com/whatwg/html/pull/5915
t('allowPaymentRequest');
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
  "source_name": "html/semantics/embedded-content/the-iframe-element/historical.html"
}
```
