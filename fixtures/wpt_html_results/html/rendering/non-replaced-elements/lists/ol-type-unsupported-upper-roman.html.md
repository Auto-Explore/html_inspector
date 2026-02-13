# html/rendering/non-replaced-elements/lists/ol-type-unsupported-upper-roman.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/lists/ol-type-unsupported-upper-roman.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>ol@type: unsupported type: upper-roman</title>
<link rel=match href=ol-type-unsupported-ref.html>
<ol type=upper-roman><li>1<li>2</ol>
<ol type=UPPER-ROMAN><li>1<li>2</ol>
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
  "source_name": "html/rendering/non-replaced-elements/lists/ol-type-unsupported-upper-roman.html"
}
```
