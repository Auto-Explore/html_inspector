# html/rendering/non-replaced-elements/form-controls/input-line-height-ref.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/form-controls/input-line-height-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Reference for 'line-height' smaller than 'normal' on input elements as text entry widgets</title>
<style>
  input { font-size: 60px; width: 240px; }
  .appearance-none { appearance: none; }
</style>
<p><input type=text value=text> <input type=text value=text class=appearance-none>
<p><input type=tel value=tel> <input type=tel value=tel class=appearance-none>
<p><input type=search value=search> <input type=search value=search class=appearance-none>
<p><input type=url value=url> <input type=url value=url class=appearance-none>
<p><input type=email value=email> <input type=email value=email class=appearance-none>
<p><input type=password value=password> <input type=password value=password class=appearance-none>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.input.url.value.invalid",
      "message": "Bad value “url” for attribute “value” on element “input”.",
      "severity": "Warning",
      "span": {
        "byte_end": 504,
        "byte_start": 478,
        "col": 4,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.input.url.value.invalid",
      "message": "Bad value “url” for attribute “value” on element “input”.",
      "severity": "Warning",
      "span": {
        "byte_end": 553,
        "byte_start": 505,
        "col": 31,
        "line": 10
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
  "source_name": "html/rendering/non-replaced-elements/form-controls/input-line-height-ref.html"
}
```
