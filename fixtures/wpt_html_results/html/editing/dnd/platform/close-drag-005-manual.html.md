# html/editing/dnd/platform/close-drag-005-manual.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/platform/close-drag-005-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>drag &amp; drop - manually closing a window while a drag is in operation</title>
<script type="text/javascript">
window.onload = function() {
  document.getElementsByTagName('ul')[0].ondragstart = function(e) {
    e.dataTransfer.effectAllowed = 'copy';
    e.dataTransfer.setData('Text', 'dummy text');
  };
};
</script>
<ul draggable='true'>
  <li>Drag this text downwards, and do not release the drag.</li>
  <li>Use a keyboard shortcut (eg. Alt+F4 on Windows) to close the window.</li>
  <li>The browser may optionally cancel the drag. The browser may optionally close the window. Fail if the drag placeholder gets stuck. Fail if the browser crashes.</li>
</ul>
<noscript><p>Enable JavaScript and reload</p></noscript>
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
        "byte_end": 135,
        "byte_start": 104,
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
  "source_name": "html/editing/dnd/platform/close-drag-005-manual.html"
}
```
