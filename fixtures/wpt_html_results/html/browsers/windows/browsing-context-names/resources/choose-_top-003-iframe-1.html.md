# html/browsers/windows/browsing-context-names/resources/choose-_top-003-iframe-1.html

Counts:
- errors: 3
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/browsing-context-names/resources/choose-_top-003-iframe-1.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: browsing context name - top (case-insensitive)</title>
<script src="/common/PrefixedLocalStorage.js"></script>
<body>
</body>
<script>
var prefixedStorage = new PrefixedLocalStorageResource({
  close_on_cleanup: true
});
window.name = 'topWin';
var iframe = document.createElement('iframe');
iframe.src = prefixedStorage.url('choose-_top-003-iframe-2.html');
document.body.appendChild(iframe);
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_start_tag",
      "message": "Stray start tag “script”.",
      "severity": "Error",
      "span": {
        "byte_end": 191,
        "byte_start": 183,
        "col": 1,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 451,
        "byte_start": 191,
        "col": 9,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 460,
        "byte_start": 451,
        "col": 1,
        "line": 15
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/browsers/windows/browsing-context-names/resources/choose-_top-003-iframe-1.html"
}
```
