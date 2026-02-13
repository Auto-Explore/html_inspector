# html/syntax/serializing-html-fragments/range-on-pi-contextual-fragment-crash.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/syntax/serializing-html-fragments/range-on-pi-contextual-fragment-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
    <title>createContextualFragment should not crash when the end node is a processing instruction</title>
    <link rel="author" title="Simon Wülker" href="mailto:simon.wuelker@arcor.de">
    <link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-range-createcontextualfragment">
    <link rel="help" href="https://github.com/servo/servo/issues/40719">
</head>
<body>
    <script>
      let pi = document.createProcessingInstruction("xml-stylesheet", 'href="mycss.css" type="text/css"');
      let range = document.createRange();
      range.setEnd(pi, 0)
      range.createContextualFragment("<div>A</div>");
    </script>
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
  "source_name": "html/syntax/serializing-html-fragments/range-on-pi-contextual-fragment-crash.html"
}
```
