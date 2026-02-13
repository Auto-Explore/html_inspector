# html/browsers/browsing-the-web/unloading-documents/beforeunload-sticky-activation-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/unloading-documents/beforeunload-sticky-activation-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Beforeunload must be gated behind sticky activation: normal top-level browsing context</title>

<p>This test is manual because we want to test non-popup, non-iframe situations. Sibling files contain automated tests for those situations.

<p>In three seconds, this document will redirect itself to a new page. The test passes if the redirect succeeds. The test fails if a beforeunload dialog pops up asking for confirmation.

<p>Be sure not to interact with any part of the page in the meantime. That would invalidate the results.

<script>
window.onbeforeunload = e => e.preventDefault();

setTimeout(() => {
  location.href = 'support/beforeunload-sticky-destination.html';
}, 3000);
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
  "source_name": "html/browsers/browsing-the-web/unloading-documents/beforeunload-sticky-activation-manual.html"
}
```
