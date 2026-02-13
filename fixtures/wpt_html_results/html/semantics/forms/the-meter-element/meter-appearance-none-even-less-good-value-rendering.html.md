# html/semantics/forms/the-meter-element/meter-appearance-none-even-less-good-value-rendering.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-meter-element/meter-appearance-none-even-less-good-value-rendering.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>meter appearance none rendering</title>
<link rel="help" href="https://drafts.csswg.org/css-ui/#valdef-appearance-none">
<link rel="mismatch" href="reference/meter-appearance-auto-even-less-good-value-rendering.html">
<meter value="0.9" low="0.6" high="0.7" style="appearance: none;"></meter>
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
  "source_name": "html/semantics/forms/the-meter-element/meter-appearance-none-even-less-good-value-rendering.html"
}
```
