# html/rendering/non-replaced-elements/the-frameset-and-frame-elements/multicol-table-crash.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/non-replaced-elements/the-frameset-and-frame-elements/multicol-table-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="help" href="http://crbug.com/1383361">
<body style="columns:2">
<div id="parent" style="display:table-caption"></div>
<script>
const caption = document.querySelector('#parent');
const frameset = caption.appendChild(document.createElement('frameset'));
frameset.setAttribute('rows', '100%');
frameset.setAttribute('cols', '100%');
frameset.innerHTML = '<frame srcdoc=""></frame><frame srcdoc=""></frame>';
</script>
</body>
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
  "source_name": "html/rendering/non-replaced-elements/the-frameset-and-frame-elements/multicol-table-crash.html"
}
```
