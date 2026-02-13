# html/semantics/document-metadata/the-link-element/stylesheet-remove-href.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-link-element/stylesheet-remove-href.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>Removing href should remove previously applied styles</title>
<link rel=match href=stylesheet-change-href-ref.html>
<style>
p {
  color: green;
}
</style>
<script>
  function removeHref() {
    var elem = document.getElementById('stylesheet');
    elem.removeAttribute('href');
    elem.onload = null;
  }
</script>
<link id=stylesheet rel=stylesheet href="resources/bad.css" onload="removeHref()">
<p>This text should be green on a white background
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
  "source_name": "html/semantics/document-metadata/the-link-element/stylesheet-remove-href.html"
}
```
