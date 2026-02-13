# html/rendering/widgets/input-date-baseline-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/input-date-baseline-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Test reference</title>
<link rel="stylesheet" href="/fonts/ahem.css">
<style>
  div {
    border: 1px solid black;
    line-height: 0;
  }
  span {
    font: 20px/1 Ahem;
    visibility: hidden;
  }
</style>
<div>
  <span>A</span>
</div>
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
  "source_name": "html/rendering/widgets/input-date-baseline-ref.html"
}
```
