# html/browsers/the-window-object/garbage-collection-and-browsing-contexts/non-automated/discard_iframe_history_2-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-window-object/garbage-collection-and-browsing-contexts/non-automated/discard_iframe_history_2-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Removing iframe from document removes it from history</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
setup({timeout:3600000})
var t = async_test();
var w = window.open("discard_iframe_history_2-1.html");
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
  "source_name": "html/browsers/the-window-object/garbage-collection-and-browsing-contexts/non-automated/discard_iframe_history_2-manual.html"
}
```
