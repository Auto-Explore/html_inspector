# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-painting-order.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-painting-order.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Legend painting order</title>
<link rel=match href=legend-painting-order-ref.html>

<p>There should be a green square below, and no red.</p>
<div style="float:left; width:0px; height:0px;">
  <div style="width:100px; height:100px; background:red;"></div>
</div>
<fieldset style="margin:0; border:none; padding:0;">
  <legend style="padding:0;">
    <div style="float:left; width:100px; height:100px; background:green;"></div>
  </legend>
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-painting-order.html"
}
```
