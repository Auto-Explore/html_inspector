# html/semantics/text-level-semantics/the-bdo-element/bdo-child.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/text-level-semantics/the-bdo-element/bdo-child.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8">
    <title>HTML Test: bdo - text directionality formatting control for its children</title>
    <link rel="author" title="Intel" href="http://www.intel.com/">
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#the-bdo-element">
    <link rel="match" href="bidi-001-ref.html">
    <meta name="assert" content="Check if the bdo element represents explicit text directionality formatting control for its children.">
  </head>
  <body>
    <p>Test passes if there is text 'WERBEH'.</p>
    <bdo dir="rtl">
      <span>HEBREW</span>
    </bdo>
  </body>
</html>
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
  "source_name": "html/semantics/text-level-semantics/the-bdo-element/bdo-child.html"
}
```
