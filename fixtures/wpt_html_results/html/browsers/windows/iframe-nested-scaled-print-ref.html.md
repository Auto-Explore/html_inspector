# html/browsers/windows/iframe-nested-scaled-print-ref.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/iframe-nested-scaled-print-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<style>
  body { margin: 0 }
  div {
    transform-origin: top left;
    transform: scale(2);
  }
  iframe {
    width: 100px;
    height: 50px;
  }
</style>
<div>
<iframe frameborder=0 scrolling=no src="resources/iframe-nested-printing-pass.html"></iframe>
</div>
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
  "source_name": "html/browsers/windows/iframe-nested-scaled-print-ref.html"
}
```
