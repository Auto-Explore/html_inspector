# html/browsers/windows/auxiliary-browsing-contexts/resources/open-closer.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/auxiliary-browsing-contexts/resources/open-closer.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<html>
<body onload="openAuxiliary()">
<a rel="opener" target="_blank">Open auxiliary context that will close this window (its opener)</a>
<script src="/common/PrefixedLocalStorage.js"></script>
<script>
function openAuxiliary () {
  var prefixedLocalStorage = new PrefixedLocalStorageResource();
  var a = document.body.querySelector('a');
  a.href = prefixedLocalStorage.url('close-opener.html');
  document.body.append(a);
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
  "source_name": "html/browsers/windows/auxiliary-browsing-contexts/resources/open-closer.html"
}
```
