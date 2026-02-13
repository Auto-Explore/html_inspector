# html/editing/dnd/platform/drag-keypress-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/platform/drag-keypress-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>drag &amp; drop - pressing tab while dragging</title>
<style>
  body > div {
    height: 200px;
    width: 200px;
    background-color: orange;
  }
  body {
    height: 5000px;
  }
  p {
    margin-top: 1000px;
  }
</style>

<script>
window.onload = function() {
  window.scrollBy(0,1000);
  document.getElementsByTagName('div')[0].ondragstart = function(e) {
    e.dataTransfer.effectAllowed = 'copy';
    e.dataTransfer.setData('text', 'dummy text');
  };
};
</script>
<noscript>Enable JavaScript and reload</noscript>
<p>Drag the orange square. While still dragging, press the Tab key on your keyboard. Fail if the page scrolls.</p>
<div draggable="true"></div>
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
  "source_name": "html/editing/dnd/platform/drag-keypress-manual.html"
}
```
