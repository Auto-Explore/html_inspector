# html/rendering/non-replaced-elements/tables/hidden-attr.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/tables/hidden-attr.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>UA style for hidden attribute on table elements</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#tables-2">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<table hidden></table>
<table><caption hidden></caption></table>
<table><colgroup hidden></table>
<table><col hidden></table>
<table><thead hidden></table>
<table><tbody hidden></table>
<table><tfoot hidden></table>
<table><tr hidden></table>
<table><tr><td hidden></table>
<table><tr><th hidden></table>
<script>
const expectedDisplay = {
  'table': 'none',
  'caption': 'none',
  'colgroup': 'table-column-group',
  'col': 'table-column',
  'thead': 'table-header-group',
  'tbody': 'table-row-group',
  'tfoot': 'table-footer-group',
  'tr': 'table-row',
  'td': 'none',
  'th': 'none',
};
for (const el of document.querySelectorAll("[hidden]")) {
  test(function() {
    const style = getComputedStyle(el);
    assert_equals(style.display, expectedDisplay[el.localName]);
    if (el instanceof HTMLTableElement ||
        el instanceof HTMLTableCaptionElement ||
        el instanceof HTMLTableCellElement) {
      assert_equals(style.visibility, 'visible');
    } else {
      assert_equals(style.visibility, 'collapse');
    }
  }, `Computed display and visibility of ${el.localName}`);
}
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.table.row.no_cells",
      "message": "Row 1 of a row group established by a “tbody” element has no cells beginning on it.",
      "severity": "Warning",
      "span": {
        "byte_end": 515,
        "byte_start": 507,
        "col": 19,
        "line": 13
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/rendering/non-replaced-elements/tables/hidden-attr.html"
}
```
