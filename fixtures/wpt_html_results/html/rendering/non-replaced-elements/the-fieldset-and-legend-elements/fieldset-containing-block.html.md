# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-containing-block.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-containing-block.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>fieldset containing block</title>
<link rel=match href=fieldset-containing-block-ref.html>
<style>
p { margin: 0; height: 100px }
fieldset { position: relative; border: none; padding: 0; margin: 0; }
legend { padding: 0; width: 100px; height: 50px; background: lime; }
div { position: absolute; top: 0; width: 100px; height: 50px; background: lime; }
.behind { height:100px; top: 108px; background: red; }

.fixed-container {
  filter: invert();
  overflow: hidden;
  width: 200px;
  height: 200px;
}
.fixed {
  position: fixed;
  width: 400px;
  height: 100px;
  background: linear-gradient(to right, #ff00ff 50%, #00ffff 50%);
}
.has-fixed {
  width: 0px;
  height: 0px;
}
</style>
<p>There should be no red.</p>
<div class="behind"></div>
<fieldset><legend></legend><div></div></fieldset>

<fieldset class="fixed-container">
<legend class="has-fixed"><div style="position:fixed; width:0; height0;"></div></legend>
<div class="fixed"></div>
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-containing-block.html"
}
```
