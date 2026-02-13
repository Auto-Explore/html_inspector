# html/semantics/grouping-content/the-ol-element/reversed-1d.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/grouping-content/the-ol-element/reversed-1d.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset=utf-8>
<title>Reverse numbering should not be affected by nested div</title>
<link rel=match href="reversed-1-ref.html">
<link rel=help href="https://html.spec.whatwg.org/#attr-ol-reversed">
<ol reversed>
  <li>Three</li>
  <div>
      <li>Two</li>
      <li>One</li>
  </div>
</ol>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 270,
        "byte_start": 266,
        "col": 7,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 289,
        "byte_start": 285,
        "col": 7,
        "line": 10
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
  "source_name": "html/semantics/grouping-content/the-ol-element/reversed-1d.html"
}
```
