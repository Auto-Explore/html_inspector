# html/rendering/the-details-element/summary-display-list-item-001-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/the-details-element/summary-display-list-item-001-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>CSS Reference: summary with 'display: list-item'</title>
<link rel="author" title="Oriol Brufau" href="mailto:obrufau@igalia.com">
<style>
details {
  margin-left: 50px;
}
.inside {
  list-style-position: inside;
}
</style>
<details>
  <summary>summary</summary>
  content
</details>
<details>
  <summary class="inside">summary</summary>
  content
</details>
<details open>
  <summary>summary</summary>
  content
</details>
<details open>
  <summary class="inside">summary</summary>
  content
</details>
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
  "source_name": "html/rendering/the-details-element/summary-display-list-item-001-ref.html"
}
```
