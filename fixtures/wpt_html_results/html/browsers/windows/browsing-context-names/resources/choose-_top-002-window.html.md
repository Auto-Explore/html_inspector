# html/browsers/windows/browsing-context-names/resources/choose-_top-002-window.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/browsing-context-names/resources/choose-_top-002-window.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<script src="/common/PrefixedLocalStorage.js"></script>
<title>HTML Test: browsing context name - _top</title>
<body>
<script>
var prefixedStorage = new PrefixedLocalStorageResource({
  close_on_cleanup:true
});
window.name = 'topWin2';
var iframe = document.createElement('iframe');
iframe.src = prefixedStorage.url('open-in-_top.html');
// Append iframe that will open another document into `_top` (this context)
document.body.appendChild(iframe);
</script>
</body>
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
  "source_name": "html/browsers/windows/browsing-context-names/resources/choose-_top-002-window.html"
}
```
