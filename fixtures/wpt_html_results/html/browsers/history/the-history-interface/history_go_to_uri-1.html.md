# html/browsers/history/the-history-interface/history_go_to_uri-1.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-history-interface/history_go_to_uri-1.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<script src="history.js"></script>
<script>
  onunload = function() {}

  onload = function() {
    if (!opener.started) {
      queue_next();
    } else {
      opener.pages.push(id);
      if (!opener.gone) {
        // This is meant to test that passing a string is not supported.
        // According to the spec, the value passed to 'go' must be an int.
        // Internet Explorer supports passing a string and will navigate
        // to that Url. This test will protect against regressing in
        // this area and reverting back to IE's incorrect behavior.
          history.go("history_entry.html");
          opener.gone = true;
      }
    }
  };
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
  "source_name": "html/browsers/history/the-history-interface/history_go_to_uri-1.html"
}
```
