# html/canvas/element/manual/filters/svg-filter-crash.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/element/manual/filters/svg-filter-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<script>
  document.addEventListener('DOMContentLoaded', async () => {
    const svg = document.createElementNS('http://www.w3.org/2000/svg', 'svg')
    const filter = document.createElementNS('http://www.w3.org/2000/svg', 'filter')
    filter.setAttribute('id', 'id_0')
    svg.appendChild(filter)
    document.documentElement.appendChild(svg)
    const canvas = document.createElementNS('http://www.w3.org/1999/xhtml', 'canvas')
    const context = canvas.getContext('2d')
    context.filter = 'url(#id_0) url(#id_0) sepia() url(#id_0)'
    svg.setAttribute('style', 'display: inline-block;')
  })
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
  "source_name": "html/canvas/element/manual/filters/svg-filter-crash.html"
}
```
