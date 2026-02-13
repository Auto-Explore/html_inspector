# html/editing/dnd/overlay/heavy-styling-008.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/overlay/heavy-styling-008.html",
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
  background-color: green;
  position: absolute;
  top: 100px;
  left: 10px;
  z-index: 1;
}

a + a {
  background-color: red;
  position: absolute;
  top: 150px;
  left: 20px;
  z-index: 2;
}


</style>

<p>Drag the green box below downwards. The drag placeholder should resemble the green box, including the text within it. It may optionally be a complete square, or the same shape as the visible part of the green box. There should be no red in the drag placeholder.</p>

<a draggable="true" ondragstart="event.dataTransfer.effectAllowed ='copy'">TEST</a>

<a href='#'>TEST</a>
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
  "source_name": "html/editing/dnd/overlay/heavy-styling-008.html"
}
```
