# html/rendering/the-details-element/summary-text-decoration.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/the-details-element/summary-text-decoration.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Rendering of summary element with text-decoration</title>
<link rel="match" href="summary-text-decoration-ref.html">
<link rel="help" href="https://html.spec.whatwg.org/multipage/rendering.html#the-details-and-summary-elements">
<meta name="assert" content="Checks that text-decoration applies to rendered summary element.">
<style>
  summary { text-decoration: underline; }
</style>
<details>
  <summary>This text should be underlined.</summary>
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
  "source_name": "html/rendering/the-details-element/summary-text-decoration.html"
}
```
