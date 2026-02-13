# html/editing/dnd/overlay/oversized-001.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/overlay/oversized-001.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPe html>
<meta charset='utf-8'>
<title>drag and drop – feedback overlay of oversized element</title>
<style>
html, body, p + p { height: 100%; width: 100%; }
a {
  display: block;
  height: 100%;
  width: 100%;
  background-color: blue;
  border: 50px solid orange; /* makes it higher and wider than the viewport */
}

</style>

<p>Drag the blue box below downwards. The drag placeholder should ideally be a blue rectangle with an orange border on all sides. It may optionally match the visible part of the blue-and-orange box. It may optionally be a rectangle with the same pattern and size as the visible part of the blue-and-orange box. It may optionally be shrunk to a manageable size. No part of the UI should be dragged with the box.</p>

<p><a href='#'></a></p>
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
  "source_name": "html/editing/dnd/overlay/oversized-001.html"
}
```
