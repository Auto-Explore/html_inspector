# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/insert-legend-in-multicol-fieldset-crash.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/insert-legend-in-multicol-fieldset-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="author" title="Morten Stenshorne" href="mailto:mstensho@chromium.org">
<link rel="help" href="https://bugs.chromium.org/p/chromium/issues/detail?id=1211926">
<fieldset id="fieldset">
  <legend id="legend"></legend>
  <div></div>
</fieldset>
<script>
  document.body.offsetTop;
  fieldset.style.columns = "2";
  legend.style.position = "relative";
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/insert-legend-in-multicol-fieldset-crash.html"
}
```
