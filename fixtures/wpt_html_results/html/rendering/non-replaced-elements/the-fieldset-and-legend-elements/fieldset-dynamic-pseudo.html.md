# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-dynamic-pseudo.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-dynamic-pseudo.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<!--
Chrome had a crash bug in a case of dynamic addition of pseudo elements.
crbug.com/1242229
-->
<html class="reftest-wait">
<head>
<link rel="match" href="fieldset-dynamic-pseudo-ref.html">
<style>
*::after { content:"after text"; border:3px solid black;}
*::before { content:"before text"; border:3px solid black; }
</style>
</head>
<body>
<fieldset><legend>Legend</legend></fielset>
<script>
document.styleSheets[0].disabled = true;

requestAnimationFrame(() => {
  requestAnimationFrame(() => {
    document.styleSheets[0].disabled = false;
    document.documentElement.className = '';
  });
});
</script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “fielset”.",
      "severity": "Error",
      "span": {
        "byte_end": 404,
        "byte_start": 394,
        "col": 34,
        "line": 15
      }
    },
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-dynamic-pseudo.html"
}
```
