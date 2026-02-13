# html/semantics/tabular-data/historical.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/tabular-data/historical.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Historical table features should not be supported</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<script>
function t(property, tagNames) {
  if (typeof tagNames === "string") {
    tagNames = [tagNames];
  }
  tagNames.forEach(function(tagName) {
    test(function() {
      assert_false(property in document.createElement(tagName));
    }, tagName + '.' + property + ' should not be supported');
  });
}

// added in https://github.com/whatwg/html/commit/6db0d8d4e3456140de958c963afe9bb9ec7b6a25
// removed in https://github.com/whatwg/html/commit/59b7e2466c2b7c5c408a4963b05b13fd808aa07a
t('onsort', 'table');
t('sortable', 'table');
t('stopSorting', 'table');
t('sorted', 'th');
t('sort', 'th');
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
  "source_name": "html/semantics/tabular-data/historical.html"
}
```
