# html/browsers/browsing-the-web/unloading-documents/prompt/manual-006.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/unloading-documents/prompt/manual-006.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Prompt when beforeunload returns string value</title>
<script>
addEventListener("beforeunload",
function(e) {return "PASS if you see this"},
false);
</script>
<p>When clicking the link below, you should get a prompt asking if you want to unload the document</p>
<a href="next.html">Click here</a>
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
  "source_name": "html/browsers/browsing-the-web/unloading-documents/prompt/manual-006.html"
}
```
