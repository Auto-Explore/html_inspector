# html/browsers/history/the-history-interface/traverse_the_history_write_after_load_1-1.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-history-interface/traverse_the_history_write_after_load_1-1.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
2
<script>
  onunload = function() {}
  opener.pages.push(2);
  onload = function() {
    setTimeout(function() {
      document.write("<!doctype html>3<script>opener.pages.push(3); if(!opener.started) {opener.started = true; history.go(-1);}<\/script>");
      document.close();
    }, 100);
  }
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
  "source_name": "html/browsers/history/the-history-interface/traverse_the_history_write_after_load_1-1.html"
}
```
