# html/semantics/embedded-content/the-embed-element/embed-document.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/embedded-content/the-embed-element/embed-document.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>HTML Test: The embed element represents a document</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<meta name="assert" content="Check if the embed element represents a document when a text/html resource source is used">
<body>
  <script type="application/javascript">
    window.childLoaded = false;
    async_test(function() {
      addEventListener("load", this.step_func_done(function() {
        assert_true(window.childLoaded);
      }));
    }, "Test document type embedding");
  </script>
  <embed src="embed-iframe.html">
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 379,
        "byte_start": 341,
        "col": 3,
        "line": 8
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
  "source_name": "html/semantics/embedded-content/the-embed-element/embed-document.html"
}
```
