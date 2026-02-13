# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-auto-margins-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-auto-margins-ref.html",
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
<style>
body, html { padding:0; margin: 0; }
div {
  border: 1px solid black;
  border-width: 10px 17px 7px 23px;
  padding: 0;
  margin: 0;
  width: 448px;
  height: 5px;
  margin-top: 5px;
  position: relative;
}
span {
  position: absolute;
  top: -15px;
  width: 200px;
  height: 20px;
  padding: 0;
  margin: 0;
  background: grey;
}
center { width: 200px; height: 20px; background: red; }
</style>
</head>
<body>
  <div><span style="right:17px"></span></div>
  <div><span style="left:31px"></span></div>
  <div><span style="left:131px"></span></div>
  <div><span style="right:32px"></span></div>
  <div><span style="left:46px"></span></div>
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-auto-margins-ref.html"
}
```
