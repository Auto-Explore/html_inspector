# html/rendering/non-replaced-elements/tables/form-in-tables-xhtml.xhtml

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/tables/form-in-tables-xhtml.xhtml",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<html xmlns="http://www.w3.org/1999/xhtml">
  <head>
    <title>UA style for form in table elements - XHTML</title>
    <link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#tables-2" />
    <link rel="author" title="Rune Lillesveen" href="mailto:futhark@chromium.org" />
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
  </head>
  <body>
    <table><form></form></table>
    <table><thead><form></form></thead></table>
    <table><tbody><form></form></tbody></table>
    <table><tfoot><form></form></tfoot></table>
    <table><tr><form></form></tr></table>
    <script>
      for (const form of document.querySelectorAll("form")) {
        test(function() {
          assert_equals(getComputedStyle(form).display, "block");
        }, `Computed display of form inside ${form.parentNode.nodeName} in xhtml should be 'block'`);
      }
    </script>
  </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.table.row.no_cells",
      "message": "Row 1 of an implicit row group has no cells beginning on it.",
      "severity": "Warning",
      "span": {
        "byte_end": 646,
        "byte_start": 638,
        "col": 34,
        "line": 14
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
  "source_name": "html/rendering/non-replaced-elements/tables/form-in-tables-xhtml.xhtml"
}
```
