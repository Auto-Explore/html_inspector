# html/semantics/document-metadata/the-style-element/non-parser-created-doc.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-style-element/non-parser-created-doc.html",
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
    <title>CSS API on style element in non-parser-created document</title>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
    <link rel=help href="https://bugzilla.mozilla.org/show_bug.cgi?id=1991800">
  </head>
  <body>
    <script>
      test(function() {
        let doc = document.implementation.createHTMLDocument("");
        let style = doc.createElement("style");
        doc.head.appendChild(style)
        assert_true(!!style.sheet, "The style element should have the sheet property.");
      }, "CSS API should work on non-parser-created documents");
    </script>
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
  "source_name": "html/semantics/document-metadata/the-style-element/non-parser-created-doc.html"
}
```
