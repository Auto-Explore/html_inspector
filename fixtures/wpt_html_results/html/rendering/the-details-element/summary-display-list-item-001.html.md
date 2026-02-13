# html/rendering/the-details-element/summary-display-list-item-001.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/the-details-element/summary-display-list-item-001.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>CSS Test: summary with 'display: list-item'</title>
<link rel="author" title="Oriol Brufau" href="mailto:obrufau@igalia.com">
<link rel="match" href="summary-display-list-item-001-ref.html">
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#the-details-and-summary-elements">
<meta name="assert" content="Checks that styling a <summary> with 'display: list-item' has no effect since it should already be a list item by default.">
<style>
summary {
  display: list-item;
}
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
  "source_name": "html/rendering/the-details-element/summary-display-list-item-001.html"
}
```
