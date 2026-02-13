# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-dynamic-baseline.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-dynamic-baseline.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="help" href="https://bugs.chromium.org/p/chromium/issues/detail?id=1307140">
<link rel="match" href="fieldset-dynamic-baseline-ref.html">
baseline
<fieldset style="display: inline-block;">
  <div style="position: relative;">
    line1<br>line2<div id="target" style="position: absolute;"></div>
  </div>
</fieldset>
<script>
document.body.offsetTop;
document.getElementById('target').style.top = '10px';
</script>
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-dynamic-baseline.html"
}
```
