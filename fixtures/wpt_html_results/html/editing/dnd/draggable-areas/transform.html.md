# html/editing/dnd/draggable-areas/transform.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/draggable-areas/transform.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPe html>
<meta charset='utf-8'>
<title>drag and drop – draggable area boundaries – transformed elements</title>
<style>
a {
  display: block;
  height: 200px;
  width: 200px;
  background-color: blue;
  margin-left: 100px;
  -moz-transform: rotate(-45deg) skew(15deg, 15deg);
  -o-transform: rotate(-45deg) skew(15deg, 15deg);
  -webkit-transform: rotate(-45deg) skew(15deg, 15deg);
  transform: rotate(-45deg) skew(15deg, 15deg);
}
</style>
<ol>
 <li>Try dragging the blue box below by clicking and holding <em>just</em>
 outside its skewed edges. It should <em>not</em> be draggable.</li>
 <li><p>Drag the blue box below. It should be draggable.</p>

<a draggable="true" ondragstart="event.dataTransfer.effectAllowed ='copy'">TEST</a>
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
  "source_name": "html/editing/dnd/draggable-areas/transform.html"
}
```
