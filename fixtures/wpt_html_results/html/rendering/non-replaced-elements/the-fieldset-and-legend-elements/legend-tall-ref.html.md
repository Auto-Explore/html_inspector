# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-tall-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-tall-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Reference for tall legend</title>
<style>
body, p { margin: 0; }
.legend { position: absolute; height: 160px; margin-left: 20px; inline-size: fit-content; background: lime }
.fieldset {
  position: absolute;
  z-index: -1;
  margin-top: calc((/* half legend height */ 160px / 2) - (/* half top border */ 20px / 2));
  background: green;
  height: 40px;
  left: 0;
  right: 0;
}
.hello { margin-top: 160px; margin-left: 20px; }
</style>
<p>There should be no red.</p>
<div class=legend>X</div>
<div class=fieldset></div>
<div class=hello>HELLO</div>
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-tall-ref.html"
}
```
