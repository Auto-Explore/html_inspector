# html/rendering/non-replaced-elements/tables/table-row-pagination-001-print-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/tables/table-row-pagination-001-print-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<html>
<head>
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
div { height:160%; }
p { height: 50%; margin:0; }
</style>
</head>
<body>

<div>
<p></p>
<p>1</p>
</div>


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
        "byte_end": 109,
        "byte_start": 86,
        "col": 1,
        "line": 6
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
  "source_name": "html/rendering/non-replaced-elements/tables/table-row-pagination-001-print-ref.html"
}
```
