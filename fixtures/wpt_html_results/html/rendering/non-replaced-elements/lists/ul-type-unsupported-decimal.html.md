# html/rendering/non-replaced-elements/lists/ul-type-unsupported-decimal.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/lists/ul-type-unsupported-decimal.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>ul@type: unsupported type: decimal</title>
<link rel=match href=ul-type-unsupported-ref.html>
<ul type=decimal><li>first item</li><li>second item</li></ul>
<ul type=DECIMAL><li>first item</li><li>second item</li></ul>
<ul type=1><li>first item</li><li>second item</li></ul>
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
  "source_name": "html/rendering/non-replaced-elements/lists/ul-type-unsupported-decimal.html"
}
```
