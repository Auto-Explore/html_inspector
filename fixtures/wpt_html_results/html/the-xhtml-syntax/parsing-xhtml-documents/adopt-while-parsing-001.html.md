# html/the-xhtml-syntax/parsing-xhtml-documents/adopt-while-parsing-001.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/the-xhtml-syntax/parsing-xhtml-documents/adopt-while-parsing-001.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Appending from the parser after adopting in an XML document doesn't miss notifications</title>
<link rel="match" href="adopt-while-parsing-001-ref.html">
<link rel="help" href="https://bugzilla.mozilla.org/show_bug.cgi?id=1511329">
<link rel="author" title="Emilio Cobos Álvarez" href="mailto:emilio@crisal.io">
<link rel="author" title="Mozilla" href="https://mozilla.org">
<style>
  html, body { margin: 0 }
</style>
<script>
  // If we don't get notified of the <div> insertion, the PASS text will never appear.
  function parsingInterrupted() {
    let frameDoc = document.querySelector("iframe").contentDocument;
    let root = frameDoc.documentElement;
    document.documentElement.appendChild(root);
    root.offsetTop;
  }
</script>
<iframe src="adopt-while-parsing.xhtml"></iframe>
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
  "source_name": "html/the-xhtml-syntax/parsing-xhtml-documents/adopt-while-parsing-001.html"
}
```
