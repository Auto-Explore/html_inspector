# html/browsers/windows/browsing-context-names/resources/post-to-top.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/browsing-context-names/resources/post-to-top.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTML Test: post window's name to top browsing context</title>
<link rel="author" title="Intel" href="http://www.intel.com/">
<script>
top.postMessage({
  name: window.name,
  isTop: (window.top === window)
}, "*");
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
  "source_name": "html/browsers/windows/browsing-context-names/resources/post-to-top.html"
}
```
