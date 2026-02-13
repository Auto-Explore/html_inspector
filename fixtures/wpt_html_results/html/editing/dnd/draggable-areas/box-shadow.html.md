# html/editing/dnd/draggable-areas/box-shadow.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/draggable-areas/box-shadow.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPe html>
<meta charset='utf-8'>
<title>drag and drop – draggable areas – box-shadow</title>
<style>
a {
  display: block;
  height: 200px;
  width: 200px;
  background-color: blue;
  box-shadow: 10px 10px 0 orange;
}
</style>

<p>Try dragging the orange area along the sides of the blue box below. It should <em>not</em> be draggable.</p>

<a href='#' draggable="true" ondragstart="event.dataTransfer.effectAllowed ='copy'"></a>
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
  "source_name": "html/editing/dnd/draggable-areas/box-shadow.html"
}
```
