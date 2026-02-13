# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/second-legend-becomes-rendered-legend-crash.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/second-legend-becomes-rendered-legend-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="author" title="Morten Stenshorne" href="mailto:mstensho@chromium.org">
<link rel="help" href="https://bugs.chromium.org/p/chromium/issues/detail?id=1119400">
<fieldset>
  <legend id="legend1"></legend>
  <legend id="legend2"><div id="child" style="float:left;"></div></legend>
</fieldset>
<script>
  requestAnimationFrame(()=> {
      requestAnimationFrame(()=> {
          legend1.style.display = "none";
          document.body.offsetTop;

          child.style.width = "22px";
          document.body.offsetTop;

          child.style.display = "none";
      });
  });
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/second-legend-becomes-rendered-legend-crash.html"
}
```
