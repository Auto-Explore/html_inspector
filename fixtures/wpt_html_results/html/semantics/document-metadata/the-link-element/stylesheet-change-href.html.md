# html/semantics/document-metadata/the-link-element/stylesheet-change-href.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/document-metadata/the-link-element/stylesheet-change-href.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>Obtaining a new stylesheet removes styles from the previous stylesheet.</title>
<link rel=match href=stylesheet-change-href-ref.html>
<script>
  function changeHref() {
    var elem = document.getElementById('stylesheet');
    elem.href = 'resources/good.css';
    elem.onload = null;
  }
</script>
<link id=stylesheet rel=stylesheet href="resources/bad.css" onload="changeHref()">
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
  "source_name": "html/semantics/document-metadata/the-link-element/stylesheet-change-href.html"
}
```
