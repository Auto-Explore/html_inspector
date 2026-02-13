# html/semantics/tabular-data/the-tbody-element/rows.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/tabular-data/the-tbody-element/rows.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>'tbody' element, 'rows' attribute</title>
<link rel="author" title="Corey Farwell" href="mailto:coreyf@rwell.org">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/html/semantics/tabular-data/html-table-section-element.js"></script>

<div id ="log"></div>

<script>
test(function () {
  testRowsAttribute('tbody');
});
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
  "source_name": "html/semantics/tabular-data/the-tbody-element/rows.html"
}
```
