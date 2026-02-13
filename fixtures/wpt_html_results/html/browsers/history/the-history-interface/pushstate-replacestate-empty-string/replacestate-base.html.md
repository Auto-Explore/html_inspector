# html/browsers/history/the-history-interface/pushstate-replacestate-empty-string/replacestate-base.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-history-interface/pushstate-replacestate-empty-string/replacestate-base.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>history.replaceState() with an empty string URL and base URL different from document's URL</title>
<link rel="help" href="https://github.com/whatwg/html/issues/9343">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<base href="/the-base">

<script>
"use strict";

test(() => {
  const before = location.pathname;

  history.replaceState(null, null, "");
  assert_equals(location.pathname, before);
});
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.base.must_come_before_link_or_script",
      "message": "The “base” element must come before any “link” or “script” elements in the document.",
      "severity": "Warning",
      "span": {
        "byte_end": 342,
        "byte_start": 319,
        "col": 1,
        "line": 7
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
  "source_name": "html/browsers/history/the-history-interface/pushstate-replacestate-empty-string/replacestate-base.html"
}
```
