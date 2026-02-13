# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-transform-translatez.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-transform-translatez.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Fieldset and transform: translateZ(0)</title>
<link rel=match href=fieldset-transform-translatez-ref.html>
<style>
#outer { transform: translateZ(0); }
fieldset { background: #eee; overflow: hidden; margin: 0 0 10px; }
#inner { position: relative; }
</style>
<p>It should say PASS below without anything obscuring the text.</p>
<div id=outer>
  <fieldset>
    <legend>
      <div id="inner">PASS</div>
    </legend>
  </fieldset>
</div>
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-transform-translatez.html"
}
```
