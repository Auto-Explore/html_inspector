# html/rendering/non-replaced-elements/form-controls/select-sizing-001-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/form-controls/select-sizing-001-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
  <meta charset="utf-8">
  <title>Reference for sizing of select elements, with wide vs. empty option selected</title>
  <link rel="author" title="Daniel Holbert" href="mailto:dholbert@mozilla.com">
  <style>
    select {
      color: transparent;
      margin: 1px;
    }
    div.customBorder > select {
      /* This class is to let us test select elements *without* native theming
         (for browsers that have both native and non-native controls): */
      border: 3px solid black;
    }
  </style>
</head>
<body>
  <div>
    <select>
      <option>some wide option</option>
    </select>
    <br>
    <select>
      <option>some wide option</option>
    </select>
    <br>
    <select>
      <option>some wide option</option>
    </select>
    <br>
    <select>
      <option>some wide option</option>
    </select>
  </div>

  <!-- This is the same as above, but now with a custom border on the
       select elements: -->
  <div class="customBorder">
    <select>
      <option>some wide option</option>
    </select>
    <br>
    <select>
      <option>some wide option</option>
    </select>
    <br>
    <select>
      <option>some wide option</option>
    </select>
    <br>
    <select>
      <option>some wide option</option>
    </select>
  </div>

</body>
</html>
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
  "source_name": "html/rendering/non-replaced-elements/form-controls/select-sizing-001-ref.html"
}
```
