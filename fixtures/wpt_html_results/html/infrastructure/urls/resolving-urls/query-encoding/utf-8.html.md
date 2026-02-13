# html/infrastructure/urls/resolving-urls/query-encoding/utf-8.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/infrastructure/urls/resolving-urls/query-encoding/utf-8.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Resolving URLs, URL character encoding, utf-8</title>
<meta name="timeout" content="long">
<meta name="variant" content="?include=nested-browsing">
<meta name="variant" content="?include=loading">
<meta name="variant" content="?include=submit">
<meta name="variant" content="?include=base-href">
<meta name="variant" content="?include=workers">
<meta name="variant" content="?include=eventsource">
<meta name="variant" content="?include=window-open">
<meta name="variant" content="?include=hyperlink-search">
<meta name="variant" content="?include=history">
<meta name="variant" content="?include=svg">
<meta name="variant" content="?include=xhr">
<meta name="variant" content="?include=websocket">
<meta name="variant" content="?include=css">
<meta name="variant" content="?include=xml">
<meta name="variant" content="?include=url">
<meta name="variant" content="?include=scheme">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/subset-tests-by-key.js"></script>
<script src="/common/utils.js"></script>
<div id=log></div>
<script src="resources/resolve-url.js?encoding=utf-8&pipe=sub"></script>
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
  "source_name": "html/infrastructure/urls/resolving-urls/query-encoding/utf-8.html"
}
```
