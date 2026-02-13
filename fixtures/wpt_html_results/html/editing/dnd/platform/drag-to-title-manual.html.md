# html/editing/dnd/platform/drag-to-title-manual.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/platform/drag-to-title-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Dropping onto the title bar and UI</title>
<script type="text/javascript">
window.onload = function () {
  document.getElementsByTagName('ul')[0].ondragstart = function () {
    e.dataTransfer.effectAllowed = 'copy';
    e.dataTransfer.setData('text','c');
  };
};
</script>
<ul draggable="true">
  <li>Drag this text upwards to the browser window's title bar.</li>
  <li>Release the drag. Fail if the drag placeholder does not disappear.</li>
  <li>Start dragging again over a blank part of the page (below the text). Fail if the placeholder starts following the mouse again.</li>
  <li>Release the drag over the browser's UI (e.g. the address bar). Fail if the browser crashes.</li>
</ul>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 97,
        "byte_start": 66,
        "col": 1,
        "line": 3
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
  "source_name": "html/editing/dnd/platform/drag-to-title-manual.html"
}
```
