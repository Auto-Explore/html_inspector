# html/rendering/widgets/the-select-element/select-invalidation-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/widgets/the-select-element/select-invalidation-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Select rendering invalidation</title>
<link rel="author" href="mailto:masonf@chromium.org">

<style>
  select {
    color: lime;
  }
</style>

<select id=select>
  <option>The down arrow should be green</option>
  <option>value B</option>
</select>
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
  "source_name": "html/rendering/widgets/the-select-element/select-invalidation-ref.html"
}
```
