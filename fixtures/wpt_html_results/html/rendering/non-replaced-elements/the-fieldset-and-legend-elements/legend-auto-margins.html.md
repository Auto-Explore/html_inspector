# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-auto-margins.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-auto-margins.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<title>legend inline auto margins</title>
<link rel="author" title="Mats Palmgren" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1488301">
<link rel="match" href="legend-auto-margins-ref.html">
<style>
body, html { padding:0; margin: 0; }
fieldset {
  border: 1px solid black;
  border-width: 10px 17px 7px 23px;
  padding: 0 17px 0 31px;
  margin: 0;
  width: 400px;
}
legend {
  width: 200px;
  height: 20px;
  padding: 0;
  margin: 0;
  background: grey;
}
</style>
</head>
<body>
  <fieldset><legend style="margin-left:auto"></legend></fieldset>
  <fieldset><legend style="margin-right:auto"></legend></fieldset>
  <fieldset><legend style="margin:0 auto"></legend></fieldset>
  <fieldset><legend style="margin:0 15px 0 auto"></legend></fieldset>
  <fieldset><legend style="margin:0 auto 0 15px"></legend></fieldset>
</body>
</html>
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-auto-margins.html"
}
```
