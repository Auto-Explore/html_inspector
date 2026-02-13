# html/browsers/history/the-history-interface/pushstate-replacestate-empty-string/replacestate-whitespace.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-history-interface/pushstate-replacestate-empty-string/replacestate-whitespace.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>history.replaceState() with a whitespace URL</title>
<link rel="help" href="https://github.com/whatwg/html/issues/9343">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<script>
"use strict";

test(() => {
  location.hash = "test";

  const before = location.pathname;

  history.replaceState(null, null, " ");
  assert_equals(location.pathname, before, "pathname");
  assert_equals(location.hash, "", "hash");
});
</script>
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
  "source_name": "html/browsers/history/the-history-interface/pushstate-replacestate-empty-string/replacestate-whitespace.html"
}
```
