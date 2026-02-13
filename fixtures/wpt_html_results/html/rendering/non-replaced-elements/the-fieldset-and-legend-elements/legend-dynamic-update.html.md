# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-dynamic-update.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-dynamic-update.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html class=reftest-wait>
<title>legend and dynamic update</title>
<link rel=fieldset-foo-ref.html>
<p>There should be a normal fieldset below with the legend "Foo".</p>
<fieldset>
  <legend>F</legend>
</fieldset>
<script>
  const legend = document.querySelector('legend');
  // force layout
  legend.offsetTop;
  requestAnimationFrame(() => {
    legend.textContent += "oo";
    requestAnimationFrame(() => {
      document.documentElement.removeAttribute('class');
    });
  });
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.link.href.required",
      "message": "A “link” element must have an “href” or “imagesrcset” attribute, or both.",
      "severity": "Warning",
      "span": {
        "byte_end": 115,
        "byte_start": 83,
        "col": 1,
        "line": 4
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-dynamic-update.html"
}
```
