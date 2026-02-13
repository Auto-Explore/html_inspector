# html/editing/dnd/draggable-areas/z-index.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/draggable-areas/z-index.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPe html>
<meta charset='utf-8'>
<title>drag and drop – feedback overlay for heavily styled elements – 008</title>
<style>
a {
  display: block;
  height: 200px;
  width: 200px;
  background-color: blue;
  position: absolute;
  top: 100px;
  left: 10px;
  z-index: 1;
}

div {
  background-color: orange;
  position: absolute;
  height: 200px;
  width: 200px;
  top: 150px;
  left: 20px;
  z-index: 2;
  opacity: 0.9;
}


</style>

<p>Click and hold the part of the orange box that overlaps the blue box. Then
move your pointing device. The blue box should <em>not</em> be dragged.

<a draggable="true" ondragstart="event.dataTransfer.effectAllowed ='copy'">TEST</a>

<div></div>
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
  "source_name": "html/editing/dnd/draggable-areas/z-index.html"
}
```
