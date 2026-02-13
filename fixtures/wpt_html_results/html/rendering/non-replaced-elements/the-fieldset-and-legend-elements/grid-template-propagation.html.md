# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/grid-template-propagation.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/grid-template-propagation.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="match" href="grid-template-propagation-ref.html">
<link rel="help" href="https://bugs.chromium.org/p/chromium/issues/detail?id=1222988">
<link rel="help" href="https://html.spec.whatwg.org/C/#anonymous-fieldset-content-box">
<style>
fieldset {
  display: grid;
  grid-template: auto / 1fr;
  grid-template-areas: "a";
  width: 100px;
  height: 100px;
  margin: 0;
  border: none;
  padding: 0;
}
</style>
<p>There should be a green box below.</p>
<fieldset>
  <div style="background: green; grid-area: a"></div>
</fieldset>
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/grid-template-propagation.html"
}
```
