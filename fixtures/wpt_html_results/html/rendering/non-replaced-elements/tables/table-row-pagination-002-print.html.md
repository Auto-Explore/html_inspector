# html/rendering/non-replaced-elements/tables/table-row-pagination-002-print.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/tables/table-row-pagination-002-print.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<link rel="match" href="table-row-pagination-002-print-ref.html">
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#tables-2">
<link rel="help" href="https://drafts.csswg.org/css-break/">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=963441">
<style>
@page { size:5in 3in; margin:0.5in; }
</style>
</head>
<body>
  <div style="float: left">
    <table cellpadding=0 cellspacing=0>
      <tr>
        <td>
          <div style="height:3in">Tall div. (Scroll to end.)</div>
          <div>IS THIS TEXT VISIBLE IN PRINT PREVIEW?</div>
        </td>
      </tr>
    </table>
  </div>
  <div style="clear: left">[clear:left]</div>
</body>
</html>
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
  "source_name": "html/rendering/non-replaced-elements/tables/table-row-pagination-002-print.html"
}
```
