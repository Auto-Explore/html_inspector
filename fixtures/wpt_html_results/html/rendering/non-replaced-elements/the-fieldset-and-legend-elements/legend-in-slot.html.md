# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-in-slot.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-in-slot.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="match" href="legend-in-slot-ref.html">

<div id="host">
  <legend id="legend">Was rendered legend</legend>
  <legend>Is rendered legend</legend>
</div>
<script>
let root = document.querySelector('#host').attachShadow({mode:"open"});
root.innerHTML = `
<fieldset>
  <slot></slot>
</fieldset>`;
document.body.offsetTop;
document.querySelector('#legend').remove();
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-in-slot.html"
}
```
