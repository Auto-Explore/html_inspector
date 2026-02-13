# html/browsers/browsing-the-web/unloading-documents/support/beforeunload-sticky-start.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/unloading-documents/support/beforeunload-sticky-start.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Beforeunload must be gated behind sticky activation: start page</title>

<p>This page will immediately navigate. If a beforeunload dialog pops up, the test fails.</p>

<script>
window.onbeforeunload = e => e.preventDefault();
location.href = 'beforeunload-sticky-destination.html';
</script>
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
  "source_name": "html/browsers/browsing-the-web/unloading-documents/support/beforeunload-sticky-start.html"
}
```
