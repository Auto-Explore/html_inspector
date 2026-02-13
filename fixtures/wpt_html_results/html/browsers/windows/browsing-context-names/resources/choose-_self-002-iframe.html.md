# html/browsers/windows/browsing-context-names/resources/choose-_self-002-iframe.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/browsing-context-names/resources/choose-_self-002-iframe.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: browsing context name - self (case-insensitive)</title>
<link rel="author" title="Intel" href="http://www.intel.com/">
<script src="/common/PrefixedLocalStorage.js"></script>
<script>
var prefixedStorage = new PrefixedLocalStorageResource({
  close_on_cleanup: true
});
var win = window.open(prefixedStorage.url('report-is-top.html'), '_sElF');
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
  "source_name": "html/browsers/windows/browsing-context-names/resources/choose-_self-002-iframe.html"
}
```
