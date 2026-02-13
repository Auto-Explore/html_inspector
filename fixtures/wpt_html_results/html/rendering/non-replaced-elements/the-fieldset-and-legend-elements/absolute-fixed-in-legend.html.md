# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/absolute-fixed-in-legend.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/absolute-fixed-in-legend.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Absolute/fixed-positioned boxes in LEGEND should be painted on the fieldset content</title>
<link rel=match href=absolute-fixed-in-legend-ref.html>
<style>
.absolute-container {
  position: relative;
  border: none;
  padding: 0;
  margin: 0;
}

.absolute-container .legend-content {
  display: block;
  font-size: 32px;
  position: absolute;
  left: 0px;
  background: lime;
  width: 10em;
}

.fixed-container {
  contain: paint;
  border: none;
  padding: 0;
  margin: 0;
}

.fixed-container .legend-content {
  display: block;
  font-size: 32px;
  position: fixed;
  left: 0px;
  background: lime;
  width: 10em;
}

.fieldset-content {
  background: red;
  font-size: 32px;
  width: 10em;
}
</style>

<fieldset class="absolute-container">
  <legend><span class="legend-content">legend</span></legend>
  <div class="fieldset-content">content</div>
</fieldset>

<fieldset class="fixed-container">
  <legend><span class="legend-content">legend</span></legend>
  <div class="fieldset-content">content</div>
</fieldset>
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/absolute-fixed-in-legend.html"
}
```
