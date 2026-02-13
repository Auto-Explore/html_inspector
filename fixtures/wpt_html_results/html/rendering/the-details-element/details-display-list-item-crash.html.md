# html/rendering/the-details-element/details-display-list-item-crash.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/the-details-element/details-display-list-item-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<link rel="help" href="https://github.com/servo/servo/issues/41231">
<style>
    details {
        display: list-item;
        list-style-image: url('data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg"></svg>');
    }
</style>
</head>
<body>
<!-- This test passes if it does not crash. -->

<details></details>
<script>
    window.reload();
</script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.details.missing_summary",
      "message": "Element “details” is missing a required instance of child element “summary”.",
      "severity": "Warning",
      "span": {
        "byte_end": 370,
        "byte_start": 360,
        "col": 10,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/rendering/the-details-element/details-display-list-item-crash.html"
}
```
