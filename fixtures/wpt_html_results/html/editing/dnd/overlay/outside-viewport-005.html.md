# html/editing/dnd/overlay/outside-viewport-005.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/overlay/outside-viewport-005.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPe html>
<meta charset='utf-8'>
<title>drag and drop – feedback overlay of elements partly outside the viewport – 005</title>
<style>
div {
  display: block;
  height: 200px;
  width: 200px;
  background-color: blue;
  position: absolute;
  top: 10px;
  right: -100px;
}

p {
 margin-right: 200px;
}
</style>

<div draggable="true" ondragstart="event.dataTransfer.effectAllowed ='copy'"></div>

<p>Drag the blue box on the right. The drag placeholder should ideally be a blue square. It may optionally be a rectangle the same shape as the visible part of the blue box. No part of the UI should be dragged with the box.</p>
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
  "source_name": "html/editing/dnd/overlay/outside-viewport-005.html"
}
```
