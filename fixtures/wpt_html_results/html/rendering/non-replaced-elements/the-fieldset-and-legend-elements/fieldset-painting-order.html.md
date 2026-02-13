# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-painting-order.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-painting-order.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>fieldset painting order</title>
<link rel=match href=fieldset-painting-order-ref.html>
<style>
fieldset, legend { margin: 0; padding: 0; }
fieldset {
  border: 100px solid red;
  width: 0;
  min-width: 0;
  height: 0;
}
legend { width: 200px; height: 200px; margin-left: -100px; background: green; }
legend > span { float: right; margin-top: 100px; width: 100px; height: 100px; background: red; }
fieldset > div { margin-top: -100px; background: lime; width: 200px; height: 200px; }
</style>
<p>There should be no red.</p>
<fieldset><legend><span></span></legend><div></div></fieldset>
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/fieldset-painting-order.html"
}
```
