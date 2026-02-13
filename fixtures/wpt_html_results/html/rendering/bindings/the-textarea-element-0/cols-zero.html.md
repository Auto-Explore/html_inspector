# html/rendering/bindings/the-textarea-element-0/cols-zero.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/bindings/the-textarea-element-0/cols-zero.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Textarea cols</title>
<link rel=match href=textarea-ref.html>
<textarea cols=0></textarea>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.textarea.cols.positive",
      "message": "Bad value “0” for attribute “cols” on element “textarea”.",
      "severity": "Warning",
      "span": {
        "byte_end": 123,
        "byte_start": 106,
        "col": 1,
        "line": 5
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
  "source_name": "html/rendering/bindings/the-textarea-element-0/cols-zero.html"
}
```
