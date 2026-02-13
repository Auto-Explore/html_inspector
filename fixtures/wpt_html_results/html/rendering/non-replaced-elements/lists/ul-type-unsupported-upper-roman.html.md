# html/rendering/non-replaced-elements/lists/ul-type-unsupported-upper-roman.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/lists/ul-type-unsupported-upper-roman.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>ul@type: unsupported type: upper-roman</title>
<link rel=match href=ul-type-unsupported-ref.html>
<ul type=upper-roman><li>first item</li><li>second item</li></ul>
<ul type=UPPER-ROMAN><li>first item</li><li>second item</li></ul>
<ul type=I><li>first item</li><li>second item</li></ul>
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
  "source_name": "html/rendering/non-replaced-elements/lists/ul-type-unsupported-upper-roman.html"
}
```
