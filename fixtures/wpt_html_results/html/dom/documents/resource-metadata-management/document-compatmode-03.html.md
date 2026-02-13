# html/dom/documents/resource-metadata-management/document-compatmode-03.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/documents/resource-metadata-management/document-compatmode-03.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<title>document.compatMode: Quirks</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-document-compatmode">
<body>
<div id="log"></div>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
test(function() {
  assert_equals(document.compatMode, "BackCompat");
})
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.start_tag_without_doctype",
      "message": "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 7,
        "byte_start": 0,
        "col": 1,
        "line": 1
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
  "source_name": "html/dom/documents/resource-metadata-management/document-compatmode-03.html"
}
```
