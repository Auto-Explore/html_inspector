# html/browsers/windows/browsing-context-names/resources/report-is-top.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/browsing-context-names/resources/report-is-top.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<script src="/common/PrefixedLocalStorage.js"></script>
<script>
var prefixedStorage = new PrefixedLocalStorageResource({
  close_on_cleanup: true
});
prefixedStorage.setItem('isTop', window === window.top);
prefixedStorage.setItem('name', window.name);
</script>
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
  "source_name": "html/browsers/windows/browsing-context-names/resources/report-is-top.html"
}
```
