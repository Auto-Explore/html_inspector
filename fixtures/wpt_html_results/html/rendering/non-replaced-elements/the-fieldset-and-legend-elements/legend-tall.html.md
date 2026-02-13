# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-tall.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-tall.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>tall legend</title>
<link rel=match href=legend-tall-ref.html>
<style>
body, p { margin: 0; }
fieldset { height: 30px; margin: 0; border: 20px solid green; padding: 0px; background: red; }
legend { height: 160px; background: lime; padding: 0; }
#behind {
  position: absolute;
  z-index: -1;
  margin-top: calc((/* half legend height */ 160px / 2) - (/* half top border */ 20px / 2));
  background: red;
  height: 40px;
  left: 0;
  right: 0;
}
</style>
<p>There should be no red.</p>
<div id=behind></div>
<fieldset><legend>X</legend>HELLO</fieldset>
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-tall.html"
}
```
