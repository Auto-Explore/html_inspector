# html/editing/dnd/draggable-areas/border-radius.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/draggable-areas/border-radius.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPe html>
<meta charset='utf-8'>
<title>drag and drop – draggable area boundaries, border-radius</title>
<style>
a {
  display: block;
  height: 200px;
  width: 200px;
  background-color: blue;
  border-radius: 100px;
}
div {
  border: 1px solid black;
  height: 200px;
  width: 200px;
}
</style>

<ol>
 <li>Try dragging the white area within the black square, outside the blue
 circle. It should <em>not</em> be draggable.</li>
 <li>Drag the blue circle below. It should be draggable.</li>
</ol>

<div><a draggable="true" ondragstart="event.dataTransfer.effectAllowed ='copy'"></a></div>
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
  "source_name": "html/editing/dnd/draggable-areas/border-radius.html"
}
```
