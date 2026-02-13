# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-baseline.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-baseline.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="help" href="https://bugs.chromium.org/p/chromium/issues/detail?id=1305890">
<link rel="match" href="fieldset-baseline-ref.html">
<style>
fieldset {
  border: solid 2px;
  padding: 10px;
  margin: 5px;
}
</style>
<div>
  text <fieldset style="display: inline-block;">line1<br>line2</fieldset>
</div>
<div>
  text <fieldset style="display: inline-flex;">line1<br>line2</fieldset>
</div>
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-baseline.html"
}
```
