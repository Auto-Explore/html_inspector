# html/editing/dnd/platform/close-drag-004-manual.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/platform/close-drag-004-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>drag &amp; drop - manually closing a tab while a drag is in operation</title>
<style type="text/css">
p.gone, ul { display: none; }
p.gone + ul { display: block; }
</style>
<script type="text/javascript">
window.onload = function() {
  if( location.href.match(/#popup$/) ) { document.getElementsByTagName('p')[0].className = 'gone'; }
  document.getElementsByTagName('ul')[0].ondragstart = function(e) {
    e.dataTransfer.effectAllowed = 'copy';
    e.dataTransfer.setData('Text', 'dummy text');
  };
};
</script>
<p><a href="#popup" target="_blank">Open this page in a new tab</a>.</p>
<ul draggable='true'>
  <li>Drag this text downwards, and do not release the drag.</li>
  <li>Use a keyboard shortcut (eg. Ctrl+W on Windows) to close the tab.</li>
  <li>The browser may optionally cancel the drag. The browser may optionally close the tab. Fail if the drag placeholder gets stuck. Fail if the browser crashes.</li>
</ul>
<noscript><p>Enable JavaScript and reload</p></noscript>
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
        "byte_end": 124,
        "byte_start": 101,
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
        "byte_end": 227,
        "byte_start": 196,
        "col": 1,
        "line": 7
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
  "source_name": "html/editing/dnd/platform/close-drag-004-manual.html"
}
```
