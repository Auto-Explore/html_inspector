# html/editing/dnd/overlay/060-manual.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/overlay/060-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>drag &amp; drop - dragging elements by children that are positioned outside them</title>
<style type="text/css">
div {
  height: 200px;
  width: 200px;
  background: blue;
  white-space: nowrap;
  position: relative;
}
span {
  display: block;
  top: 0;
  left: 210px;
  position: absolute;
  width: 100px;
  height: 100px;
  background: orange;
}
</style>
<script type="text/javascript">
window.onload = function () {
  document.getElementsByTagName('div')[0].ondragstart = function (e) {
    e.dataTransfer.effectAllowed = 'copy';
    e.dataTransfer.setData('text','dummy text');
  };
};
</script>
<div draggable="true"><span></span></div>
<p>Drag the orange square sideways. Pass if the drag placeholder shows that both the blue and orange squares are being dragged.</p>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.type.unnecessary",
      "message": "The “type” attribute for the “style” element is not needed and should be omitted.",
      "severity": "Warning",
      "span": {
        "byte_end": 135,
        "byte_start": 112,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 411,
        "byte_start": 380,
        "col": 1,
        "line": 21
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/editing/dnd/overlay/060-manual.html"
}
```
