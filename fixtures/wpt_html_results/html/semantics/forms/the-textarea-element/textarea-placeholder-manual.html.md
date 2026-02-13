# html/semantics/forms/the-textarea-element/textarea-placeholder-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-textarea-element/textarea-placeholder-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: textarea - placeholder attribute</title>
<link rel="author" title="Intel" href="http://www.intel.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#attr-textarea-placeholder">
<meta name="flags" content="interact">
<body>
  <p>
    Test passes if there is a "Placeholder Text" in the text area,
    and if the "Placeholder Text" disappears after type in any character.
  </p>
  <textarea placeholder="Placeholder Text"></textarea>
</body>

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
  "source_name": "html/semantics/forms/the-textarea-element/textarea-placeholder-manual.html"
}
```
