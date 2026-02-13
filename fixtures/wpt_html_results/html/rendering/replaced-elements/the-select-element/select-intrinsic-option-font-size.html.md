# html/rendering/replaced-elements/the-select-element/select-intrinsic-option-font-size.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/the-select-element/select-intrinsic-option-font-size.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Select should be as wide as needed to fit its options regardless of option styles</title>
<link rel=match href=select-intrinsic-option-font-size-ref.html>
<select>
  <option style="font-size: 5px">ABC</option>
</select>
<select>
  <option style="font-size: 50px">ABC</option>
</select>
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
  "source_name": "html/rendering/replaced-elements/the-select-element/select-intrinsic-option-font-size.html"
}
```
