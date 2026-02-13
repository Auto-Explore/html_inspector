# html/browsers/windows/iframe-cross-origin-print.sub.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/iframe-cross-origin-print.sub.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<link rel=match href="iframe-nested-print-ref.html">
<style>
  body { margin: 0 }
</style>
<iframe frameborder=0 scrolling=no src="//{{hosts[alt][www]}}:{{ports[http][0]}}{{location[path]}}/../resources/iframe-nested-cross-origin.html"></iframe>
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
  "source_name": "html/browsers/windows/iframe-cross-origin-print.sub.html"
}
```
