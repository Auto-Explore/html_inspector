# html/browsers/windows/auxiliary-browsing-contexts/resources/no-opener.html

Counts:
- errors: 2
- warnings: 0
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/auxiliary-browsing-contexts/resources/no-opener.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<html>
<p>This window should have no opener.</p>
<script src="/common/PrefixedLocalStorage.js"></script>
<script>
var prefixedLocalStorage = new PrefixedLocalStorageResource({
  close_on_cleanup: true
});
function checkOpener () {
  return prefixedLocalStorage.setItem('openerIsNull', window.opener === null);
}
</script>
<body onload="checkOpener()">
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.body.already_open",
      "message": "Start tag “body” seen but an element of the same type was already open.",
      "severity": "Error",
      "span": {
        "byte_end": 390,
        "byte_start": 361,
        "col": 1,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.parser.cannot_recover",
      "message": "Cannot recover after last error. Any further errors will be ignored.",
      "severity": "Error",
      "span": {
        "byte_end": 390,
        "byte_start": 361,
        "col": 1,
        "line": 14
      }
    }
  ],
  "source_name": "html/browsers/windows/auxiliary-browsing-contexts/resources/no-opener.html"
}
```
