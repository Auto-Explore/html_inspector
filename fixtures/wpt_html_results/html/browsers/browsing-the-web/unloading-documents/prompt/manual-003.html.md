# html/browsers/browsing-the-web/unloading-documents/prompt/manual-003.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/unloading-documents/prompt/manual-003.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Prompt when beforeunload is canceled</title>
<script>
addEventListener("beforeunload",
function(e) {e.preventDefault()},
false);
</script>
<p>When clicking the button below, you should get a prompt asking if you want to unload the document</p>
<form method="get" action="next.html">
<input type="submit" value="Click here">
</form>
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
  "source_name": "html/browsers/browsing-the-web/unloading-documents/prompt/manual-003.html"
}
```
