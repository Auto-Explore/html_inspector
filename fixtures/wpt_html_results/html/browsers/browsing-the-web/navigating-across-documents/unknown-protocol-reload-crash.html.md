# html/browsers/browsing-the-web/navigating-across-documents/unknown-protocol-reload-crash.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/navigating-across-documents/unknown-protocol-reload-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html class="test-wait">
  <meta charset="utf-8">
  <link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1921972">
  <link rel="author" title="Emilio Cobos Álvarez" href="mailto:emilio@crisal.io">
  <link rel="author" title="Mozilla" href="https://mozilla.org">
  <!-- frame removes test-wait -->
  <iframe src="resources/unknown-protocol-reload-crash-frame.html"></iframe>
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
  "source_name": "html/browsers/browsing-the-web/navigating-across-documents/unknown-protocol-reload-crash.html"
}
```
