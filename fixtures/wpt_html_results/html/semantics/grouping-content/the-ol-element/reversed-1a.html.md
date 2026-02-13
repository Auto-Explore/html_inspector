# html/semantics/grouping-content/the-ol-element/reversed-1a.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/grouping-content/the-ol-element/reversed-1a.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>`reversed` should reverse the numbering correctly</title>
<link rel=match href="reversed-1-ref.html">
<link rel=help href="https://html.spec.whatwg.org/#attr-ol-reversed">
<ol reversed>
  <li>Three</li>
  <li>Two</li>
  <li>One</li>
</ol>
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
  "source_name": "html/semantics/grouping-content/the-ol-element/reversed-1a.html"
}
```
