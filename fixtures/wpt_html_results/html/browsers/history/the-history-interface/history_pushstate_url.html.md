# html/browsers/history/the-history-interface/history_pushstate_url.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-history-interface/history_pushstate_url.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
<head>
<meta charset="utf-8">
<title>History pushState sets the url</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
</head>
<body>
<div id="log"></div>
<script>
async_test(function(t) {
    var oldLocation = window.location.toString();
    window.history.pushState(null, "", "#hash");
    assert_equals(oldLocation + "#hash", window.location.toString(), "pushState updates url");
    history.back();
    window.onhashchange = () => {
        assert_equals(oldLocation, window.location.toString(), 'history traversal restores old url');
        t.done();
    };
}, "history pushState sets url");
</script>
</body>
</html>
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
  "source_name": "html/browsers/history/the-history-interface/history_pushstate_url.html"
}
```
