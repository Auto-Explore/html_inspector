# html/meta/resources/refresh1.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/meta/resources/refresh1.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>refresh 1</title>

<meta http-equiv="refresh" content="1;url=gotRefreshed.html">
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.meta.refresh.invalid",
      "message": "Bad value “1;url=gotRefreshed.html” for attribute “content” on element “meta”.",
      "severity": "Warning",
      "span": {
        "byte_end": 126,
        "byte_start": 65,
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
  "source_name": "html/meta/resources/refresh1.html"
}
```
