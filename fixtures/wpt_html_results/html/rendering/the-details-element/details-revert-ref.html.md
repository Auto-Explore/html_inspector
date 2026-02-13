# html/rendering/the-details-element/details-revert-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/the-details-element/details-revert-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>CSS Test Reference</title>
<style>
  summary {
    display: list-item;
    counter-increment: list-item 0;
    list-style: disclosure-closed inside;
  }
  details[open] > summary {
    list-style-type: disclosure-open;
  }
</style>
<details>
  <summary>Example</summary>
</details>
<details open>
  <summary>Example</summary>
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
  "source_name": "html/rendering/the-details-element/details-revert-ref.html"
}
```
