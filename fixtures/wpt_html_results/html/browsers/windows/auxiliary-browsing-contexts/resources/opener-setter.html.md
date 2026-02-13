# html/browsers/windows/auxiliary-browsing-contexts/resources/opener-setter.html

Counts:
- errors: 2
- warnings: 0
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/auxiliary-browsing-contexts/resources/opener-setter.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<html>
<p>This window should set the window.opener attribute</p>
<script src="/common/PrefixedLocalStorage.js"></script>
<script>
var prefixedLocalStorage = new PrefixedLocalStorageResource({
  close_on_cleanup: true
});
function checkOpener () {
  if (window.name == 'iShouldSetOpenerToNull') {
    window.opener = null;
    return prefixedLocalStorage.setItem('openerIsNull', window.opener === null);
  }
  if (window.name == 'iShouldSetOpenerToTest') {
    window.opener = 'test';
    return prefixedLocalStorage.setItem('openerIsTest', window.opener === "test");
  }
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
        "byte_end": 651,
        "byte_start": 622,
        "col": 1,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.parser.cannot_recover",
      "message": "Cannot recover after last error. Any further errors will be ignored.",
      "severity": "Error",
      "span": {
        "byte_end": 651,
        "byte_start": 622,
        "col": 1,
        "line": 21
      }
    }
  ],
  "source_name": "html/browsers/windows/auxiliary-browsing-contexts/resources/opener-setter.html"
}
```
