# html/rendering/non-replaced-elements/tables/table-row-pagination-001-print.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/tables/table-row-pagination-001-print.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html>
<head>
<link rel="match" href="table-row-pagination-001-print-ref.html">
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#tables-2">
<link rel="help" href="https://drafts.csswg.org/css-break/">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=685012">
<meta charset="utf-8">
<title>Testing row split</title>
<style type="text/css">
@page { size:5in 3in; margin:0.5in; }
html,body {
  print-color-adjust: exact;
  color:black;
  background-color:white;
  font-size:16px;
  padding:0;
  margin:0;
  height:100%;
}
table { height:160%; width:100%; }
td { height:50%; width:100%; }
</style>
</head>
<body>

<table border="0" cellspacing="0" cellpadding="0">
<tr><td></td></tr>
<tr><td valign="top">1</td></tr>
</table>


</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.type.unnecessary",
      "message": "The “type” attribute for the “style” element is not needed and should be omitted.",
      "severity": "Warning",
      "span": {
        "byte_end": 401,
        "byte_start": 378,
        "col": 1,
        "line": 10
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
  "source_name": "html/rendering/non-replaced-elements/tables/table-row-pagination-001-print.html"
}
```
