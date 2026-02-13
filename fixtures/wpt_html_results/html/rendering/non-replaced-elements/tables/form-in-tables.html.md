# html/rendering/non-replaced-elements/tables/form-in-tables.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/tables/form-in-tables.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>UA style for form in table elements</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#tables-2">
<link rel="author" title="Rune Lillesveen" href="mailto:futhark@chromium.org">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<style>
  .block { display: block !important }
</style>
<div id="display">
  <table><form></form></table>
  <table><thead><form></form></thead></table>
  <table><tbody><form></form></tbody></table>
  <table><tfoot><form></form></tfoot></table>
  <table><tr><form></form></tr></table>
</div>
<div id="important">
  <table><form class="block"></form></table>
  <table><thead><form class="block"></form></thead></table>
  <table><tbody><form class="block"></form></tbody></table>
  <table><tfoot><form class="block"></form></tfoot></table>
  <table><tr><form class="block"></form></tr></table>
</div>
<script>
  for (const form of display.querySelectorAll("form")) {
    test(function() {
      assert_equals(getComputedStyle(form).display, "none");
    }, `Computed display of form inside ${form.parentNode.nodeName} should be 'none'`);
  }
  for (const form of important.querySelectorAll("form")) {
    test(function() {
      assert_equals(getComputedStyle(form).display, "none");
    }, `Computed display of form inside ${form.parentNode.nodeName} should be 'none' (!important UA style))`);
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
        "byte_end": 623,
        "byte_start": 615,
        "col": 32,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.table.row.no_cells",
      "message": "Row 1 of a row group established by a “tbody” element has no cells beginning on it.",
      "severity": "Warning",
      "span": {
        "byte_end": 930,
        "byte_start": 922,
        "col": 46,
        "line": 22
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
  "source_name": "html/rendering/non-replaced-elements/tables/form-in-tables.html"
}
```
