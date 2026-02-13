# html/semantics/forms/the-input-element/range-tick-marks-01-notref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/range-tick-marks-01-notref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>LTR range input with datalist reference</title>
<input type="range" min="-100" max="100" value="0" step="10" name="power" list="powers">
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.input.list.must_refer_to_datalist",
      "message": "The “list” attribute of the “input” element must refer to a “datalist” element.",
      "severity": "Warning",
      "span": {
        "byte_end": 159,
        "byte_start": 71,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/forms/the-input-element/range-tick-marks-01-notref.html"
}
```
