# html/browsers/browsing-the-web/unloading-documents/unload/pagehide-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/unloading-documents/unload/pagehide-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Document salvagable state after setting pagehide handler</title>
<script>onpagehide = function() {setTimeout(function(){document.body.innerHTML = "PASS"}, 100)}</script>
<p>Click the link below then navigate back to this page. Shortly after returning you should see the text "PASS"</p>
<p><a href="pagehide-manual-1.html">Click here</a>
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
  "source_name": "html/browsers/browsing-the-web/unloading-documents/unload/pagehide-manual.html"
}
```
