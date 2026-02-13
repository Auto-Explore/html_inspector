# html/browsers/windows/auxiliary-browsing-contexts/resources/close-opener.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/auxiliary-browsing-contexts/resources/close-opener.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<html>
<body onload="closeOpener()">
<p>This window should close its opener.</p>
<script src="/common/PrefixedLocalStorage.js"></script>
<script>
var prefixedLocalStorage = new PrefixedLocalStorageResource({
  close_on_cleanup: true
});
var prefixedLocalStorage = new PrefixedLocalStorageResource({
  close_on_cleanup: true
});
function closeOpener () {
  if (window.opener) {
    window.opener.close();

    // Give the browsing context a chance to dispose of itself
    function waitForContextDiscard () {
      if (window.opener === null) {
        return prefixedLocalStorage.setItem('openerIsNull', 'true');
      }
      return setTimeout(waitForContextDiscard, 0);
    }
    waitForContextDiscard();
  }
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
  "source_name": "html/browsers/windows/auxiliary-browsing-contexts/resources/close-opener.html"
}
```
