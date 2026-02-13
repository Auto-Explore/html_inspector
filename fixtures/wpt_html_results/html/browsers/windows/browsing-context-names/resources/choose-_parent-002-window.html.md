# html/browsers/windows/browsing-context-names/resources/choose-_parent-002-window.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/browsing-context-names/resources/choose-_parent-002-window.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: browsing context name - parent: top-level context</title>
<iframe name="iframeParent" src="choose-_parent-002-iframe.html"></iframe>
<script>
// Relay a message from child context to opener context
window.addEventListener('message', e => {
  if (window.opener) {
    window.opener.postMessage(e.data, '*');
  }
});
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
  "source_name": "html/browsers/windows/browsing-context-names/resources/choose-_parent-002-window.html"
}
```
