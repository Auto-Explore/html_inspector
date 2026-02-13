# html/semantics/forms/the-input-element/range-tick-marks-05-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/range-tick-marks-05-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>no range tick marks for range tick marks that are step mismatches reference</title>
<input type=range step=3 value=1 min=-5 max=5 list=degrees>
<datalist id=degrees>
  <option value=-2>
  <option value=4>
</datalist>
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
  "source_name": "html/semantics/forms/the-input-element/range-tick-marks-05-ref.html"
}
```
