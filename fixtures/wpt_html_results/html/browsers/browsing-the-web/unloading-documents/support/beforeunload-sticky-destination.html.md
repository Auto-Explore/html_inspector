# html/browsers/browsing-the-web/unloading-documents/support/beforeunload-sticky-destination.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/unloading-documents/support/beforeunload-sticky-destination.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Beforeunload must be gated behind sticky activation: destination page</title>

<p>If you reached this page without clicking through a confirmation dialog, then the test has passed!

<script>
if (window.opener) {
  window.opener.postMessage('navigated successfully');
} else if (window.parent) {
  window.parent.postMessage('navigated successfully');
}
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
  "source_name": "html/browsers/browsing-the-web/unloading-documents/support/beforeunload-sticky-destination.html"
}
```
