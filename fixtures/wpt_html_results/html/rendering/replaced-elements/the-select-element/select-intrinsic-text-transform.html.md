# html/rendering/replaced-elements/the-select-element/select-intrinsic-text-transform.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/the-select-element/select-intrinsic-text-transform.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>text-transform in option doesn't affect combobox rendering</title>
<link rel=match href=select-intrinsic-text-transform-ref.html>
<link rel=help href="https://bugs.chromium.org/p/chromium/issues/detail?id=1283930">
<link rel=author href="mailto:emilio@crisal.io" title="Emilio Cobos Álvarez">
<select>
  <option style="text-transform: lowercase">ABCDEFGHIJK</option>
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
  "source_name": "html/rendering/replaced-elements/the-select-element/select-intrinsic-text-transform.html"
}
```
