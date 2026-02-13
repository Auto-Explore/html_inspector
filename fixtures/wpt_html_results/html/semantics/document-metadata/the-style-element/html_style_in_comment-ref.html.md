# html/semantics/document-metadata/the-style-element/html_style_in_comment-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-style-element/html_style_in_comment-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>[style] Reference file</title>
<link rel="author" title="Intel" href="http://www.intel.com">
<style>
  h4 {
    color: green;
  }
</style>
<body>
  <p>
    This page tests that Style written inside HTML comment is not applied
  </p>
  This test passes if the text below is <b>Green. NOT Red.</b>
  <h4>
    This is some text.
  </h4>
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
  "source_name": "html/semantics/document-metadata/the-style-element/html_style_in_comment-ref.html"
}
```
