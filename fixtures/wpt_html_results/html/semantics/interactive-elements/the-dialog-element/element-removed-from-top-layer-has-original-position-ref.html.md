# html/semantics/interactive-elements/the-dialog-element/element-removed-from-top-layer-has-original-position-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/the-dialog-element/element-removed-from-top-layer-has-original-position-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
<head>
<style>
.green {
    color: green;
}
</style>
</head>
<body>
<p>Bug <a href="http://webkit.org/b/106538">106538</a>: Top layer fails for inline elements</p>
<p>This tests that position 'static' no longer computes to 'absolute' for an
element that has been removed from the top layer. The test passes if you see
a single line of text.</p>
<span class="green">This is the span.</span>
<span class="green">This is the dialog following it.</span>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/interactive-elements/the-dialog-element/element-removed-from-top-layer-has-original-position-ref.html"
}
```
