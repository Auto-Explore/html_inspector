# html/editing/dnd/overlay/heavy-styling-009.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/overlay/heavy-styling-009.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPe html>
<meta charset='utf-8'>
<title>drag and drop – feedback overlay for heavily styled elements – 009</title>
<style>
a {
  display: block;
  height: 200px;
  width: 200px;
  background-color: blue;
  border-radius: 50px;
}
</style>

<p>Drag the blue box below downwards. The drag placeholder should resemble the blue box, including the rounded corners.</p>

<a draggable="true" ondragstart="event.dataTransfer.effectAllowed ='copy'"></a>
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
  "source_name": "html/editing/dnd/overlay/heavy-styling-009.html"
}
```
