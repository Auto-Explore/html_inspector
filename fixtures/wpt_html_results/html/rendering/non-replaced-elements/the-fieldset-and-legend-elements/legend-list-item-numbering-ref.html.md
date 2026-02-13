# html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-list-item-numbering-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-list-item-numbering-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Reference for legend and display: list-item numbering</title>
<link rel="author" title="Ting-Yu Lin" href="mailto:tlin@mozilla.com">
<link rel="author" title="Mozilla" href="http://www.mozilla.org/">

<style>
ol { margin: 0; padding: 0; border: none; }
ol > * { margin: 0 40px; padding: 0; }
</style>

<ol>
  <li value="2">B</li>
  <li value="1">A</li>
  <li value="3">C</li>
</ol>
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
  "source_name": "html/rendering/non-replaced-elements/the-fieldset-and-legend-elements/legend-list-item-numbering-ref.html"
}
```
