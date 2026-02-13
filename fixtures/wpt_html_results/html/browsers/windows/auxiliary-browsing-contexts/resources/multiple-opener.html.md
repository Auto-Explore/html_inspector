# html/browsers/windows/auxiliary-browsing-contexts/resources/multiple-opener.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/auxiliary-browsing-contexts/resources/multiple-opener.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
<script src="/common/PrefixedLocalStorage.js"></script>
<body onload="openNested()">
<script>
var prefixedLocalStorage = new PrefixedLocalStorageResource({
  close_on_cleanup: true
});
function openNested () {
  // Listen for message from opened context and pass through to this
  // context's opener
  window.addEventListener('message', (e) => {
    if (window.opener) {
      window.opener.postMessage({
        aux2: e.data, // From multipleOpenee
        aux1: { // This context
          name               : window.name,
          openerName         : window.opener.name,
          isTop              : window.top === window
        }
      }, '*');
    }
  });
  var a = document.createElement('a');
  a.target = 'multipleOpenee';
  a.href = prefixedLocalStorage.url('message-window-opener.html');
  document.body.appendChild(a);
  a.click();
}
</script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/browsers/windows/auxiliary-browsing-contexts/resources/multiple-opener.html"
}
```
